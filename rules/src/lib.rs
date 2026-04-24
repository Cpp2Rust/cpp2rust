// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#![allow(warnings)]
mod modules;
pub use modules::*;

pub use modules::stdio_tgt_refcount::f5 as fread_refcount;
pub use modules::stdio_tgt_unsafe::f5 as fread_unsafe;
