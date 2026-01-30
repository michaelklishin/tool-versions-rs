// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "serde")]

use tool_versions::{ToolEntry, ToolVersions};

#[test]
fn tool_entry_serialize() {
    let entry = ToolEntry::new("erlang", "26.2.1");
    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("erlang"));
    assert!(json.contains("26.2.1"));
}

#[test]
fn tool_entry_deserialize() {
    let json = r#"{"name":"erlang","versions":["26.2.1"]}"#;
    let entry: ToolEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.name, "erlang");
    assert_eq!(entry.versions, vec!["26.2.1"]);
}

#[test]
fn tool_entry_roundtrip() {
    let entry =
        ToolEntry::with_versions("python", vec!["3.12.0".to_string(), "3.11.0".to_string()]);
    let json = serde_json::to_string(&entry).unwrap();
    let deserialized: ToolEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, entry);
}

#[test]
fn tool_versions_serialize() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    tv.set("nodejs", "20.10.0");
    let json = serde_json::to_string(&tv).unwrap();
    assert!(json.contains("erlang"));
    assert!(json.contains("nodejs"));
}

#[test]
fn tool_versions_deserialize() {
    let json = r#"{"tools":[{"name":"erlang","versions":["26.2.1"]}]}"#;
    let tv: ToolVersions = serde_json::from_str(json).unwrap();
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn tool_versions_roundtrip() {
    let mut tv = ToolVersions::new();
    tv.set("erlang", "26.2.1");
    tv.set_versions("python", vec!["3.12.0".to_string(), "3.11.0".to_string()]);
    let json = serde_json::to_string(&tv).unwrap();
    let deserialized: ToolVersions = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.get_version("erlang"), tv.get_version("erlang"));
    assert_eq!(
        deserialized.get("python").unwrap().versions,
        tv.get("python").unwrap().versions
    );
}

#[test]
fn tool_versions_deserialize_to_string() {
    let json = r#"{"tools":[{"name":"erlang","versions":["26.2.1"]},{"name":"nodejs","versions":["20.10.0"]}]}"#;
    let tv: ToolVersions = serde_json::from_str(json).unwrap();
    let output = tv.to_string();
    assert!(output.contains("erlang 26.2.1"));
    assert!(output.contains("nodejs 20.10.0"));
}
