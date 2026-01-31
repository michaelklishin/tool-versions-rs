// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::collections::HashMap;

use proptest::prelude::*;
use tool_versions::ToolVersions;

fn tool_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_.-]{0,15}"
}

fn version_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        (1u32..100, 0u32..100, 0u32..100).prop_map(|(a, b, c)| format!("{}.{}.{}", a, b, c)),
        Just("system".to_string()),
    ]
}

fn versions_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(version_strategy(), 1..4)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    //
    // HashMap roundtrips
    //

    #[test]
    fn hashmap_roundtrip_all_versions(
        entries in prop::collection::hash_map(tool_name_strategy(), versions_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into();
        let map: HashMap<String, Vec<String>> = tv.into();

        for (name, versions) in entries {
            prop_assert_eq!(map.get(&name), Some(&versions));
        }
    }

    #[test]
    fn hashmap_roundtrip_primary_version(
        entries in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into();
        let map: HashMap<String, String> = tv.into();

        for (name, version) in entries {
            prop_assert_eq!(map.get(&name), Some(&version));
        }
    }

    #[test]
    fn hashmap_borrowed_preserves_original(
        entries in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into();
        let _map: HashMap<String, String> = (&tv).into();

        for (name, version) in entries {
            prop_assert_eq!(tv.get_version(&name), Some(version.as_str()));
        }
    }

    //
    // BTreeMap roundtrips
    //

    #[test]
    fn btreemap_roundtrip_all_versions(
        entries in prop::collection::btree_map(tool_name_strategy(), versions_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into();
        let map: BTreeMap<String, Vec<String>> = tv.into();

        for (name, versions) in entries {
            prop_assert_eq!(map.get(&name), Some(&versions));
        }
    }

    #[test]
    fn btreemap_roundtrip_primary_version(
        entries in prop::collection::btree_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into();
        let map: BTreeMap<String, String> = tv.into();

        for (name, version) in entries {
            prop_assert_eq!(map.get(&name), Some(&version));
        }
    }

    #[test]
    fn btreemap_is_always_sorted(
        entries in prop::collection::btree_map(tool_name_strategy(), version_strategy(), 2..10)
    ) {
        let tv: ToolVersions = entries.into();
        let map: BTreeMap<String, String> = tv.into();

        let keys: Vec<_> = map.keys().collect();
        let mut sorted = keys.clone();
        sorted.sort();
        prop_assert_eq!(keys, sorted);
    }

    //
    // IntoIterator
    //

    #[test]
    fn into_iter_length_matches(
        entries in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let expected_len = entries.len();
        let tv: ToolVersions = entries.into();
        let collected: Vec<_> = tv.into_iter().collect();
        prop_assert_eq!(collected.len(), expected_len);
    }

    #[test]
    fn into_iter_borrowed_length_matches(
        entries in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let expected_len = entries.len();
        let tv: ToolVersions = entries.into();
        let collected: Vec<_> = (&tv).into_iter().collect();
        prop_assert_eq!(collected.len(), expected_len);
        prop_assert_eq!(tv.len(), expected_len);
    }

    //
    // FromIterator
    //

    #[test]
    fn from_iter_preserves_all_entries(
        entries in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into_iter().collect();

        for (name, version) in entries {
            prop_assert!(tv.contains(&name));
            prop_assert_eq!(tv.get_version(&name), Some(version.as_str()));
        }
    }

    #[test]
    fn from_iter_with_vec_versions(
        entries in prop::collection::hash_map(tool_name_strategy(), versions_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into_iter().collect();

        for (name, versions) in entries {
            let entry = tv.get(&name).unwrap();
            prop_assert_eq!(&entry.versions, &versions);
        }
    }

    //
    // Extend
    //

    #[test]
    fn extend_adds_all_entries(
        initial in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 0..5),
        additional in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..5)
    ) {
        let mut tv: ToolVersions = initial.into();
        tv.extend(additional.clone());

        for (name, version) in additional {
            prop_assert_eq!(tv.get_version(&name), Some(version.as_str()));
        }
    }

    #[test]
    fn extend_overwrites_on_conflict(
        name in tool_name_strategy(),
        version1 in version_strategy(),
        version2 in version_strategy()
    ) {
        let mut tv = ToolVersions::new();
        tv.set(&name, &version1);
        tv.extend(vec![(name.clone(), version2.clone())]);

        prop_assert_eq!(tv.get_version(&name), Some(version2.as_str()));
        prop_assert_eq!(tv.len(), 1);
    }

    //
    // Cross-type conversions
    //

    #[test]
    fn hashmap_to_btreemap_same_data(
        entries in prop::collection::hash_map(tool_name_strategy(), version_strategy(), 1..10)
    ) {
        let tv: ToolVersions = entries.clone().into();

        let hmap: HashMap<String, String> = (&tv).into();
        let bmap: BTreeMap<String, String> = tv.into();

        for (name, version) in entries {
            prop_assert_eq!(hmap.get(&name), Some(&version));
            prop_assert_eq!(bmap.get(&name), Some(&version));
        }
    }
}
