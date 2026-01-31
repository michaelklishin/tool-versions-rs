# .tool-versions in Rust

A Rust library for parsing and manipulating [`.tool-versions`](https://asdf-vm.com/manage/configuration.html) files used by [asdf](https://asdf-vm.com/) and compatible version managers.

## Usage

```rust
use tool_versions::ToolVersions;

// Parse from string
let content = "erlang 27.3.4.6\nruby 3.4.1";
let tv = ToolVersions::parse(content)?;

// Get a tool's version
assert_eq!(tv.get_version("erlang"), Some("27.3.4.6"));

// Load from a file, modify, and save back
let mut tv = ToolVersions::load(".tool-versions")?;
tv.set("nodejs", "24.0.0");
tv.save(".tool-versions")?;

// Load or create if the file doesn't exist
let mut tv = ToolVersions::load_or_default(".tool-versions")?;
tv.set("python", "3.13.0");
tv.save(".tool-versions")?;

// Iterate over all tools
for (name, versions) in &tv {
    println!("{}: {:?}", name, versions);
}

// Convert to a HashMap
use std::collections::HashMap;
let map: HashMap<String, String> = tv.into();

// Build from a HashMap
let mut map = HashMap::new();
map.insert("rust".to_string(), "1.93.0".to_string());
map.insert("go".to_string(), "1.25.0".to_string());
let tv: ToolVersions = map.into();

// Collect from an iterator
let tv: ToolVersions = [
    ("erlang".to_string(), "27.3.4.6".to_string()),
    ("elixir".to_string(), "1.18.0".to_string()),
].into_iter().collect();
```

## Features

- Parse `.tool-versions` files preserving comments and blank lines
- Get, set, and remove tool versions
- Round-trip editing (preserves file structure)
- Convert to/from `HashMap` and `BTreeMap`
- Iterate, collect, and extend using standard traits
- Optional serde support with the `serde` feature

## Serde Support

Enable the `serde` feature for serialization support:

```toml
[dependencies]
tool-versions = { version = "1.0", features = ["serde"] }
```

## License

This library is double-licensed under the MIT License and the Apache License, Version 2.0.

## Copyright

(c) 2025-2026 Michael S. Klishin and Contributors.
