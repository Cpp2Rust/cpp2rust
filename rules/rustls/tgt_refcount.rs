// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t3() -> Ptr<libcc2rs::RustlsCertificate> {
    Ptr::null()
}

fn t4() -> libcc2rs::RustlsStr {
    Default::default()
}

fn t5() -> u32 {
    0
}

fn t6() -> i32 {
    0
}

fn t7() -> u16 {
    0
}

fn f1() -> u32 {
    libcc2rs::RUSTLS_RESULT_OK
}

fn f2() -> u32 {
    libcc2rs::RUSTLS_RESULT_NULL_PARAMETER
}

fn f3() -> u32 {
    libcc2rs::RUSTLS_RESULT_PLAINTEXT_EMPTY
}

fn f4() -> u32 {
    libcc2rs::RUSTLS_RESULT_UNEXPECTED_EOF
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

fn f29(
    a0: Ptr<libcc2rs::RustlsCertificate>,
    a1: Ptr<Ptr<u8>>,
    a2: Ptr<usize>,
) -> u32 {
    libcc2rs::rustls_certificate_get_der(a0.clone(), a1.clone(), a2.clone())
}

fn f39(
    a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsCryptoProvider>>,
) -> u32 {
    libcc2rs::rustls_crypto_provider_builder_build(a0.clone(), a1.clone())
}

fn f40(a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>) {
    a0.delete()
}

fn f41(a0: Ptr<Ptr<libcc2rs::RustlsCryptoProviderBuilder>>) -> u32 {
    libcc2rs::rustls_crypto_provider_builder_new_from_default(a0.clone())
}

fn f42(
    a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsSupportedCiphersuite>>,
    a2: usize,
) -> u32 {
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

fn f46(a0: Ptr<u8>, a1: usize) -> u32 {
    libcc2rs::rustls_default_crypto_provider_random(a0.clone(), a1)
}

fn f53(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_get_suite(a0.clone())
}

fn f54(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_protocol_version(a0.clone())
}
