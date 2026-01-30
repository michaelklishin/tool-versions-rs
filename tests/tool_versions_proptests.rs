// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::*;
use tool_versions::ToolVersions;

fn tool_name_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("erlang".to_string()),
        Just("nodejs".to_string()),
        Just("python".to_string()),
        Just("rust".to_string()),
        Just("ruby".to_string()),
        Just("elixir".to_string()),
        Just("java".to_string()),
        Just("golang".to_string()),
        Just("java.openjdk".to_string()),
        "[a-z][a-z0-9_.-]{1,15}",
    ]
}

fn version_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (1u32..100, 0u32..100, 0u32..100).prop_map(|(a, b, c)| format!("{}.{}.{}", a, b, c)),
        (1u32..100, 0u32..100).prop_map(|(a, b)| format!("{}.{}", a, b)),
        Just("system".to_string()),
        "[a-f0-9]{7}".prop_map(|s| format!("ref:{}", s)),
    ]
}

fn tool_line_strategy() -> impl Strategy<Value = String> {
    (tool_name_strategy(), version_strategy())
        .prop_map(|(name, version)| format!("{} {}\n", name, version))
}

fn comment_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("# Comment\n".to_string()),
        Just("# Development tools\n".to_string()),
        Just("#\n".to_string()),
    ]
}

fn multi_line_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            3 => tool_line_strategy(),
            1 => comment_strategy(),
            1 => Just("\n".to_string()),
        ],
        0..10,
    )
    .prop_map(|lines| lines.join(""))
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(300))]

    #[test]
    fn roundtrip_set_get(name in tool_name_strategy(), version in version_strategy()) {
        let mut tv = ToolVersions::new();
        tv.set(&name, &version);
        prop_assert_eq!(tv.get_version(&name), Some(version.as_str()));
    }

    #[test]
    fn roundtrip_parse_to_string(name in tool_name_strategy(), version in version_strategy()) {
        let content = format!("{} {}\n", name, version);
        let tv = ToolVersions::parse(&content).unwrap();
        let output = tv.to_string();
        let reparsed = ToolVersions::parse(&output).unwrap();
        prop_assert_eq!(reparsed.get_version(&name), Some(version.as_str()));
    }

    #[test]
    fn set_overwrite(name in tool_name_strategy(), version1 in version_strategy(), version2 in version_strategy()) {
        let mut tv = ToolVersions::new();
        tv.set(&name, &version1);
        tv.set(&name, &version2);
        prop_assert_eq!(tv.get_version(&name), Some(version2.as_str()));
    }

    #[test]
    fn remove_then_get(name in tool_name_strategy(), version in version_strategy()) {
        let mut tv = ToolVersions::new();
        tv.set(&name, &version);
        tv.remove(&name);
        prop_assert_eq!(tv.get(&name), None);
    }

    #[test]
    fn contains_after_set(name in tool_name_strategy(), version in version_strategy()) {
        let mut tv = ToolVersions::new();
        prop_assert!(!tv.contains(&name));
        tv.set(&name, &version);
        prop_assert!(tv.contains(&name));
    }

    #[test]
    fn parse_multiline(content in multi_line_strategy()) {
        let result = ToolVersions::parse(&content);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn multiline_roundtrip(content in multi_line_strategy()) {
        let tv = ToolVersions::parse(&content).unwrap();
        let output = tv.to_string();
        let reparsed = ToolVersions::parse(&output).unwrap();

        for entry in tv.tools() {
            prop_assert_eq!(reparsed.get_version(&entry.name), tv.get_version(&entry.name));
        }
    }

    #[test]
    fn tool_count_matches(names in prop::collection::hash_set(tool_name_strategy(), 1..10)) {
        let mut tv = ToolVersions::new();
        for name in &names {
            tv.set(name, "1.0.0");
        }
        prop_assert_eq!(tv.len(), names.len());
    }

    #[test]
    fn multiple_versions_preserved(
        name in tool_name_strategy(),
        versions in prop::collection::vec(version_strategy(), 1..5)
    ) {
        let mut tv = ToolVersions::new();
        tv.set_versions(&name, versions.clone());
        let entry = tv.get(&name).unwrap();
        prop_assert_eq!(&entry.versions, &versions);
    }
}
