// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::io::{ErrorKind, Read, Write};
use std::rc::Rc;
use std::sync::Arc;

use rustls::RootCertStore;
use rustls::SupportedCipherSuite;
use rustls::DigitallySignedStruct;
use rustls::client::ResolvesClientCert;
use rustls::client::WebPkiServerVerifier;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified};
use rustls::pki_types::UnixTime;
use rustls::{ClientConfig, KeyLog, ProtocolVersion, SignatureScheme, SupportedProtocolVersion};
use rustls::pki_types::ServerName;
use rustls::client::danger::ServerCertVerifier;
use rustls::crypto::CryptoProvider;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::CertificateRevocationListDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::pem::PemObject;
use rustls::server::VerifierBuilderError;
use rustls::sign::CertifiedKey;

use crate::{AnyPtr, ByteRepr, FnPtr, Ptr, Value};

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

pub struct RustlsCertifiedKey(pub Arc<CertifiedKey>);
impl ByteRepr for RustlsCertifiedKey {}

pub fn rustls_certified_key_build(
    cert_chain: Ptr<u8>,
    cert_chain_len: usize,
    private_key: Ptr<u8>,
    private_key_len: usize,
    certified_key_out: Ptr<Ptr<RustlsCertifiedKey>>,
) -> RustlsResult {
    if cert_chain.is_null() || private_key.is_null() || certified_key_out.is_null() {
        return RustlsResult::NullParameter;
    }
    let private_key_der = match private_key.with_slice(private_key_len, |s| {
        PrivateKeyDer::from_pem_slice(s)
    }) {
        Ok(der) => der,
        Err(_) => return RustlsResult::PrivateKeyParseError,
    };
    let signing_key = match default_crypto_provider()
        .key_provider
        .load_private_key(private_key_der)
    {
        Ok(key) => key,
        Err(e) => return map_rustls_error(&e),
    };
    let parsed_chain = match cert_chain.with_slice(cert_chain_len, |s| {
        CertificateDer::pem_slice_iter(s).collect::<Result<Vec<_>, _>>()
    }) {
        Ok(chain) => chain,
        Err(_) => return RustlsResult::CertificateParseError,
    };
    certified_key_out.write(Ptr::alloc(RustlsCertifiedKey(Arc::new(CertifiedKey::new(
        parsed_chain,
        signing_key,
    )))));
    RustlsResult::Ok
}

pub fn rustls_certified_key_keys_match(key: Ptr<RustlsCertifiedKey>) -> RustlsResult {
    match key.with(|k| k.0.keys_match()) {
        Ok(()) => RustlsResult::Ok,
        Err(e) => map_rustls_error(&e),
    }
}

pub struct RustlsServerCertVerifier(pub Arc<dyn ServerCertVerifier>);
impl ByteRepr for RustlsServerCertVerifier {}

pub struct WebPkiVerifierBuilderState {
    pub roots: Arc<RootCertStore>,
    pub crls: Vec<CertificateRevocationListDer<'static>>,
}

pub struct RustlsWebPkiServerCertVerifierBuilder(pub Option<WebPkiVerifierBuilderState>);
impl ByteRepr for RustlsWebPkiServerCertVerifierBuilder {}

pub fn rustls_web_pki_server_cert_verifier_builder_new(
    store: Ptr<RustlsRootCertStore>,
) -> Ptr<RustlsWebPkiServerCertVerifierBuilder> {
    let roots = store.with(|s| s.0.clone());
    Ptr::alloc(RustlsWebPkiServerCertVerifierBuilder(Some(
        WebPkiVerifierBuilderState {
            roots,
            crls: Vec::new(),
        },
    )))
}

pub fn rustls_web_pki_server_cert_verifier_builder_add_crl(
    builder: Ptr<RustlsWebPkiServerCertVerifierBuilder>,
    crl_pem: Ptr<u8>,
    crl_pem_len: usize,
) -> RustlsResult {
    if crl_pem.is_null() {
        return RustlsResult::NullParameter;
    }
    let crls = match crl_pem.with_slice(crl_pem_len, |s| {
        CertificateRevocationListDer::pem_slice_iter(s).collect::<Result<Vec<_>, _>>()
    }) {
        Ok(crls) => crls,
        Err(_) => return RustlsResult::CertificateRevocationListParseError,
    };
    if crls.is_empty() {
        return RustlsResult::CertificateRevocationListParseError;
    }
    builder.with_mut(|b| match b.0.as_mut() {
        None => RustlsResult::AlreadyUsed,
        Some(state) => {
            state.crls.extend(crls);
            RustlsResult::Ok
        }
    })
}

