// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::collections::HashMap;

use tool_versions::ToolVersions;

//
// HashMap conversions (owned)
//

#[test]
fn to_hashmap_with_all_versions() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: HashMap<String, Vec<String>> = tv.into();

    assert_eq!(map.get("erlang"), Some(&vec!["26.2.1".to_string()]));
    assert_eq!(
        map.get("python"),
        Some(&vec!["3.12.0".to_string(), "3.11.0".to_string()])
    );
}

#[test]
fn to_hashmap_with_primary_version() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: HashMap<String, String> = tv.into();

    assert_eq!(map.get("erlang"), Some(&"26.2.1".to_string()));
    assert_eq!(map.get("python"), Some(&"3.12.0".to_string()));
}

#[test]
fn from_hashmap_with_all_versions() {
    let mut map = HashMap::new();
    map.insert("erlang".to_string(), vec!["26.2.1".to_string()]);
    map.insert(
        "python".to_string(),
        vec!["3.12.0".to_string(), "3.11.0".to_string()],
    );

    let tv: ToolVersions = map.into();

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("python"), Some("3.12.0"));
    let python = tv.get("python").unwrap();
    assert_eq!(python.versions.len(), 2);
}

#[test]
fn from_hashmap_with_single_version() {
    let mut map = HashMap::new();
    map.insert("erlang".to_string(), "26.2.1".to_string());
    map.insert("nodejs".to_string(), "20.10.0".to_string());

    let tv: ToolVersions = map.into();

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

//
// HashMap conversions (borrowed)
//

#[test]
fn to_hashmap_borrowed_with_all_versions() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: HashMap<String, Vec<String>> = (&tv).into();

    assert_eq!(map.get("erlang"), Some(&vec!["26.2.1".to_string()]));
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn to_hashmap_borrowed_with_primary_version() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: HashMap<String, String> = (&tv).into();

    assert_eq!(map.get("erlang"), Some(&"26.2.1".to_string()));
    assert_eq!(map.get("python"), Some(&"3.12.0".to_string()));
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

//
// BTreeMap conversions (owned)
//

#[test]
fn to_btreemap_with_all_versions() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: BTreeMap<String, Vec<String>> = tv.into();

    assert_eq!(map.get("erlang"), Some(&vec!["26.2.1".to_string()]));
    assert_eq!(
        map.get("python"),
        Some(&vec!["3.12.0".to_string(), "3.11.0".to_string()])
    );
}

#[test]
fn to_btreemap_with_primary_version() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: BTreeMap<String, String> = tv.into();

    assert_eq!(map.get("erlang"), Some(&"26.2.1".to_string()));
    assert_eq!(map.get("python"), Some(&"3.12.0".to_string()));
}

#[test]
fn btreemap_is_sorted() {
    let tv = ToolVersions::parse("python 3.12.0\nerlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let map: BTreeMap<String, String> = tv.into();

    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec!["erlang", "nodejs", "python"]);
}

#[test]
fn from_btreemap_with_all_versions() {
    let mut map = BTreeMap::new();
    map.insert("erlang".to_string(), vec!["26.2.1".to_string()]);
    map.insert(
        "python".to_string(),
        vec!["3.12.0".to_string(), "3.11.0".to_string()],
    );

    let tv: ToolVersions = map.into();

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("python"), Some("3.12.0"));
}

#[test]
fn from_btreemap_with_single_version() {
    let mut map = BTreeMap::new();
    map.insert("erlang".to_string(), "26.2.1".to_string());
    map.insert("nodejs".to_string(), "20.10.0".to_string());

    let tv: ToolVersions = map.into();

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

//
// BTreeMap conversions (borrowed)
//

#[test]
fn to_btreemap_borrowed_with_all_versions() {
    let tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    let map: BTreeMap<String, Vec<String>> = (&tv).into();

    assert_eq!(map.get("erlang"), Some(&vec!["26.2.1".to_string()]));
    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
}

#[test]
fn to_btreemap_borrowed_with_primary_version() {
    let tv = ToolVersions::parse("erlang 26.2.1\npython 3.12.0 3.11.0\n").unwrap();
    let map: BTreeMap<String, String> = (&tv).into();

    assert_eq!(map.get("python"), Some(&"3.12.0".to_string()));
    assert_eq!(tv.get_version("python"), Some("3.12.0"));
}

//
// IntoIterator
//

#[test]
fn into_iter_owned() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let pairs: Vec<(String, Vec<String>)> = tv.into_iter().collect();

    assert_eq!(pairs.len(), 2);
    assert!(pairs.iter().any(|(k, _)| k == "erlang"));
    assert!(pairs.iter().any(|(k, _)| k == "nodejs"));
}

