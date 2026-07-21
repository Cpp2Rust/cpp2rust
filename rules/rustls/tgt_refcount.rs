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

fn f29(
    a0: Ptr<libcc2rs::RustlsCertificate>,
    a1: Ptr<Ptr<u8>>,
    a2: Ptr<usize>,
) -> libcc2rs::RustlsResult {
    libcc2rs::rustls_certificate_get_der(a0.clone(), a1.clone(), a2.clone())
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

fn f53(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_get_suite(a0.clone())
}

fn f54(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_protocol_version(a0.clone())
}