pub fn rustls_web_pki_server_cert_verifier_builder_build(
    builder: Ptr<RustlsWebPkiServerCertVerifierBuilder>,
    verifier_out: Ptr<Ptr<RustlsServerCertVerifier>>,
) -> RustlsResult {
    if verifier_out.is_null() {
        return RustlsResult::NullParameter;
    }
    builder.with_mut(|b| match b.0.take() {
        None => RustlsResult::AlreadyUsed,
        Some(state) => {
            let verifier_builder = WebPkiServerVerifier::builder_with_provider(
                state.roots,
                Arc::new(default_crypto_provider()),
            )
            .with_crls(state.crls);
            match verifier_builder.build() {
                Ok(verifier) => {
                    verifier_out.write(Ptr::alloc(RustlsServerCertVerifier(verifier)));
                    RustlsResult::Ok
                }
                Err(VerifierBuilderError::InvalidCrl(_)) => {
                    RustlsResult::CertificateRevocationListParseError
                }
                Err(_) => RustlsResult::General,
            }
        }
    })
}

pub fn rustls_platform_server_cert_verifier(
    verifier_out: Ptr<Ptr<RustlsServerCertVerifier>>,
) -> RustlsResult {
    if verifier_out.is_null() {
        return RustlsResult::NullParameter;
    }
    match rustls_platform_verifier::Verifier::new(Arc::new(default_crypto_provider())) {
        Ok(verifier) => {
            verifier_out.write(Ptr::alloc(RustlsServerCertVerifier(Arc::new(verifier))));
            RustlsResult::Ok
        }
        Err(e) => map_rustls_error(&e),
    }
}

pub struct RustlsClientConfig(pub Arc<ClientConfig>);
impl ByteRepr for RustlsClientConfig {}

pub struct RustlsClientConfigBuilder {
    pub provider: Arc<CryptoProvider>,
    pub versions: Vec<&'static SupportedProtocolVersion>,
    pub verifier: Option<Arc<dyn ServerCertVerifier>>,
    pub alpn_protocols: Vec<Vec<u8>>,
    pub cert_resolver: Option<Arc<dyn ResolvesClientCert>>,
    pub key_log: Option<Arc<dyn KeyLog>>,
}
impl ByteRepr for RustlsClientConfigBuilder {}

pub type RustlsKeylogLogCallback = fn(RustlsStr, Ptr<u8>, usize, Ptr<u8>, usize);
pub type RustlsKeylogWillLogCallback = fn(RustlsStr) -> i32;

#[derive(Debug)]
struct CallbackKeyLog {
    log_cb: RustlsKeylogLogCallback,
    will_log_cb: Option<RustlsKeylogWillLogCallback>,
}

impl KeyLog for CallbackKeyLog {
    fn log(&self, label: &str, client_random: &[u8], secret: &[u8]) {
        let cr = Ptr::alloc_array(client_random.to_vec().into_boxed_slice());
        let sec = Ptr::alloc_array(secret.to_vec().into_boxed_slice());
        (self.log_cb)(
            RustlsStr::copy_from(label),
            cr.clone(),
            client_random.len(),
            sec.clone(),
            secret.len(),
        );
        cr.delete_array();
        sec.delete_array();
    }

    fn will_log(&self, label: &str) -> bool {
        match self.will_log_cb {
            Some(cb) => cb(RustlsStr::copy_from(label)) != 0,
            None => true,
        }
    }
}

pub fn rustls_client_config_builder_set_key_log(
    builder: Ptr<RustlsClientConfigBuilder>,
    log_cb: FnPtr<RustlsKeylogLogCallback>,
    will_log_cb: FnPtr<RustlsKeylogWillLogCallback>,
) -> RustlsResult {
    if log_cb.is_null() {
        return RustlsResult::NullParameter;
    }
    let key_log = CallbackKeyLog {
        log_cb: *log_cb,
        will_log_cb: match will_log_cb.is_null() {
            true => None,
            false => Some(*will_log_cb),
        },
    };
    builder.with_mut(|b| b.key_log = Some(Arc::new(key_log)));
    RustlsResult::Ok
}

#[derive(Debug)]
struct ResolvesClientCertFromChoices {
    keys: Vec<Arc<CertifiedKey>>,
}

impl ResolvesClientCert for ResolvesClientCertFromChoices {
    fn resolve(
        &self,
        _root_hint_subjects: &[&[u8]],
        sig_schemes: &[SignatureScheme],
    ) -> Option<Arc<CertifiedKey>> {
        for key in self.keys.iter() {
            if key.key.choose_scheme(sig_schemes).is_some() {
                return Some(key.clone());
            }
        }
        None
    }

    fn has_certs(&self) -> bool {
        !self.keys.is_empty()
    }
}

