# .tool-versions in Rust

A Rust library for parsing and manipulating [`.tool-versions`](https://asdf-vm.com/manage/configuration.html) files used by [asdf](https://asdf-vm.com/) and compatible version managers.

## Usage

```rust
use tool_versions::ToolVersions;

// Parse from string
let content = "erlang 26.2.1\nruby 3.3.0";
let tv = ToolVersions::parse(content)?;

// Get a tool's version
assert_eq!(tv.get_version("erlang"), Some("26.2.1"));

// Load from a file, modify, and save back
let mut tv = ToolVersions::load(".tool-versions")?;
tv.set("nodejs", "22.0.0");
tv.save(".tool-versions")?;

// Load or create if the file doesn't exist
let mut tv = ToolVersions::load_or_default(".tool-versions")?;
tv.set("python", "3.12.0");
tv.save(".tool-versions")?;
```

## Features

- Parse `.tool-versions` files preserving comments and blank lines
- Get, set, and remove tool versions
- Round-trip editing (preserves file structure)
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
