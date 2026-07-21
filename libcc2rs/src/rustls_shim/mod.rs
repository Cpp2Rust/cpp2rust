// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::io::{ErrorKind, Read, Write};
use std::rc::Rc;
use std::sync::Arc;

use rustls::RootCertStore;
use rustls::SupportedCipherSuite;
use rustls::crypto::CryptoProvider;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::pem::PemObject;

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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RustlsResult {
    Ok = 7000,
    Io = 7001,
    NullParameter = 7002,
    InvalidDnsNameError = 7003,
    Panic = 7004,
    CertificateParseError = 7005,
    PrivateKeyParseError = 7006,
    InsufficientSize = 7007,
    NotFound = 7008,
    InvalidParameter = 7009,
    UnexpectedEof = 7010,
    PlaintextEmpty = 7011,
    AcceptorNotReady = 7012,
    AlreadyUsed = 7013,
    CertificateRevocationListParseError = 7014,
    NoServerCertVerifier = 7015,
    NoDefaultCryptoProvider = 7016,
    GetRandomFailed = 7017,
    NoCertificatesPresented = 7101,
    DecryptError = 7102,
    FailedToGetCurrentTime = 7103,
    HandshakeNotComplete = 7104,
    PeerSentOversizedRecord = 7105,
    NoApplicationProtocol = 7106,
    PeerIncompatibleError = 7107,
    PeerMisbehavedError = 7108,
    InappropriateMessage = 7109,
    InappropriateHandshakeMessage = 7110,
    General = 7112,
    FailedToGetRandomBytes = 7113,
    BadMaxFragmentSize = 7114,
    UnsupportedNameType = 7115,
    EncryptError = 7116,
    CertEncodingBad = 7121,
    CertExpired = 7122,
    CertNotYetValid = 7123,
    CertRevoked = 7124,
    CertUnhandledCriticalExtension = 7125,
    CertUnknownIssuer = 7126,
    CertBadSignature = 7127,
    CertNotValidForName = 7128,
    CertInvalidPurpose = 7129,
    CertApplicationVerificationFailure = 7130,
    CertOtherError = 7131,
    CertUnknownRevocationStatus = 7154,
    CertExpiredRevocationList = 7156,
    CertUnsupportedSignatureAlgorithm = 7157,
    AlertCloseNotify = 7200,
    AlertHandshakeFailure = 7206,
    AlertBadCertificate = 7208,
    AlertUnknownCA = 7214,
    AlertDecodeError = 7216,
    AlertProtocolVersion = 7219,
    AlertInternalError = 7221,
}

impl ByteRepr for RustlsResult {}