pub fn rustls_client_config_builder_new_custom(
    provider: Ptr<RustlsCryptoProvider>,
    tls_versions: Ptr<u16>,
    tls_versions_len: usize,
    builder_out: Ptr<Ptr<RustlsClientConfigBuilder>>,
) -> RustlsResult {
    if provider.is_null() || builder_out.is_null() {
        return RustlsResult::NullParameter;
    }
    let provider = provider.with(|p| Arc::new(p.0.clone()));
    let mut versions = Vec::new();
    for i in 0..tls_versions_len {
        let proto = ProtocolVersion::from(tls_versions.offset(i).read());
        if proto == rustls::version::TLS12.version {
            versions.push(&rustls::version::TLS12);
        } else if proto == rustls::version::TLS13.version {
            versions.push(&rustls::version::TLS13);
        }
    }
    builder_out.write(Ptr::alloc(RustlsClientConfigBuilder {
        provider,
        versions,
        verifier: None,
        alpn_protocols: Vec::new(),
        cert_resolver: None,
        key_log: None,
    }));
    RustlsResult::Ok
}

pub fn rustls_client_config_builder_set_alpn_protocols(
    builder: Ptr<RustlsClientConfigBuilder>,
    protocols: Ptr<RustlsSliceBytes>,
    len: usize,
) -> RustlsResult {
    let mut vv = Vec::with_capacity(len);
    for i in 0..len {
        vv.push(protocols.offset(i).with(|p| p.to_vec()));
    }
    builder.with_mut(|b| b.alpn_protocols = vv);
    RustlsResult::Ok
}

pub fn rustls_client_config_builder_set_certified_key(
    builder: Ptr<RustlsClientConfigBuilder>,
    certified_keys: Ptr<Ptr<RustlsCertifiedKey>>,
    certified_keys_len: usize,
) -> RustlsResult {
    let mut keys = Vec::new();
    for i in 0..certified_keys_len {
        let key_ptr = certified_keys.offset(i).read();
        if key_ptr.is_null() {
            return RustlsResult::NullParameter;
        }
        keys.push(key_ptr.with(|k| k.0.clone()));
    }
    builder.with_mut(|b| {
        b.cert_resolver = Some(Arc::new(ResolvesClientCertFromChoices { keys }))
    });
    RustlsResult::Ok
}

#[derive(Default)]
pub struct RustlsVerifyServerCertParams;
impl ByteRepr for RustlsVerifyServerCertParams {}

pub type RustlsVerifyServerCertCallback = fn(AnyPtr, Ptr<RustlsVerifyServerCertParams>) -> u32;

