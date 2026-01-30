// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! tool-versions - Parser and manipulation library for .tool-versions files
//!
//! This crate provides functionality to parse, modify, and serialize
//! `.tool-versions` files used by asdf and compatible version managers.

pub mod errors;
pub mod tool_versions;

pub use errors::Error;
pub use tool_versions::{ToolEntry, ToolVersions};

pub type Result<T> = std::result::Result<T, Error>;
