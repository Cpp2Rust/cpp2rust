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

pub struct RustlsSliceBytes {
    pub data: Value<Ptr<u8>>,
    pub len: Value<usize>,
}

impl Default for RustlsSliceBytes {
    fn default() -> Self {
        RustlsSliceBytes {
            data: Rc::new(RefCell::new(Ptr::null())),
            len: Rc::new(RefCell::new(0)),
        }
    }
}

impl ByteRepr for RustlsSliceBytes {}

impl RustlsSliceBytes {
    pub fn to_vec(&self) -> Vec<u8> {
        let len = *self.len.borrow();
        self.data.borrow().with_slice(len, |s| s.to_vec())
    }
}

const RUSTLS_SHIM_VERSION: &str = "rustls-ffi/0.15.3/rustls/0.23.0";

pub const RUSTLS_RESULT_OK: u32 = 7000;
pub const RUSTLS_RESULT_IO: u32 = 7001;
pub const RUSTLS_RESULT_NULL_PARAMETER: u32 = 7002;
pub const RUSTLS_RESULT_INVALID_DNS_NAME_ERROR: u32 = 7003;
pub const RUSTLS_RESULT_CERTIFICATE_PARSE_ERROR: u32 = 7005;
pub const RUSTLS_RESULT_PRIVATE_KEY_PARSE_ERROR: u32 = 7006;
pub const RUSTLS_RESULT_UNEXPECTED_EOF: u32 = 7010;
pub const RUSTLS_RESULT_PLAINTEXT_EMPTY: u32 = 7011;
pub const RUSTLS_RESULT_ALREADY_USED: u32 = 7013;
pub const RUSTLS_RESULT_CRL_PARSE_ERROR: u32 = 7014;
pub const RUSTLS_RESULT_NO_SERVER_CERT_VERIFIER: u32 = 7015;
pub const RUSTLS_RESULT_GET_RANDOM_FAILED: u32 = 7017;
pub const RUSTLS_RESULT_GENERAL: u32 = 7112;

pub fn map_rustls_error(err: &rustls::Error) -> u32 {
    use rustls::AlertDescription;
    use rustls::CertificateError;
    use rustls::Error;
    match err {
        Error::InappropriateMessage { .. } => 7109,
        Error::InappropriateHandshakeMessage { .. } => 7110,
        Error::NoCertificatesPresented => 7101,
        Error::DecryptError => 7102,
        Error::PeerIncompatible(_) => 7107,
        Error::PeerMisbehaved(_) => 7108,
        Error::UnsupportedNameType => 7115,
        Error::EncryptError => 7116,
        Error::FailedToGetCurrentTime => 7103,
        Error::FailedToGetRandomBytes => 7113,
        Error::HandshakeNotComplete => 7104,
        Error::PeerSentOversizedRecord => 7105,
        Error::NoApplicationProtocol => 7106,
        Error::BadMaxFragmentSize => 7114,
        Error::InvalidCertificate(e) => match e {
            CertificateError::BadEncoding => 7121,
            CertificateError::Expired | CertificateError::ExpiredContext { .. } => 7122,
            CertificateError::NotValidYet | CertificateError::NotValidYetContext { .. } => 7123,
            CertificateError::Revoked => 7124,
            CertificateError::UnhandledCriticalExtension => 7125,
            CertificateError::UnknownIssuer => 7126,
            CertificateError::UnknownRevocationStatus => 7154,
            CertificateError::ExpiredRevocationList
            | CertificateError::ExpiredRevocationListContext { .. } => 7156,
            CertificateError::BadSignature => 7127,
            CertificateError::UnsupportedSignatureAlgorithmContext { .. } => 7157,
            CertificateError::NotValidForName
            | CertificateError::NotValidForNameContext { .. } => 7128,
            CertificateError::InvalidPurpose
            | CertificateError::InvalidPurposeContext { .. } => 7129,
            CertificateError::ApplicationVerificationFailure => 7130,
            _ => 7131,
        },
        Error::AlertReceived(alert) => match alert {
            AlertDescription::CloseNotify => 7200,
            AlertDescription::HandshakeFailure => 7206,
            AlertDescription::BadCertificate => 7208,
            AlertDescription::UnknownCA => 7214,
            AlertDescription::DecodeError => 7216,
            AlertDescription::ProtocolVersion => 7219,
            AlertDescription::InternalError => 7221,
            _ => RUSTLS_RESULT_GENERAL,
        },
        Error::InvalidCertRevocationList(_) => RUSTLS_RESULT_CRL_PARSE_ERROR,
        _ => RUSTLS_RESULT_GENERAL,
    }
}

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
) -> u32 {
    provider_out.write(Ptr::alloc(RustlsCryptoProvider(
        builder.with(|b| b.build()),
    )));
    RUSTLS_RESULT_OK
}

pub fn rustls_crypto_provider_builder_new_from_default(
    builder_out: Ptr<Ptr<RustlsCryptoProviderBuilder>>,
) -> u32 {
    builder_out.write(Ptr::alloc(RustlsCryptoProviderBuilder {
        base: Arc::new(default_crypto_provider()),
        cipher_suites: Vec::new(),
    }));
    RUSTLS_RESULT_OK
}

pub fn rustls_crypto_provider_builder_set_cipher_suites(
    builder: Ptr<RustlsCryptoProviderBuilder>,
    cipher_suites: Ptr<Ptr<RustlsSupportedCiphersuite>>,
    cipher_suites_len: usize,
) -> u32 {
    let mut suites = Vec::with_capacity(cipher_suites_len);
    for i in 0..cipher_suites_len {
        suites.push(cipher_suites.offset(i).read().with(|c| c.0));
    }
    builder.with_mut(|b| b.cipher_suites = suites);
    RUSTLS_RESULT_OK
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

pub fn rustls_default_crypto_provider_random(buf: Ptr<u8>, len: usize) -> u32 {
    let mut tmp = Vec::new();
    tmp.resize(len, 0u8);
    match default_crypto_provider().secure_random.fill(&mut tmp) {
        Ok(()) => {
            if len > 0 {
                buf.with_slice_mut(len, |dst| dst.copy_from_slice(&tmp));
            }
            RUSTLS_RESULT_OK
        }
        Err(_) => RUSTLS_RESULT_GET_RANDOM_FAILED,
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
