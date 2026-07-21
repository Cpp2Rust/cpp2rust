// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

use crate::{ByteRepr, Ptr, Value};

pub struct RustlsStr {
    pub data: Value<Ptr<u8>>,
    pub len: Value<usize>,
}

impl Default for RustlsStr {
    fn default() -> Self {
        RustlsStr {
            data: Rc::new(RefCell::new(Ptr::null())),
            len: Rc::new(RefCell::new(0)),
        }
    }
}

impl ByteRepr for RustlsStr {}

impl RustlsStr {
    pub fn copy_from(s: &str) -> RustlsStr {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);
        RustlsStr {
            data: Rc::new(RefCell::new(Ptr::alloc_array(bytes.into_boxed_slice()))),
            len: Rc::new(RefCell::new(s.len())),
        }
    }
}

const RUSTLS_SHIM_VERSION: &str = "rustls-ffi/0.15.3/rustls/0.23.0";

pub fn rustls_version_refcount() -> RustlsStr {
    RustlsStr::copy_from(RUSTLS_SHIM_VERSION)
}
