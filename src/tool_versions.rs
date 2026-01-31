// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::vec;

use crate::Result;
use crate::errors::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolEntry {
    pub name: String,
    pub versions: Vec<String>,
}

impl ToolEntry {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            versions: vec![version.into()],
        }
    }

    pub fn with_versions(name: impl Into<String>, versions: Vec<String>) -> Self {
        Self {
            name: name.into(),
            versions,
        }
    }

    pub fn primary_version(&self) -> Option<&str> {
        self.versions.first().map(String::as_str)
    }
}

impl fmt::Display for ToolEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.versions.join(" "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Tool(ToolEntry),
    Comment(String),
    Empty,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolVersions {
    #[cfg_attr(feature = "serde", serde(skip))]
    lines: Vec<Line>,
    tools: Vec<ToolEntry>,
}

impl ToolVersions {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            tools: Vec::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        match fs::read_to_string(path) {
            Ok(content) => Self::parse(&content),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Self::new()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn parse(content: &str) -> Result<Self> {
        let mut lines = Vec::new();
        let mut tools = Vec::new();
        let mut seen_tools = HashSet::new();

        for (line_num, line) in content.lines().enumerate() {
            let parsed = parse_line(line, line_num + 1)?;

            if let Line::Tool(entry) = &parsed
                && seen_tools.insert(entry.name.clone())
            {
                tools.push(entry.clone());
            }

            lines.push(parsed);
        }

        Ok(Self { lines, tools })
    }

    pub fn get(&self, tool_name: &str) -> Option<&ToolEntry> {
        self.tools.iter().find(|e| e.name == tool_name)
    }

    pub fn get_version(&self, tool_name: &str) -> Option<&str> {
        self.get(tool_name)?.primary_version()
    }

    pub fn set(&mut self, tool_name: &str, version: &str) {
        self.set_versions(tool_name, vec![version.to_string()]);
    }

    pub fn set_versions(&mut self, tool_name: &str, versions: Vec<String>) {
        let entry = ToolEntry::with_versions(tool_name, versions);

        if let Some(idx) = self.tools.iter().position(|e| e.name == tool_name) {
            self.tools[idx] = entry.clone();

            for line in &mut self.lines {
                if let Line::Tool(e) = line
                    && e.name == tool_name
                {
                    *line = Line::Tool(entry);
                    return;
                }
            }
        } else {
            self.tools.push(entry.clone());
            self.lines.push(Line::Tool(entry));
        }
    }

    pub fn remove(&mut self, tool_name: &str) -> bool {
        let removed = self.tools.iter().position(|e| e.name == tool_name);

        if let Some(idx) = removed {
            self.tools.remove(idx);

            self.lines
                .retain(|line| !matches!(line, Line::Tool(e) if e.name == tool_name));

            true
        } else {
            false
        }
    }

    pub fn contains(&self, tool_name: &str) -> bool {
        self.tools.iter().any(|e| e.name == tool_name)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn tools(&self) -> impl Iterator<Item = &ToolEntry> {
        self.tools.iter()
    }

    pub fn tool_names(&self) -> impl Iterator<Item = &str> {
        self.tools.iter().map(|e| e.name.as_str())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::write(path, self.to_string())?;
        Ok(())
    }
}

impl fmt::Display for ToolVersions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lines.is_empty() {
            for tool in &self.tools {
                writeln!(f, "{}", tool)?;
            }
        } else {
            for line in &self.lines {
                match line {
                    Line::Tool(entry) => writeln!(f, "{}", entry)?,
                    Line::Comment(text) => writeln!(f, "{}", text)?,
                    Line::Empty => writeln!(f)?,
                }
            }
        }
        Ok(())
    }
}

//
// Conversions to HashMap
//

impl From<ToolVersions> for HashMap<String, Vec<String>> {
    fn from(tv: ToolVersions) -> Self {
        tv.tools.into_iter().map(|e| (e.name, e.versions)).collect()
    }
}

impl From<ToolVersions> for HashMap<String, String> {
    fn from(tv: ToolVersions) -> Self {
        tv.tools
            .into_iter()
            .filter_map(|e| e.versions.into_iter().next().map(|v| (e.name, v)))
            .collect()
    }
}

impl From<&ToolVersions> for HashMap<String, Vec<String>> {
    fn from(tv: &ToolVersions) -> Self {
        tv.tools
            .iter()
            .map(|e| (e.name.clone(), e.versions.clone()))
            .collect()
    }
}

impl From<&ToolVersions> for HashMap<String, String> {
    fn from(tv: &ToolVersions) -> Self {
        tv.tools
            .iter()
            .filter_map(|e| e.versions.first().map(|v| (e.name.clone(), v.clone())))
            .collect()
    }
}

//
// Conversions from HashMap
//

impl From<HashMap<String, Vec<String>>> for ToolVersions {
    fn from(map: HashMap<String, Vec<String>>) -> Self {
        let tools: Vec<ToolEntry> = map
            .into_iter()
            .map(|(name, versions)| ToolEntry::with_versions(name, versions))
            .collect();
        Self {
            lines: Vec::new(),
            tools,
        }
    }
}

impl From<HashMap<String, String>> for ToolVersions {
    fn from(map: HashMap<String, String>) -> Self {
        let tools: Vec<ToolEntry> = map
            .into_iter()
            .map(|(name, version)| ToolEntry::new(name, version))
            .collect();
        Self {
            lines: Vec::new(),
            tools,
        }
    }
}

//
// Conversions to BTreeMap
//

impl From<ToolVersions> for BTreeMap<String, Vec<String>> {
    fn from(tv: ToolVersions) -> Self {
        tv.tools.into_iter().map(|e| (e.name, e.versions)).collect()
    }
}

impl From<ToolVersions> for BTreeMap<String, String> {
    fn from(tv: ToolVersions) -> Self {
        tv.tools
            .into_iter()
            .filter_map(|e| e.versions.into_iter().next().map(|v| (e.name, v)))
            .collect()
    }
}

impl From<&ToolVersions> for BTreeMap<String, Vec<String>> {
    fn from(tv: &ToolVersions) -> Self {
        tv.tools
            .iter()
            .map(|e| (e.name.clone(), e.versions.clone()))
            .collect()
    }
}

impl From<&ToolVersions> for BTreeMap<String, String> {
    fn from(tv: &ToolVersions) -> Self {
        tv.tools
            .iter()
            .filter_map(|e| e.versions.first().map(|v| (e.name.clone(), v.clone())))
            .collect()
    }
}

//
// Conversions from BTreeMap
//

impl From<BTreeMap<String, Vec<String>>> for ToolVersions {
    fn from(map: BTreeMap<String, Vec<String>>) -> Self {
        let tools: Vec<ToolEntry> = map
            .into_iter()
            .map(|(name, versions)| ToolEntry::with_versions(name, versions))
            .collect();
        Self {
            lines: Vec::new(),
            tools,
        }
    }
}

impl From<BTreeMap<String, String>> for ToolVersions {
    fn from(map: BTreeMap<String, String>) -> Self {
        let tools: Vec<ToolEntry> = map
            .into_iter()
            .map(|(name, version)| ToolEntry::new(name, version))
            .collect();
        Self {
            lines: Vec::new(),
            tools,
        }
    }
}

//
// IntoIterator
//

pub struct IntoIter {
    inner: vec::IntoIter<ToolEntry>,
}

impl Iterator for IntoIter {
    type Item = (String, Vec<String>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|e| (e.name, e.versions))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for IntoIter {}

impl IntoIterator for ToolVersions {
    type Item = (String, Vec<String>);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.tools.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a ToolVersions {
    type Item = (&'a str, &'a [String]);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.tools
                .iter()
                .map(|e| (e.name.as_str(), e.versions.as_slice())),
        )
    }
}

//
// FromIterator
//

impl FromIterator<(String, Vec<String>)> for ToolVersions {
    fn from_iter<I: IntoIterator<Item = (String, Vec<String>)>>(iter: I) -> Self {
        let mut tv = Self::new();
        tv.extend(iter);
        tv
    }
}

impl FromIterator<(String, String)> for ToolVersions {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let mut tv = Self::new();
        tv.extend(iter);
        tv
    }
}

