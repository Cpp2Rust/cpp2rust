// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t4() -> libcc2rs::RustlsStr {
    Default::default()
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

fn t26() -> Ptr<libcc2rs::RustlsSupportedCiphersuite> {
    Ptr::null()
}

fn t27() -> Ptr<libcc2rs::RustlsSupportedCiphersuite> {
    Ptr::null()
}

fn f39(
    a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsCryptoProvider>>,
) -> ::rustls_ffi::rustls_result {
    libcc2rs::rustls_crypto_provider_builder_build(a0, a1);
    ::rustls_ffi::rustls_result::Ok
}

fn f40(a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>) {
    a0.delete()
}

fn f41(a0: Ptr<Ptr<libcc2rs::RustlsCryptoProviderBuilder>>) -> ::rustls_ffi::rustls_result {
    libcc2rs::rustls_crypto_provider_builder_new_from_default(a0);
    ::rustls_ffi::rustls_result::Ok
}

fn f42(
    a0: Ptr<libcc2rs::RustlsCryptoProviderBuilder>,
    a1: Ptr<Ptr<libcc2rs::RustlsSupportedCiphersuite>>,
    a2: usize,
) -> ::rustls_ffi::rustls_result {
    libcc2rs::rustls_crypto_provider_builder_set_cipher_suites(a0, a1, a2);
    ::rustls_ffi::rustls_result::Ok
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

fn f46(a0: Ptr<u8>, a1: usize) -> ::rustls_ffi::rustls_result {
    match libcc2rs::rustls_default_crypto_provider_random(a0.clone(), a1) {
        true => ::rustls_ffi::rustls_result::Ok,
        false => ::rustls_ffi::rustls_result::GetRandomFailed,
    }
}

fn f53(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_get_suite(a0.clone())
}

fn f54(a0: Ptr<libcc2rs::RustlsSupportedCiphersuite>) -> u16 {
    libcc2rs::rustls_supported_ciphersuite_protocol_version(a0.clone())
}
