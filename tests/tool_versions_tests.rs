// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use tempfile::TempDir;
use tool_versions::{ToolEntry, ToolVersions};

#[test]
fn parse_empty() {
    let tv = ToolVersions::parse("").unwrap();
    assert!(tv.is_empty());
}

#[test]
fn parse_single_tool() {
    let tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn parse_multiple_tools() {
    let content = "erlang 26.2.1\nnodejs 20.10.0\nrust 1.75.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
    assert_eq!(tv.get_version("rust"), Some("1.75.0"));
}

#[test]
fn parse_with_comments() {
    let content = "# Development tools\nerlang 26.2.1\n# Node.js\nnodejs 20.10.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn parse_with_inline_comment() {
    let content = "erlang 26.2.1 # OTP version\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn parse_with_empty_lines() {
    let content = "\nerlang 26.2.1\n\nnodejs 20.10.0\n\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn parse_multiple_versions() {
    let content = "python 3.11.0 3.10.0 system\n";
    let tv = ToolVersions::parse(content).unwrap();
    let entry = tv.get("python").unwrap();
    assert_eq!(entry.versions, vec!["3.11.0", "3.10.0", "system"]);
    assert_eq!(entry.primary_version(), Some("3.11.0"));
}

#[test]
fn parse_ref_version() {
    let content = "elixir ref:v1.15.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("elixir"), Some("ref:v1.15.0"));
}

#[test]
fn parse_path_version() {
    let content = "elixir path:~/src/elixir\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("elixir"), Some("path:~/src/elixir"));
}

#[test]
fn parse_system_version() {
    let content = "ruby system\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("ruby"), Some("system"));
}

#[test]
fn parse_tool_with_hyphen() {
    let content = "java-openjdk 21.0.1\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("java-openjdk"), Some("21.0.1"));
}

#[test]
fn parse_tool_with_underscore() {
    let content = "my_tool 1.0.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("my_tool"), Some("1.0.0"));
}

#[test]
fn get_nonexistent_tool() {
    let tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    assert_eq!(tv.get("nonexistent"), None);
    assert_eq!(tv.get_version("nonexistent"), None);
}

#[test]
fn set_new_tool() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn set_update_existing_tool() {
    let mut tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    tv.set("erlang", "27.0.0");
    assert_eq!(tv.get_version("erlang"), Some("27.0.0"));
}

#[test]
fn set_versions_multiple() {
    let mut tv = ToolVersions::new();
    tv.set_versions("python", vec!["3.12.0".to_string(), "3.11.0".to_string()]);
    let entry = tv.get("python").unwrap();
    assert_eq!(entry.versions, vec!["3.12.0", "3.11.0"]);
}

#[test]
fn remove_tool() {
    let mut tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    assert!(tv.remove("erlang"));
    assert_eq!(tv.get("erlang"), None);
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn remove_nonexistent_tool() {
    let mut tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    assert!(!tv.remove("nonexistent"));
}

#[test]
fn contains() {
    let tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    assert!(tv.contains("erlang"));
    assert!(!tv.contains("nodejs"));
}

#[test]
fn tools_iterator() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let tools: Vec<_> = tv.tools().collect();
    assert_eq!(tools.len(), 2);
}

#[test]
fn tool_names_iterator() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let names: Vec<_> = tv.tool_names().collect();
    assert!(names.contains(&"erlang"));
    assert!(names.contains(&"nodejs"));
}

#[test]
fn to_string_single_tool() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    assert_eq!(tv.to_string(), "erlang 26.2.1\n");
}

#[test]
fn to_string_preserves_comments() {
    let content = "# Comment\nerlang 26.2.1\n";
    let tv = ToolVersions::parse(content).unwrap();
    let output = tv.to_string();
    assert!(output.contains("# Comment"));
    assert!(output.contains("erlang 26.2.1"));
}

#[test]
fn to_string_preserves_empty_lines() {
    let content = "erlang 26.2.1\n\nnodejs 20.10.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    let output = tv.to_string();
    assert!(output.contains("\n\n"));
}

#[test]
fn to_string_multiple_versions() {
    let mut tv = ToolVersions::new();
    tv.set_versions("python", vec!["3.12.0".to_string(), "3.11.0".to_string()]);
    assert_eq!(tv.to_string(), "python 3.12.0 3.11.0\n");
}

#[test]
fn save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join(".tool-versions");

    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    tv.set("nodejs", "20.10.0");
    tv.save(&path).unwrap();

    let loaded = ToolVersions::load(&path).unwrap();
    assert_eq!(loaded.get_version("erlang"), Some("26.2.1"));
    assert_eq!(loaded.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn load_nonexistent_file() {
    let result = ToolVersions::load("/nonexistent/path/.tool-versions");
    assert!(result.is_err());
}

#[test]
fn load_or_default_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join(".tool-versions");

    let tv = ToolVersions::load_or_default(&path).unwrap();
    assert!(tv.is_empty());
}

