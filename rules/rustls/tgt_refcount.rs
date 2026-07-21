// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> Ptr<libcc2rs::RustlsConnection> {
    Ptr::null()
}

fn t2() -> Ptr<libcc2rs::RustlsConnection> {
    Ptr::null()
}

fn t3() -> Ptr<libcc2rs::RustlsCertificate> {
    Ptr::null()
}

fn t4() -> libcc2rs::RustlsStr {
    Default::default()
}

fn t5() -> libcc2rs::RustlsResult {
    libcc2rs::RustlsResult::Ok
}

fn t6() -> i32 {
    0
}

fn t7() -> u16 {
    0
}

fn f1() -> libcc2rs::RustlsResult {
    libcc2rs::RustlsResult::Ok
}

fn f2() -> libcc2rs::RustlsResult {
    libcc2rs::RustlsResult::NullParameter
}

fn f3() -> libcc2rs::RustlsResult {
    libcc2rs::RustlsResult::PlaintextEmpty
}

fn f4() -> libcc2rs::RustlsResult {
    libcc2rs::RustlsResult::UnexpectedEof
}

fn f5() -> u16 {
    0x0303
}

fn f6() -> u16 {
    0x0304
}

fn f55() -> libcc2rs::RustlsStr {
    libcc2rs::rustls_version()
}

fn f56(a0: u32, a1: Ptr<u8>, a2: usize, a3: Ptr<usize>) {
    libcc2rs::rustls_error(a0, a1, a2, a3)
}

fn t8() -> Ptr<libcc2rs::RustlsClientConfig> {
    Ptr::null()
}

fn t9() -> Ptr<libcc2rs::RustlsClientConfig> {
    Ptr::null()
}

fn t10() -> Ptr<libcc2rs::RustlsClientConfigBuilder> {
    Ptr::null()
}

fn t11() -> Ptr<libcc2rs::RustlsClientConfigBuilder> {
    Ptr::null()
}

fn t12() -> Ptr<libcc2rs::RustlsCertifiedKey> {
    Ptr::null()
}

fn t13() -> Ptr<libcc2rs::RustlsCertifiedKey> {
    Ptr::null()
}

fn t14() -> Ptr<libcc2rs::RustlsCryptoProvider> {
    Ptr::null()
}

fn t15() -> Ptr<libcc2rs::RustlsCryptoProvider> {
    Ptr::null()
}

fn t16() -> Ptr<libcc2rs::RustlsCryptoProviderBuilder> {
    Ptr::null()
}

fn t17() -> Ptr<libcc2rs::RustlsCryptoProviderBuilder> {
    Ptr::null()
}

fn t28() -> libcc2rs::RustlsSliceBytes {
    Default::default()
}

fn t29() -> libcc2rs::RustlsVerifyServerCertParams {
    Default::default()
}

fn t18() -> Ptr<libcc2rs::RustlsRootCertStore> {
    Ptr::null()
}

fn t19() -> Ptr<libcc2rs::RustlsRootCertStore> {
    Ptr::null()
}

fn t20() -> Ptr<libcc2rs::RustlsRootCertStoreBuilder> {
    Ptr::null()
}

fn t21() -> Ptr<libcc2rs::RustlsRootCertStoreBuilder> {
    Ptr::null()
}

fn t22() -> Ptr<libcc2rs::RustlsServerCertVerifier> {
    Ptr::null()
}

fn t23() -> Ptr<libcc2rs::RustlsServerCertVerifier> {
    Ptr::null()
}

fn t24() -> Ptr<libcc2rs::RustlsWebPkiServerCertVerifierBuilder> {
    Ptr::null()
}

fn t25() -> Ptr<libcc2rs::RustlsWebPkiServerCertVerifierBuilder> {
    Ptr::null()
}

fn t26() -> Ptr<libcc2rs::RustlsSupportedCiphersuite> {
    Ptr::null()
}

fn t27() -> Ptr<libcc2rs::RustlsSupportedCiphersuite> {
    Ptr::null()
}

fn f7(
    a0: Ptr<libcc2rs::RustlsConnection>,
    a1: Ptr<u8>,
    a2: usize,
    a3: Ptr<usize>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_connection_read(a0.clone(), a1.clone(), a2, a3.clone())
}

fn f8(
    a0: Ptr<libcc2rs::RustlsConnection>,
    a1: Ptr<u8>,
    a2: usize,
    a3: Ptr<usize>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_connection_write(a0.clone(), a1.clone(), a2, a3.clone())
}

fn f9(a0: Ptr<libcc2rs::RustlsConnection>) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_connection_process_new_packets(a0.clone())
}

fn f10(a0: Ptr<libcc2rs::RustlsConnection>) -> bool {
    libcc2rs::rustls_connection_wants_read(a0.clone())
}

