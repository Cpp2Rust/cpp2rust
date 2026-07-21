// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use rustls::SupportedCipherSuite;
use rustls::crypto::CryptoProvider;

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

pub fn default_crypto_provider() -> CryptoProvider {
    rustls::crypto::aws_lc_rs::default_provider()
}

pub struct RustlsCryptoProvider(pub CryptoProvider);
impl ByteRepr for RustlsCryptoProvider {}

pub struct RustlsCryptoProviderBuilder {
    pub base: Arc<CryptoProvider>,
    pub cipher_suites: Vec<SupportedCipherSuite>,
}
impl ByteRepr for RustlsCryptoProviderBuilder {}

impl RustlsCryptoProviderBuilder {
    pub fn build(&self) -> CryptoProvider {
        let cipher_suites = if self.cipher_suites.is_empty() {
            self.base.cipher_suites.clone()
        } else {
            self.cipher_suites.clone()
        };
        CryptoProvider {
            cipher_suites,
            kx_groups: self.base.kx_groups.clone(),
            signature_verification_algorithms: self.base.signature_verification_algorithms,
            secure_random: self.base.secure_random,
            key_provider: self.base.key_provider,
        }
    }
}

#[derive(Clone, Copy)]
pub struct RustlsSupportedCiphersuite(pub SupportedCipherSuite);
impl ByteRepr for RustlsSupportedCiphersuite {}

pub fn rustls_crypto_provider_builder_build(
    builder: Ptr<RustlsCryptoProviderBuilder>,
    provider_out: Ptr<Ptr<RustlsCryptoProvider>>,
) {
    provider_out.write(Ptr::alloc(RustlsCryptoProvider(
        builder.with(|b| b.build()),
    )));
}

pub fn rustls_crypto_provider_builder_new_from_default(
    builder_out: Ptr<Ptr<RustlsCryptoProviderBuilder>>,
) {
    builder_out.write(Ptr::alloc(RustlsCryptoProviderBuilder {
        base: Arc::new(default_crypto_provider()),
        cipher_suites: Vec::new(),
    }));
}

pub fn rustls_crypto_provider_builder_set_cipher_suites(
    builder: Ptr<RustlsCryptoProviderBuilder>,
    cipher_suites: Ptr<Ptr<RustlsSupportedCiphersuite>>,
    cipher_suites_len: usize,
) {
    let mut suites = Vec::with_capacity(cipher_suites_len);
    for i in 0..cipher_suites_len {
        suites.push(cipher_suites.offset(i).read().with(|c| c.0));
    }
    builder.with_mut(|b| b.cipher_suites = suites);
}

pub fn rustls_default_crypto_provider_ciphersuites_get(
    index: usize,
) -> Ptr<RustlsSupportedCiphersuite> {
    match default_crypto_provider().cipher_suites.get(index) {
        Some(cs) => Ptr::alloc(RustlsSupportedCiphersuite(*cs)),
        None => Ptr::null(),
    }
}

pub fn rustls_default_crypto_provider_ciphersuites_len() -> usize {
    default_crypto_provider().cipher_suites.len()
}

pub fn rustls_default_crypto_provider_random(buf: Ptr<u8>, len: usize) -> bool {
    let mut tmp = Vec::new();
    tmp.resize(len, 0u8);
    match default_crypto_provider().secure_random.fill(&mut tmp) {
        Ok(()) => {
            if len > 0 {
                buf.with_slice_mut(len, |dst| dst.copy_from_slice(&tmp));
            }
            true
        }
        Err(_) => false,
    }
}

pub fn rustls_supported_ciphersuite_get_suite(suite: Ptr<RustlsSupportedCiphersuite>) -> u16 {
    suite.with(|c| u16::from(c.0.suite()))
}

pub fn rustls_supported_ciphersuite_protocol_version(
    suite: Ptr<RustlsSupportedCiphersuite>,
) -> u16 {
    suite.with(|c| u16::from(c.0.version().version))
}