pub fn map_rustls_error(err: &rustls::Error) -> RustlsResult {
    use RustlsResult::*;
    use rustls::AlertDescription;
    use rustls::CertificateError;
    use rustls::Error;
    match err {
        Error::InappropriateMessage { .. } => InappropriateMessage,
        Error::InappropriateHandshakeMessage { .. } => InappropriateHandshakeMessage,
        Error::NoCertificatesPresented => NoCertificatesPresented,
        Error::DecryptError => DecryptError,
        Error::PeerIncompatible(_) => PeerIncompatibleError,
        Error::PeerMisbehaved(_) => PeerMisbehavedError,
        Error::UnsupportedNameType => UnsupportedNameType,
        Error::EncryptError => EncryptError,
        Error::FailedToGetCurrentTime => FailedToGetCurrentTime,
        Error::FailedToGetRandomBytes => FailedToGetRandomBytes,
        Error::HandshakeNotComplete => HandshakeNotComplete,
        Error::PeerSentOversizedRecord => PeerSentOversizedRecord,
        Error::NoApplicationProtocol => NoApplicationProtocol,
        Error::BadMaxFragmentSize => BadMaxFragmentSize,
        Error::InvalidCertificate(e) => match e {
            CertificateError::BadEncoding => CertEncodingBad,
            CertificateError::Expired | CertificateError::ExpiredContext { .. } => CertExpired,
            CertificateError::NotValidYet | CertificateError::NotValidYetContext { .. } => {
                CertNotYetValid
            }
            CertificateError::Revoked => CertRevoked,
            CertificateError::UnhandledCriticalExtension => CertUnhandledCriticalExtension,
            CertificateError::UnknownIssuer => CertUnknownIssuer,
            CertificateError::UnknownRevocationStatus => CertUnknownRevocationStatus,
            CertificateError::ExpiredRevocationList
            | CertificateError::ExpiredRevocationListContext { .. } => CertExpiredRevocationList,
            CertificateError::BadSignature => CertBadSignature,
            CertificateError::UnsupportedSignatureAlgorithmContext { .. } => {
                CertUnsupportedSignatureAlgorithm
            }
            CertificateError::NotValidForName
            | CertificateError::NotValidForNameContext { .. } => CertNotValidForName,
            CertificateError::InvalidPurpose
            | CertificateError::InvalidPurposeContext { .. } => CertInvalidPurpose,
            CertificateError::ApplicationVerificationFailure => {
                CertApplicationVerificationFailure
            }
            _ => CertOtherError,
        },
        Error::AlertReceived(alert) => match alert {
            AlertDescription::CloseNotify => AlertCloseNotify,
            AlertDescription::HandshakeFailure => AlertHandshakeFailure,
            AlertDescription::BadCertificate => AlertBadCertificate,
            AlertDescription::UnknownCA => AlertUnknownCA,
            AlertDescription::DecodeError => AlertDecodeError,
            AlertDescription::ProtocolVersion => AlertProtocolVersion,
            AlertDescription::InternalError => AlertInternalError,
            _ => General,
        },
        Error::InvalidCertRevocationList(_) => CertificateRevocationListParseError,
        _ => General,
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

pub struct RustlsCertificate(pub CertificateDer<'static>);
impl ByteRepr for RustlsCertificate {}

pub struct RustlsRootCertStore(pub Arc<RootCertStore>);
impl ByteRepr for RustlsRootCertStore {}

pub struct RustlsRootCertStoreBuilder(pub Option<RootCertStore>);
impl ByteRepr for RustlsRootCertStoreBuilder {}

pub fn rustls_root_cert_store_builder_new() -> Ptr<RustlsRootCertStoreBuilder> {
    Ptr::alloc(RustlsRootCertStoreBuilder(Some(RootCertStore::empty())))
}

fn add_certs_to_builder(
    builder: Ptr<RustlsRootCertStoreBuilder>,
    certs: Vec<CertificateDer<'static>>,
    strict: bool,
) -> RustlsResult {
    builder.with_mut(|b| match b.0.as_mut() {
        None => RustlsResult::AlreadyUsed,
        Some(roots) => {
            let mut new_store = RootCertStore::empty();
            let (parsed, rejected) = new_store.add_parsable_certificates(certs);
            if strict && (rejected > 0 || parsed == 0) {
                return RustlsResult::CertificateParseError;
            }
            roots.roots.append(&mut new_store.roots);
            RustlsResult::Ok
        }
    })
}

pub fn rustls_root_cert_store_builder_add_pem(
    builder: Ptr<RustlsRootCertStoreBuilder>,
    pem: Ptr<u8>,
    pem_len: usize,
    strict: bool,
) -> RustlsResult {
    if pem.is_null() {
        return RustlsResult::NullParameter;
    }
    let certs = match pem.with_slice(pem_len, |s| {
        CertificateDer::pem_slice_iter(s).collect::<Result<Vec<_>, _>>()
    }) {
        Ok(certs) => certs,
        Err(_) => return RustlsResult::CertificateParseError,
    };
    add_certs_to_builder(builder, certs, strict)
}

pub fn rustls_root_cert_store_builder_load_roots_from_file(
    builder: Ptr<RustlsRootCertStoreBuilder>,
    filename: Ptr<u8>,
    strict: bool,
) -> RustlsResult {
    if filename.is_null() {
        return RustlsResult::NullParameter;
    }
    let filename = filename.to_rust_string();
    let certs = match CertificateDer::pem_file_iter(&filename) {
        Ok(certs) => certs,
        Err(_) => return RustlsResult::Io,
    };
    let certs = match certs.collect::<Result<Vec<_>, _>>() {
        Ok(certs) => certs,
        Err(_) => return RustlsResult::CertificateParseError,
    };
    add_certs_to_builder(builder, certs, strict)
}

pub fn rustls_root_cert_store_builder_build(
    builder: Ptr<RustlsRootCertStoreBuilder>,
    root_cert_store_out: Ptr<Ptr<RustlsRootCertStore>>,
) -> RustlsResult {
    if root_cert_store_out.is_null() {
        return RustlsResult::NullParameter;
    }
    builder.with_mut(|b| match b.0.take() {
        None => RustlsResult::AlreadyUsed,
        Some(roots) => {
            root_cert_store_out.write(Ptr::alloc(RustlsRootCertStore(Arc::new(roots))));
            RustlsResult::Ok
        }
    })
}

pub struct RustlsConnection {
    pub conn: rustls::ClientConnection,
}
impl ByteRepr for RustlsConnection {}

pub fn rustls_connection_read(
    conn: Ptr<RustlsConnection>,
    buf: Ptr<u8>,
    count: usize,
    out_n: Ptr<usize>,
) -> RustlsResult {
    if buf.is_null() || out_n.is_null() {
        return RustlsResult::NullParameter;
    }
    let n_read = conn.with_mut(|c| buf.with_slice_mut(count, |dst| c.conn.reader().read(dst)));
    match n_read {
        Ok(n) => {
            out_n.write(n);
            RustlsResult::Ok
        }
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => RustlsResult::UnexpectedEof,
        Err(e) if e.kind() == ErrorKind::WouldBlock => RustlsResult::PlaintextEmpty,
        Err(_) => RustlsResult::Io,
    }
}

pub fn rustls_connection_write(
    conn: Ptr<RustlsConnection>,
    buf: Ptr<u8>,
    count: usize,
    out_n: Ptr<usize>,
) -> RustlsResult {
    if buf.is_null() || out_n.is_null() {
        return RustlsResult::NullParameter;
    }
    let n_written = conn.with_mut(|c| buf.with_slice(count, |src| c.conn.writer().write(src)));
    match n_written {
        Ok(n) => {
            out_n.write(n);
            RustlsResult::Ok
        }
        Err(_) => RustlsResult::Io,
    }
}

pub fn rustls_connection_process_new_packets(conn: Ptr<RustlsConnection>) -> RustlsResult {
    match conn.with_mut(|c| c.conn.process_new_packets().map(|_| ())) {
        Ok(()) => RustlsResult::Ok,
        Err(e) => map_rustls_error(&e),
    }
}

pub fn rustls_connection_wants_read(conn: Ptr<RustlsConnection>) -> bool {
    conn.with(|c| c.conn.wants_read())
}

pub fn rustls_connection_wants_write(conn: Ptr<RustlsConnection>) -> bool {
    conn.with(|c| c.conn.wants_write())
}

pub fn rustls_connection_is_handshaking(conn: Ptr<RustlsConnection>) -> bool {
    conn.with(|c| c.conn.is_handshaking())
}

pub fn rustls_connection_send_close_notify(conn: Ptr<RustlsConnection>) {
    conn.with_mut(|c| c.conn.send_close_notify());
}

pub fn rustls_connection_get_alpn_protocol(
    conn: Ptr<RustlsConnection>,
    protocol_out: Ptr<Ptr<u8>>,
    protocol_out_len: Ptr<usize>,
) {
    if protocol_out.is_null() || protocol_out_len.is_null() {
        return;
    }
    conn.with(|c| match c.conn.alpn_protocol() {
        Some(p) => {
            protocol_out.write(Ptr::alloc_array(p.to_vec().into_boxed_slice()));
            protocol_out_len.write(p.len());
        }
        None => {
            protocol_out.write(Ptr::null());
            protocol_out_len.write(0);
        }
    });
}

pub fn rustls_connection_get_protocol_version(conn: Ptr<RustlsConnection>) -> u16 {
    conn.with(|c| c.conn.protocol_version().map(u16::from).unwrap_or_default())
}

pub fn rustls_connection_get_negotiated_ciphersuite_name(conn: Ptr<RustlsConnection>) -> RustlsStr {
    conn.with(|c| {
        RustlsStr::copy_from(
            c.conn
                .negotiated_cipher_suite()
                .and_then(|cs| cs.suite().as_str())
                .unwrap_or_default(),
        )
    })
}

pub fn rustls_connection_get_negotiated_key_exchange_group_name(
    conn: Ptr<RustlsConnection>,
) -> RustlsStr {
    conn.with(|c| {
        RustlsStr::copy_from(
            c.conn
                .negotiated_key_exchange_group()
                .and_then(|kxg| kxg.name().as_str())
                .unwrap_or_default(),
        )
    })
}

pub fn rustls_connection_get_peer_certificate(
    conn: Ptr<RustlsConnection>,
    i: usize,
) -> Ptr<RustlsCertificate> {
    conn.with(|c| {
        match c.conn.peer_certificates().and_then(|certs| certs.get(i)) {
            Some(cert) => Ptr::alloc(RustlsCertificate(cert.clone().into_owned())),
            None => Ptr::null(),
        }
    })
}

pub fn rustls_certificate_get_der(
    cert: Ptr<RustlsCertificate>,
    out_der_data: Ptr<Ptr<u8>>,
    out_der_len: Ptr<usize>,
) -> RustlsResult {
    if out_der_data.is_null() || out_der_len.is_null() {
        return RustlsResult::NullParameter;
    }
    cert.with(|c| {
        let der = c.0.as_ref();
        out_der_data.write(Ptr::alloc_array(der.to_vec().into_boxed_slice()));
        out_der_len.write(der.len());
    });
    RustlsResult::Ok
}

pub fn rustls_crypto_provider_builder_build(
    builder: Ptr<RustlsCryptoProviderBuilder>,
    provider_out: Ptr<Ptr<RustlsCryptoProvider>>,
) -> RustlsResult {
    provider_out.write(Ptr::alloc(RustlsCryptoProvider(
        builder.with(|b| b.build()),
    )));
    RustlsResult::Ok
}

pub fn rustls_crypto_provider_builder_new_from_default(
    builder_out: Ptr<Ptr<RustlsCryptoProviderBuilder>>,
) -> RustlsResult {
    builder_out.write(Ptr::alloc(RustlsCryptoProviderBuilder {
        base: Arc::new(default_crypto_provider()),
        cipher_suites: Vec::new(),
    }));
    RustlsResult::Ok
}

pub fn rustls_crypto_provider_builder_set_cipher_suites(
    builder: Ptr<RustlsCryptoProviderBuilder>,
    cipher_suites: Ptr<Ptr<RustlsSupportedCiphersuite>>,
    cipher_suites_len: usize,
) -> RustlsResult {
    let mut suites = Vec::with_capacity(cipher_suites_len);
    for i in 0..cipher_suites_len {
        suites.push(cipher_suites.offset(i).read().with(|c| c.0));
    }
    builder.with_mut(|b| b.cipher_suites = suites);
    RustlsResult::Ok
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

pub fn rustls_default_crypto_provider_random(buf: Ptr<u8>, len: usize) -> RustlsResult {
    let mut tmp = Vec::new();
    tmp.resize(len, 0u8);
    match default_crypto_provider().secure_random.fill(&mut tmp) {
        Ok(()) => {
            if len > 0 {
                buf.with_slice_mut(len, |dst| dst.copy_from_slice(&tmp));
            }
            RustlsResult::Ok
        }
        Err(_) => RustlsResult::GetRandomFailed,
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