fn f11(a0: Ptr<libcc2rs::RustlsConnection>) -> bool {
    libcc2rs::rustls_connection_wants_write(a0.clone())
}

fn f12(a0: Ptr<libcc2rs::RustlsConnection>) -> bool {
    libcc2rs::rustls_connection_is_handshaking(a0.clone())
}

fn f13(a0: Ptr<libcc2rs::RustlsConnection>) {
    libcc2rs::rustls_connection_send_close_notify(a0.clone())
}

fn f14(a0: Ptr<libcc2rs::RustlsConnection>, a1: AnyPtr) {
    libcc2rs::rustls_connection_set_userdata(a0.clone(), a1.clone())
}

fn f15(
    a0: Ptr<libcc2rs::RustlsConnection>,
    a1: Ptr<Ptr<u8>>,
    a2: Ptr<usize>,
) {
    libcc2rs::rustls_connection_get_alpn_protocol(a0.clone(), a1.clone(), a2.clone())
}

fn f16(a0: Ptr<libcc2rs::RustlsConnection>) -> u16 {
    libcc2rs::rustls_connection_get_protocol_version(a0.clone())
}

fn f17(a0: Ptr<libcc2rs::RustlsConnection>) -> libcc2rs::RustlsStr {
    libcc2rs::rustls_connection_get_negotiated_ciphersuite_name(a0.clone())
}

fn f18(a0: Ptr<libcc2rs::RustlsConnection>) -> libcc2rs::RustlsStr {
    libcc2rs::rustls_connection_get_negotiated_key_exchange_group_name(a0.clone())
}

fn f19(
    a0: Ptr<libcc2rs::RustlsConnection>,
    a1: usize,
) -> Ptr<libcc2rs::RustlsCertificate> {
    libcc2rs::rustls_connection_get_peer_certificate(a0.clone(), a1)
}

fn f20(a0: Ptr<libcc2rs::RustlsConnection>) {
    a0.delete()
}

fn f21(
    a0: Ptr<libcc2rs::RustlsClientConfigBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsClientConfig>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_config_builder_build(a0.clone(), a1.clone())
}

fn f22(a0: Ptr<libcc2rs::RustlsClientConfigBuilder>) {
    a0.delete()
}

fn f23(
    a0: Ptr<libcc2rs::RustlsCryptoProvider>,
    a1: Ptr<u16>,
    a2: usize,
    a3: Ptr<Ptr<libcc2rs::RustlsClientConfigBuilder>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_config_builder_new_custom(a0.clone(), a1.clone(), a2, a3.clone())
}

fn f24(
    a0: Ptr<libcc2rs::RustlsClientConfigBuilder>,
    a1: Ptr<libcc2rs::RustlsSliceBytes>,
    a2: usize,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_config_builder_set_alpn_protocols(a0.clone(), a1.clone(), a2)
}

fn f25(
    a0: Ptr<libcc2rs::RustlsClientConfigBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsCertifiedKey>>,
    a2: usize,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_config_builder_set_certified_key(a0.clone(), a1.clone(), a2)
}

fn f26(
    a0: Ptr<libcc2rs::RustlsClientConfigBuilder>,
    a1: Ptr<libcc2rs::RustlsServerCertVerifier>,
) {
    libcc2rs::rustls_client_config_builder_set_server_verifier(a0.clone(), a1.clone())
}

fn f27(a0: Ptr<libcc2rs::RustlsClientConfig>) {
    a0.delete()
}

fn f28(
    a0: Ptr<libcc2rs::RustlsClientConfig>,
    a1: Ptr<u8>,
    a2: Ptr<Ptr<libcc2rs::RustlsConnection>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_connection_new(a0.clone(), a1.clone(), a2.clone())
}

fn f29(
    a0: Ptr<libcc2rs::RustlsCertificate>,
    a1: Ptr<Ptr<u8>>,
    a2: Ptr<usize>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_certificate_get_der(a0.clone(), a1.clone(), a2.clone())
}

fn f30(
    a0: Ptr<u8>,
    a1: usize,
    a2: Ptr<u8>,
    a3: usize,
    a4: Ptr<Ptr<libcc2rs::RustlsCertifiedKey>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_certified_key_build(a0.clone(), a1, a2.clone(), a3, a4.clone())
}

fn f31(a0: Ptr<libcc2rs::RustlsCertifiedKey>) {
    a0.delete()
}

fn f32(a0: Ptr<libcc2rs::RustlsCertifiedKey>) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_certified_key_keys_match(a0.clone())
}

fn f33(
    a0: Ptr<libcc2rs::RustlsRootCertStoreBuilder>,
    a1: Ptr<u8>,
    a2: usize,
    a3: bool,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_root_cert_store_builder_add_pem(a0.clone(), a1.clone(), a2, a3)
}

