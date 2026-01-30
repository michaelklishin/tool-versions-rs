// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}