//
// Extend
//

impl Extend<(String, Vec<String>)> for ToolVersions {
    fn extend<I: IntoIterator<Item = (String, Vec<String>)>>(&mut self, iter: I) {
        for (name, versions) in iter {
            self.set_versions(&name, versions);
        }
    }
}

impl Extend<(String, String)> for ToolVersions {
    fn extend<I: IntoIterator<Item = (String, String)>>(&mut self, iter: I) {
        for (name, version) in iter {
            self.set(&name, &version);
        }
    }
}

fn parse_line(line: &str, line_num: usize) -> Result<Line> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return Ok(Line::Empty);
    }

    if trimmed.starts_with('#') {
        return Ok(Line::Comment(line.to_string()));
    }

    let content = if let Some(comment_pos) = trimmed.find('#') {
        trimmed[..comment_pos].trim()
    } else {
        trimmed
    };

    let mut parts = content.split_whitespace();

    let tool_name = parts.next().ok_or_else(|| Error::ParseError {
        line: line_num,
        message: "empty tool name".to_string(),
    })?;

    if !is_valid_tool_name(tool_name) {
        return Err(Error::ParseError {
            line: line_num,
            message: format!("invalid tool name: {}", tool_name),
        });
    }

    let versions: Vec<String> = parts.map(|s| s.to_string()).collect();

    if versions.is_empty() {
        return Err(Error::ParseError {
            line: line_num,
            message: format!("no version specified for tool: {}", tool_name),
        });
    }

    Ok(Line::Tool(ToolEntry::with_versions(tool_name, versions)))
}

fn is_valid_tool_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}