fn f34(
    a0: Ptr<libcc2rs::RustlsRootCertStoreBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsRootCertStore>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_root_cert_store_builder_build(a0.clone(), a1.clone())
}

fn f35(a0: Ptr<libcc2rs::RustlsRootCertStoreBuilder>) {
    a0.delete()
}

fn f36(
    a0: Ptr<libcc2rs::RustlsRootCertStoreBuilder>,
    a1: Ptr<u8>,
    a2: bool,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_root_cert_store_builder_load_roots_from_file(a0.clone(), a1.clone(), a2)
}

fn f37() -> Ptr<libcc2rs::RustlsRootCertStoreBuilder> {
    libcc2rs::rustls_root_cert_store_builder_new()
}

fn f38(a0: Ptr<libcc2rs::RustlsRootCertStore>) {
    a0.delete()
}

fn f39(
    a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsCryptoProvider>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_crypto_provider_builder_build(a0.clone(), a1.clone())
}

fn f40(a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>) {
    a0.delete()
}

fn f41(a0: Ptr<Ptr<libcc2rs::RustlsCryptoProviderBuilder>>) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_crypto_provider_builder_new_from_default(a0.clone())
}

fn f42(
    a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsSupportedCiphersuite>>,
    a2: usize,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_crypto_provider_builder_set_cipher_suites(a0.clone(), a1.clone(), a2)
}

fn f43(a0: Ptr<libcc2rs::RustlsCryptoProvider>) {
    a0.delete()
}

fn f44(a0: usize) -> Ptr<libcc2rs::RustlsSupportedCiphersuite> {
    libcc2rs::rustls_default_crypto_provider_ciphersuites_get(a0)
}

fn f45() -> usize {
    libcc2rs::rustls_default_crypto_provider_ciphersuites_len()
}

fn f46(a0: Ptr<u8>, a1: usize) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_default_crypto_provider_random(a0.clone(), a1)
}

fn f47(a0: Ptr<Ptr<libcc2rs::RustlsServerCertVerifier>>) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_platform_server_cert_verifier(a0.clone())
}

fn f48(a0: Ptr<libcc2rs::RustlsServerCertVerifier>) {
    a0.delete()
}

fn f49(
    a0: Ptr<libcc2rs::RustlsWebPkiServerCertVerifierBuilder>,
    a1: Ptr<u8>,
    a2: usize,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_web_pki_server_cert_verifier_builder_add_crl(a0.clone(), a1.clone(), a2)
}

fn f50(
    a0: Ptr<libcc2rs::RustlsWebPkiServerCertVerifierBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsServerCertVerifier>>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_web_pki_server_cert_verifier_builder_build(a0.clone(), a1.clone())
}

fn f51(a0: Ptr<libcc2rs::RustlsWebPkiServerCertVerifierBuilder>) {
    a0.delete()
}

fn f52(
    a0: Ptr<libcc2rs::RustlsRootCertStore>,
) -> Ptr<libcc2rs::RustlsWebPkiServerCertVerifierBuilder> {
    libcc2rs::rustls_web_pki_server_cert_verifier_builder_new(a0.clone())
}

fn f58(
    a0: Ptr<libcc2rs::RustlsConnection>,
    a1: FnPtr<libcc2rs::RustlsReadCallback>,
    a2: AnyPtr,
    a3: Ptr<usize>,
) -> i32 {
    libcc2rs::rustls_connection_read_tls(a0.clone(), a1.clone(), a2.clone(), a3.clone())
}

fn f59(
    a0: Ptr<libcc2rs::RustlsConnection>,
    a1: FnPtr<libcc2rs::RustlsWriteCallback>,
    a2: AnyPtr,
    a3: Ptr<usize>,
) -> i32 {
    libcc2rs::rustls_connection_write_tls(a0.clone(), a1.clone(), a2.clone(), a3.clone())
}

fn f61(
    a0: Ptr<libcc2rs::RustlsClientConfigBuilder>,
    a1: FnPtr<libcc2rs::RustlsVerifyServerCertCallback>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_config_builder_dangerous_set_certificate_verifier(a0.clone(), a1.clone())
}

fn f60(
    a0: Ptr<libcc2rs::RustlsClientConfigBuilder>,
    a1: FnPtr<libcc2rs::RustlsKeylogLogCallback>,
    a2: FnPtr<libcc2rs::RustlsKeylogWillLogCallback>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_client_config_builder_set_key_log(a0.clone(), a1.clone(), a2.clone())
}

fn f53(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_get_suite(a0.clone())
}

fn f54(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_protocol_version(a0.clone())
}