#[test]
fn into_iter_borrowed() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let pairs: Vec<(&str, &[String])> = (&tv).into_iter().collect();

    assert_eq!(pairs.len(), 2);
    assert!(pairs.iter().any(|(k, _)| *k == "erlang"));
    assert_eq!(tv.len(), 2);
}

#[test]
fn for_loop_owned() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let mut count = 0;
    for (name, versions) in tv {
        assert!(!name.is_empty());
        assert!(!versions.is_empty());
        count += 1;
    }
    assert_eq!(count, 2);
}

#[test]
fn for_loop_borrowed() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let mut count = 0;
    for (name, versions) in &tv {
        assert!(!name.is_empty());
        assert!(!versions.is_empty());
        count += 1;
    }
    assert_eq!(count, 2);
    assert_eq!(tv.len(), 2);
}

#[test]
fn into_iter_exact_size() {
    let tv = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\nrust 1.75.0\n").unwrap();
    let iter = tv.into_iter();

    assert_eq!(iter.len(), 3);
}

//
// FromIterator
//

#[test]
fn collect_from_tuples_with_vec_versions() {
    let pairs = vec![
        ("erlang".to_string(), vec!["26.2.1".to_string()]),
        (
            "python".to_string(),
            vec!["3.12.0".to_string(), "3.11.0".to_string()],
        ),
    ];

    let tv: ToolVersions = pairs.into_iter().collect();

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("python"), Some("3.12.0"));
    assert_eq!(tv.get("python").unwrap().versions.len(), 2);
}

#[test]
fn collect_from_tuples_with_single_version() {
    let pairs = vec![
        ("erlang".to_string(), "26.2.1".to_string()),
        ("nodejs".to_string(), "20.10.0".to_string()),
    ];

    let tv: ToolVersions = pairs.into_iter().collect();

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn collect_empty() {
    let pairs: Vec<(String, String)> = vec![];
    let tv: ToolVersions = pairs.into_iter().collect();
    assert!(tv.is_empty());
}

#[test]
fn collect_with_duplicate_names_keeps_last() {
    let pairs = vec![
        ("erlang".to_string(), "26.0.0".to_string()),
        ("nodejs".to_string(), "20.0.0".to_string()),
        ("erlang".to_string(), "27.0.0".to_string()),
    ];

    let tv: ToolVersions = pairs.into_iter().collect();

    assert_eq!(tv.len(), 2);
    assert_eq!(tv.get_version("erlang"), Some("27.0.0"));
    assert_eq!(tv.get_version("nodejs"), Some("20.0.0"));
}

//
// Extend
//

#[test]
fn extend_with_vec_versions() {
    let mut tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    tv.extend(vec![("nodejs".to_string(), vec!["20.10.0".to_string()])]);

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn extend_with_single_version() {
    let mut tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    tv.extend(vec![("nodejs".to_string(), "20.10.0".to_string())]);

    assert_eq!(tv.get_version("erlang"), Some("26.2.1"));
    assert_eq!(tv.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn extend_overwrites_existing() {
    let mut tv = ToolVersions::parse("erlang 26.2.1\n").unwrap();
    tv.extend(vec![("erlang".to_string(), "27.0.0".to_string())]);

    assert_eq!(tv.get_version("erlang"), Some("27.0.0"));
    assert_eq!(tv.len(), 1);
}

#[test]
fn extend_from_hashmap() {
    let mut tv = ToolVersions::new();
    let mut map = HashMap::new();
    map.insert("erlang".to_string(), "26.2.1".to_string());
    map.insert("nodejs".to_string(), "20.10.0".to_string());

    tv.extend(map);

    assert_eq!(tv.len(), 2);
    assert!(tv.contains("erlang"));
    assert!(tv.contains("nodejs"));
}

//
// Edge cases
//

#[test]
fn empty_toolversions_to_hashmap() {
    let tv = ToolVersions::new();
    let map: HashMap<String, Vec<String>> = tv.into();
    assert!(map.is_empty());
}

#[test]
fn empty_hashmap_to_toolversions() {
    let map: HashMap<String, String> = HashMap::new();
    let tv: ToolVersions = map.into();
    assert!(tv.is_empty());
}

#[test]
fn roundtrip_through_hashmap() {
    let original = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let map: HashMap<String, Vec<String>> = (&original).into();
    let restored: ToolVersions = map.into();

    assert_eq!(restored.get_version("erlang"), Some("26.2.1"));
    assert_eq!(restored.get_version("nodejs"), Some("20.10.0"));
}

#[test]
fn roundtrip_through_btreemap() {
    let original = ToolVersions::parse("erlang 26.2.1\nnodejs 20.10.0\n").unwrap();
    let map: BTreeMap<String, Vec<String>> = (&original).into();
    let restored: ToolVersions = map.into();

    assert_eq!(restored.get_version("erlang"), Some("26.2.1"));
    assert_eq!(restored.get_version("nodejs"), Some("20.10.0"));
}