#[test]
fn load_or_default_existing() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join(".tool-versions");
    fs::write(&path, "erlang 26.2.1\n").unwrap();

    let tv = ToolVersions::load_or_default(&path).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn roundtrip_preserves_data() {
    let content = "# Tools\nerlang 26.2.1\n\nnodejs 20.10.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    let output = tv.to_string();
    let reparsed = ToolVersions::parse(&output).unwrap();

    assert_eq!(reparsed.get_version("erlang"), Some("26.2.1"));
    assert_eq!(reparsed.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn parse_error_no_version() {
    let result = ToolVersions::parse("erlang\n");
    assert!(result.is_err());
}

#[test]
fn parse_error_invalid_tool_name() {
    let result = ToolVersions::parse("invalid/tool 1.0.0\n");
    assert!(result.is_err());
}

#[test]
fn parse_whitespace_variations() {
    let content = "  erlang   26.2.1  \n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn parse_tabs() {
    let content = "erlang\t26.2.1\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn tool_entry_display() {
    let entry = ToolEntry::new("erlang", "26.2.1");
    assert_eq!(entry.to_string(), "erlang 26.2.1");
}

#[test]
fn tool_entry_with_versions_display() {
    let entry =
        ToolEntry::with_versions("python", vec!["3.12.0".to_string(), "3.11.0".to_string()]);
    assert_eq!(entry.to_string(), "python 3.12.0 3.11.0");
}

#[test]
fn set_preserves_order() {
    let content = "erlang 26.2.1\nnodejs 20.10.0\nrust 1.75.0\n";
    let mut tv = ToolVersions::parse(content).unwrap();
    tv.set("nodejs", "21.0.0");

    let output = tv.to_string();
    let lines: Vec<_> = output.lines().collect();

    assert!(lines[0].starts_with("erlang"));
    assert!(lines[1].starts_with("nodejs"));
    assert!(lines[2].starts_with("rust"));
}

#[test]
fn set_appends_new_tool() {
    let content = "erlang 26.2.1\n";
    let mut tv = ToolVersions::parse(content).unwrap();
    tv.set("nodejs", "20.10.0");

    let output = tv.to_string();
    assert!(output.contains("erlang 26.2.1"));
    assert!(output.contains("nodejs 20.10.0"));
}

#[test]
fn parse_duplicate_tools_keeps_first() {
    let content = "erlang 26.2.1\nnodejs 20.10.0\nerlang 25.0.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.len(), 2);
}

#[test]
fn parse_tool_with_dot_in_name() {
    let content = "java.openjdk 21.0.1\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("java.openjdk"), Some("21.0.1"));
}

#[test]
fn parse_comment_only_line() {
    let content = "#\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert!(tv.is_empty());
}

#[test]
fn parse_comment_with_tool_like_content() {
    let content = "# erlang 26.2.1\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert!(tv.get("erlang").is_none());
}

#[test]
fn remove_preserves_comments() {
    let content = "# Tools\nerlang 26.2.1\n# Node\nnodejs 20.10.0\n";
    let mut tv = ToolVersions::parse(content).unwrap();
    tv.remove("erlang");

    let output = tv.to_string();
    assert!(output.contains("# Tools"));
    assert!(output.contains("# Node"));
    assert!(!output.contains("erlang"));
}

#[test]
fn set_on_duplicate_updates_first_occurrence() {
    let content = "erlang 26.2.1\nnodejs 20.10.0\nerlang 25.0.0\n";
    let mut tv = ToolVersions::parse(content).unwrap();
    tv.set("erlang", "27.0.0");
    assert_eq!(tv.get_version("erlang"), Some("27.0.0"));
}

#[test]
fn primary_version_empty() {
    let entry = ToolEntry::with_versions("test", vec![]);
    assert_eq!(entry.primary_version(), None);
}

#[test]
fn parse_error_display() {
    let result = ToolVersions::parse("erlang\n");
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("parse error"));
    assert!(msg.contains("line 1"));
}

#[test]
fn io_error_display() {
    let result = ToolVersions::load("/nonexistent/path/.tool-versions");
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("I/O error"));
}

#[test]
fn tool_entry_equality() {
    let a = ToolEntry::new("erlang", "26.2.1");
    let b = ToolEntry::new("erlang", "26.2.1");
    let c = ToolEntry::new("erlang", "27.0.0");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn tool_versions_equality() {
    let mut a = ToolVersions::new();
    a.set("erlang", "26.2.1");
    let mut b = ToolVersions::new();
    b.set("erlang", "26.2.1");
    assert_eq!(a.get("erlang"), b.get("erlang"));
}

#[test]
fn tool_versions_default() {
    let tv = ToolVersions::default();
    assert!(tv.is_empty());
}

#[test]
fn tool_entry_clone() {
    let entry = ToolEntry::new("erlang", "26.2.1");
    let cloned = entry.clone();
    assert_eq!(entry, cloned);
}

#[test]
fn tool_versions_clone() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    let cloned = tv.clone();
    assert_eq!(cloned.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn len_empty() {
    let tv = ToolVersions::new();
    assert_eq!(tv.len(), 0);
}

#[test]
fn len_with_tools() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    tv.set("nodejs", "20.10.0");
    assert_eq!(tv.len(), 2);
}

#[test]
fn is_empty_true() {
    let tv = ToolVersions::new();
    assert!(tv.is_empty());
}

#[test]
fn is_empty_false() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    assert!(!tv.is_empty());
}

#[test]
fn parse_crlf_line_endings() {
    let content = "erlang 26.2.1\r\nnodejs 20.10.0\r\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn parse_mixed_line_endings() {
    let content = "erlang 26.2.1\nnodejs 20.10.0\r\nrust 1.75.0\n";
    let tv = ToolVersions::parse(content).unwrap();
    assert_eq!(tv.len(), 3);
}

#[test]
fn parse_error_unicode_tool_name() {
    // Japanese
    let result = ToolVersions::parse("工具 1.0.0\n");
    assert!(result.is_err());

    // Korean
    let result = ToolVersions::parse("도구 1.0.0\n");
    assert!(result.is_err());
}

#[test]
fn parse_error_emoji_tool_name() {
    let result = ToolVersions::parse("🦀rust 1.75.0\n");
    assert!(result.is_err());
}