#[derive(Debug)]
struct CallbackVerifier {
    cb: RustlsVerifyServerCertCallback,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for CallbackVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        match (self.cb)(AnyPtr::default(), Ptr::null()) {
            7000 => Ok(ServerCertVerified::assertion()),
            _ => Err(rustls::Error::General(
                "custom certificate verifier rejected the certificate".to_string(),
            )),
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

pub fn rustls_client_config_builder_dangerous_set_certificate_verifier(
    builder: Ptr<RustlsClientConfigBuilder>,
    callback: FnPtr<RustlsVerifyServerCertCallback>,
) -> RustlsResult {
    if callback.is_null() {
        return RustlsResult::NullParameter;
    }
    let cb = *callback;
    builder.with_mut(|b| {
        b.verifier = Some(Arc::new(CallbackVerifier {
            cb,
            provider: b.provider.clone(),
        }));
    });
    RustlsResult::Ok
}

pub fn rustls_client_config_builder_set_server_verifier(
    builder: Ptr<RustlsClientConfigBuilder>,
    verifier: Ptr<RustlsServerCertVerifier>,
) {
    let verifier = verifier.with(|v| v.0.clone());
    builder.with_mut(|b| b.verifier = Some(verifier));
}

pub fn rustls_client_config_builder_build(
    builder: Ptr<RustlsClientConfigBuilder>,
    config_out: Ptr<Ptr<RustlsClientConfig>>,
) -> RustlsResult {
    if config_out.is_null() {
        return RustlsResult::NullParameter;
    }
    let (provider, versions, verifier, alpn_protocols, cert_resolver, key_log) = builder.with(|b| {
        (
            b.provider.clone(),
            b.versions.clone(),
            b.verifier.clone(),
            b.alpn_protocols.clone(),
            b.cert_resolver.clone(),
            b.key_log.clone(),
        )
    });
    let verifier = match verifier {
        Some(v) => v,
        None => return RustlsResult::NoServerCertVerifier,
    };
    let versions = match versions.is_empty() {
        true => rustls::DEFAULT_VERSIONS,
        false => versions.as_slice(),
    };
    let wants_verifier = match ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(versions)
    {
        Ok(config) => config,
        Err(e) => return map_rustls_error(&e),
    };
    let config = wants_verifier
        .dangerous()
        .with_custom_certificate_verifier(verifier);
    let mut config = match cert_resolver {
        Some(r) => config.with_client_cert_resolver(r),
        None => config.with_no_client_auth(),
    };
    config.alpn_protocols = alpn_protocols;
    if let Some(key_log) = key_log {
        config.key_log = key_log;
    }
    config_out.write(Ptr::alloc(RustlsClientConfig(Arc::new(config))));
    RustlsResult::Ok
}

pub fn rustls_client_connection_new(
    config: Ptr<RustlsClientConfig>,
    server_name: Ptr<u8>,
    conn_out: Ptr<Ptr<RustlsConnection>>,
) -> RustlsResult {
    if server_name.is_null() || conn_out.is_null() {
        return RustlsResult::NullParameter;
    }
    let server_name: ServerName<'static> = match server_name.to_rust_string().try_into() {
        Ok(name) => name,
        Err(_) => return RustlsResult::InvalidDnsNameError,
    };
    let config = config.with(|c| c.0.clone());
    match rustls::ClientConnection::new(config, server_name) {
        Ok(conn) => {
            conn_out.write(Ptr::alloc(RustlsConnection {
                conn,
                userdata: AnyPtr::default(),
            }));
            RustlsResult::Ok
        }
        Err(e) => map_rustls_error(&e),
    }
}

pub struct RustlsConnection {
    pub conn: rustls::ClientConnection,
    pub userdata: AnyPtr,
}
impl ByteRepr for RustlsConnection {}

pub fn rustls_connection_set_userdata(conn: Ptr<RustlsConnection>, userdata: AnyPtr) {
    conn.with_mut(|c| c.userdata = userdata.clone());
}

pub type RustlsReadCallback = fn(AnyPtr, Ptr<u8>, u64, Ptr<u64>) -> i32;
pub type RustlsWriteCallback = fn(AnyPtr, Ptr<u8>, u64, Ptr<u64>) -> i32;

struct CallbackReader {
    cb: RustlsReadCallback,
    userdata: AnyPtr,
}

impl Read for CallbackReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let tmp = Ptr::alloc_array(vec![0u8; buf.len()].into_boxed_slice());
        let out_n = Ptr::alloc(0u64);
        let rc = (self.cb)(
            self.userdata.clone(),
            tmp.clone(),
            buf.len() as u64,
            out_n.clone(),
        );
        let result = if rc != 0 {
            Err(std::io::Error::from_raw_os_error(rc))
        } else {
            let n = out_n.read() as usize;
            tmp.with_slice(n, |s| buf[..n].copy_from_slice(s));
            Ok(n)
        };
        tmp.delete_array();
        out_n.delete();
        result
    }
}

struct CallbackWriter {
    cb: RustlsWriteCallback,
    userdata: AnyPtr,
}

impl Write for CallbackWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let tmp = Ptr::alloc_array(buf.to_vec().into_boxed_slice());
        let out_n = Ptr::alloc(0u64);
        let rc = (self.cb)(
            self.userdata.clone(),
            tmp.clone(),
            buf.len() as u64,
            out_n.clone(),
        );
        let result = if rc != 0 {
            Err(std::io::Error::from_raw_os_error(rc))
        } else {
            Ok(out_n.read() as usize)
        };
        tmp.delete_array();
        out_n.delete();
        result
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn rustls_connection_read_tls(
    conn: Ptr<RustlsConnection>,
    callback: FnPtr<RustlsReadCallback>,
    userdata: AnyPtr,
    out_n: Ptr<usize>,
) -> i32 {
    if callback.is_null() || out_n.is_null() {
        return libc::EINVAL;
    }
    let mut reader = CallbackReader {
        cb: *callback,
        userdata,
    };
    match conn.with_mut(|c| c.conn.read_tls(&mut reader)) {
        Ok(n) => {
            out_n.write(n);
            0
        }
        Err(e) => e.raw_os_error().unwrap_or(libc::EIO),
    }
}

pub fn rustls_connection_write_tls(
    conn: Ptr<RustlsConnection>,
    callback: FnPtr<RustlsWriteCallback>,
    userdata: AnyPtr,
    out_n: Ptr<usize>,
) -> i32 {
    if callback.is_null() || out_n.is_null() {
        return libc::EINVAL;
    }
    let mut writer = CallbackWriter {
        cb: *callback,
        userdata,
    };
    match conn.with_mut(|c| c.conn.write_tls(&mut writer)) {
        Ok(n) => {
            out_n.write(n);
            0
        }
        Err(e) => e.raw_os_error().unwrap_or(libc::EIO),
    }
}

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
