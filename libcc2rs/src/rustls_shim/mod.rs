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

pub fn rustls_version() -> RustlsStr {
    RustlsStr::copy_from(RUSTLS_SHIM_VERSION)
}

fn rustls_result_message(result: u32) -> String {
    match result {
        7000 => "OK".to_string(),
        7001 => "I/O error".to_string(),
        7002 => "a parameter was NULL".to_string(),
        7003 => "server name was malformed (not a valid hostname or IP address)".to_string(),
        7004 => "a Rust component panicked".to_string(),
        7005 => "error parsing certificate".to_string(),
        7006 => "error parsing private key".to_string(),
        7007 => "provided buffer is of insufficient size".to_string(),
        7008 => "the item was not found".to_string(),
        7009 => "a parameter had an invalid value".to_string(),
        7010 => "peer closed TCP connection without first closing TLS connection".to_string(),
        7011 => "no plaintext available; call rustls_connection_read_tls again".to_string(),
        7014 => "error parsing certificate revocation list (CRL)".to_string(),
        7015 => "no server certificate verifier was configured on the client config builder"
            .to_string(),
        7016 => "no default process-wide crypto provider has been installed".to_string(),
        7017 => "failed to get random bytes from the crypto provider".to_string(),
        other => format!("rustls result {other}"),
    }
}

pub fn rustls_error(result: u32, buf: Ptr<u8>, len: usize, out_n: Ptr<usize>) {
    if buf.is_null() || out_n.is_null() {
        return;
    }
    let msg = rustls_result_message(result);
    let bytes = msg.as_bytes();
    let n = len.min(bytes.len());
    if n > 0 {
        buf.with_slice_mut(n, |dst| dst.copy_from_slice(&bytes[..n]));
    }
    out_n.write(n);
}
