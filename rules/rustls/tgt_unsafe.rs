// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> *mut ::rustls_ffi::connection::rustls_connection {
    std::ptr::null_mut()
}
fn t2() -> *const ::rustls_ffi::connection::rustls_connection {
    std::ptr::null()
}
fn t3() -> *const ::rustls_ffi::certificate::rustls_certificate<'static> {
    std::ptr::null()
}
fn t4() -> ::rustls_ffi::rslice::rustls_str<'static> {
    unsafe { std::mem::zeroed() }
}
fn t5() -> ::rustls_ffi::rustls_result {
    ::rustls_ffi::rustls_result::Ok
}
fn t6() -> ::rustls_ffi::rustls_io_result {
    ::rustls_ffi::rustls_io_result(0)
}
fn t7() -> ::rustls_ffi::enums::rustls_tls_version {
    ::rustls_ffi::enums::rustls_tls_version::Unknown
}

unsafe fn f1() -> ::rustls_ffi::rustls_result {
    ::rustls_ffi::rustls_result::Ok
}
unsafe fn f2() -> ::rustls_ffi::rustls_result {
    ::rustls_ffi::rustls_result::NullParameter
}
unsafe fn f3() -> ::rustls_ffi::rustls_result {
    ::rustls_ffi::rustls_result::PlaintextEmpty
}
unsafe fn f4() -> ::rustls_ffi::rustls_result {
    ::rustls_ffi::rustls_result::UnexpectedEof
}

unsafe fn f5() -> i32 {
    ::rustls_ffi::enums::rustls_tls_version::Tlsv1_2 as i32
}
unsafe fn f6() -> i32 {
    ::rustls_ffi::enums::rustls_tls_version::Tlsv1_3 as i32
}

unsafe fn f7(
    a0: *mut ::rustls_ffi::connection::rustls_connection,
    a1: *mut u8,
    a2: usize,
    a3: *mut usize,
) -> u32 {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_read(a0, a1, a2, a3) as u32
}
unsafe fn f8(
    a0: *mut ::rustls_ffi::connection::rustls_connection,
    a1: *const u8,
    a2: usize,
    a3: *mut usize,
) -> u32 {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_write(a0, a1, a2, a3) as u32
}
unsafe fn f9(a0: *mut ::rustls_ffi::connection::rustls_connection) -> u32 {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_process_new_packets(a0) as u32
}
unsafe fn f10(a0: *const ::rustls_ffi::connection::rustls_connection) -> bool {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_wants_read(a0)
}
unsafe fn f11(a0: *const ::rustls_ffi::connection::rustls_connection) -> bool {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_wants_write(a0)
}
unsafe fn f12(a0: *const ::rustls_ffi::connection::rustls_connection) -> bool {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_is_handshaking(a0)
}
unsafe fn f13(a0: *mut ::rustls_ffi::connection::rustls_connection) {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_send_close_notify(a0)
}
unsafe fn f14(a0: *mut ::rustls_ffi::connection::rustls_connection, a1: *mut std::ffi::c_void) {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_set_userdata(a0, a1)
}
unsafe fn f15(
    a0: *const ::rustls_ffi::connection::rustls_connection,
    a1: *mut *const u8,
    a2: *mut usize,
) {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_get_alpn_protocol(a0, a1, a2)
}
unsafe fn f16(a0: *const ::rustls_ffi::connection::rustls_connection) -> u16 {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_get_protocol_version(a0)
}
unsafe fn f17(
    a0: *const ::rustls_ffi::connection::rustls_connection,
) -> ::rustls_ffi::rslice::rustls_str<'static> {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_get_negotiated_ciphersuite_name(
        a0,
    )
}
unsafe fn f18(
    a0: *const ::rustls_ffi::connection::rustls_connection,
) -> ::rustls_ffi::rslice::rustls_str<'static> {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_get_negotiated_key_exchange_group_name(a0)
}
unsafe fn f19(
    a0: *const ::rustls_ffi::connection::rustls_connection,
    a1: usize,
) -> *const ::rustls_ffi::certificate::rustls_certificate<'static> {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_get_peer_certificate(a0, a1)
}
unsafe fn f20(a0: *mut ::rustls_ffi::connection::rustls_connection) {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_free(a0)
}

fn t8() -> *mut ::rustls_ffi::client::rustls_client_config {
    std::ptr::null_mut()
}
fn t9() -> *const ::rustls_ffi::client::rustls_client_config {
    std::ptr::null()
}
fn t10() -> *mut ::rustls_ffi::client::rustls_client_config_builder {
    std::ptr::null_mut()
}
fn t11() -> *const ::rustls_ffi::client::rustls_client_config_builder {
    std::ptr::null()
}
fn t12() -> *mut ::rustls_ffi::certificate::rustls_certified_key {
    std::ptr::null_mut()
}
fn t13() -> *const ::rustls_ffi::certificate::rustls_certified_key {
    std::ptr::null()
}
fn t14() -> *mut ::rustls_ffi::crypto_provider::rustls_crypto_provider {
    std::ptr::null_mut()
}
fn t15() -> *const ::rustls_ffi::crypto_provider::rustls_crypto_provider {
    std::ptr::null()
}
fn t16() -> *mut ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder {
    std::ptr::null_mut()
}
fn t17() -> *const ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder {
    std::ptr::null()
}
fn t18() -> *mut ::rustls_ffi::certificate::rustls_root_cert_store {
    std::ptr::null_mut()
}
fn t19() -> *const ::rustls_ffi::certificate::rustls_root_cert_store {
    std::ptr::null()
}
fn t20() -> *mut ::rustls_ffi::certificate::rustls_root_cert_store_builder {
    std::ptr::null_mut()
}
fn t21() -> *const ::rustls_ffi::certificate::rustls_root_cert_store_builder {
    std::ptr::null()
}
fn t22() -> *mut ::rustls_ffi::verifier::rustls_server_cert_verifier {
    std::ptr::null_mut()
}
fn t23() -> *const ::rustls_ffi::verifier::rustls_server_cert_verifier {
    std::ptr::null()
}
fn t24() -> *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder {
    std::ptr::null_mut()
}
fn t25() -> *const ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder {
    std::ptr::null()
}
fn t26() -> *mut ::rustls_ffi::cipher::rustls_supported_ciphersuite {
    std::ptr::null_mut()
}
fn t27() -> *const ::rustls_ffi::cipher::rustls_supported_ciphersuite {
    std::ptr::null()
}
fn t28() -> ::rustls_ffi::rslice::rustls_slice_bytes<'static> {
    unsafe { std::mem::zeroed() }
}
fn t29() -> ::rustls_ffi::client::rustls_verify_server_cert_params<'static> {
    unsafe { std::mem::zeroed() }
}

unsafe fn f21(
    a0: *mut ::rustls_ffi::client::rustls_client_config_builder,
    a1: *mut *const ::rustls_ffi::client::rustls_client_config,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_build(a0, a1)
        as u32
}
unsafe fn f22(a0: *mut ::rustls_ffi::client::rustls_client_config_builder) {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_free(a0)
}
unsafe fn f23(
    a0: *const ::rustls_ffi::crypto_provider::rustls_crypto_provider,
    a1: *const u16,
    a2: usize,
    a3: *mut *mut ::rustls_ffi::client::rustls_client_config_builder,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_new_custom(
        a0, a1, a2, a3,
    ) as u32
}
unsafe fn f24(
    a0: *mut ::rustls_ffi::client::rustls_client_config_builder,
    a1: *const ::rustls_ffi::rslice::rustls_slice_bytes<'static>,
    a2: usize,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_set_alpn_protocols(a0, a1, a2) as u32
}
unsafe fn f25(
    a0: *mut ::rustls_ffi::client::rustls_client_config_builder,
    a1: *const *const ::rustls_ffi::certificate::rustls_certified_key,
    a2: usize,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_set_certified_key(a0, a1, a2) as u32
}
unsafe fn f26(
    a0: *mut ::rustls_ffi::client::rustls_client_config_builder,
    a1: *const ::rustls_ffi::verifier::rustls_server_cert_verifier,
) {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_set_server_verifier(a0, a1)
}
unsafe fn f27(a0: *const ::rustls_ffi::client::rustls_client_config) {
    ::rustls_ffi::client::rustls_client_config::rustls_client_config_free(a0)
}
unsafe fn f28(
    a0: *const ::rustls_ffi::client::rustls_client_config,
    a1: *const u8,
    a2: *mut *mut ::rustls_ffi::connection::rustls_connection,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config::rustls_client_connection_new(
        a0,
        a1 as *const ::std::ffi::c_char,
        a2,
    ) as u32
}

unsafe fn f29(
    a0: *const ::rustls_ffi::certificate::rustls_certificate<'static>,
    a1: *mut *const u8,
    a2: *mut usize,
) -> u32 {
    ::rustls_ffi::certificate::rustls_certificate_get_der(a0, a1, a2) as u32
}
unsafe fn f30(
    a0: *const u8,
    a1: usize,
    a2: *const u8,
    a3: usize,
    a4: *mut *const ::rustls_ffi::certificate::rustls_certified_key,
) -> u32 {
    ::rustls_ffi::certificate::rustls_certified_key::rustls_certified_key_build(a0, a1, a2, a3, a4)
        as u32
}
unsafe fn f31(a0: *const ::rustls_ffi::certificate::rustls_certified_key) {
    ::rustls_ffi::certificate::rustls_certified_key::rustls_certified_key_free(a0)
}
unsafe fn f32(a0: *const ::rustls_ffi::certificate::rustls_certified_key) -> u32 {
    ::rustls_ffi::certificate::rustls_certified_key::rustls_certified_key_keys_match(a0) as u32
}
unsafe fn f33(
    a0: *mut ::rustls_ffi::certificate::rustls_root_cert_store_builder,
    a1: *const u8,
    a2: usize,
    a3: bool,
) -> u32 {
    ::rustls_ffi::certificate::rustls_root_cert_store_builder::rustls_root_cert_store_builder_add_pem(a0, a1, a2, a3) as u32
}
unsafe fn f34(
    a0: *mut ::rustls_ffi::certificate::rustls_root_cert_store_builder,
    a1: *mut *const ::rustls_ffi::certificate::rustls_root_cert_store,
) -> u32 {
    ::rustls_ffi::certificate::rustls_root_cert_store_builder::rustls_root_cert_store_builder_build(
        a0, a1,
    ) as u32
}
unsafe fn f35(a0: *mut ::rustls_ffi::certificate::rustls_root_cert_store_builder) {
    ::rustls_ffi::certificate::rustls_root_cert_store_builder::rustls_root_cert_store_builder_free(
        a0,
    )
}
unsafe fn f36(
    a0: *mut ::rustls_ffi::certificate::rustls_root_cert_store_builder,
    a1: *const u8,
    a2: bool,
) -> u32 {
    ::rustls_ffi::certificate::rustls_root_cert_store_builder::rustls_root_cert_store_builder_load_roots_from_file(a0, a1 as *const ::std::ffi::c_char, a2) as u32
}
unsafe fn f37() -> *mut ::rustls_ffi::certificate::rustls_root_cert_store_builder {
    ::rustls_ffi::certificate::rustls_root_cert_store_builder::rustls_root_cert_store_builder_new()
}
unsafe fn f38(a0: *const ::rustls_ffi::certificate::rustls_root_cert_store) {
    ::rustls_ffi::certificate::rustls_root_cert_store::rustls_root_cert_store_free(a0)
}

unsafe fn f39(
    a0: *mut ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder,
    a1: *mut *const ::rustls_ffi::crypto_provider::rustls_crypto_provider,
) -> u32 {
    ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder_build(a0, a1) as u32
}
unsafe fn f40(a0: *mut ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder) {
    ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder_free(a0)
}
unsafe fn f41(a0: *mut *mut ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder) -> u32 {
    ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder_new_from_default(a0) as u32
}
unsafe fn f42(
    a0: *mut ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder,
    a1: *const *const ::rustls_ffi::cipher::rustls_supported_ciphersuite,
    a2: usize,
) -> u32 {
    ::rustls_ffi::crypto_provider::rustls_crypto_provider_builder_set_cipher_suites(a0, a1, a2)
        as u32
}
unsafe fn f43(a0: *const ::rustls_ffi::crypto_provider::rustls_crypto_provider) {
    ::rustls_ffi::crypto_provider::rustls_crypto_provider_free(a0)
}
unsafe fn f44(a0: usize) -> *const ::rustls_ffi::cipher::rustls_supported_ciphersuite {
    ::rustls_ffi::crypto_provider::rustls_default_crypto_provider_ciphersuites_get(a0)
}
unsafe fn f45() -> usize {
    ::rustls_ffi::crypto_provider::rustls_default_crypto_provider_ciphersuites_len()
}
unsafe fn f46(a0: *mut u8, a1: usize) -> u32 {
    ::rustls_ffi::crypto_provider::rustls_default_crypto_provider_random(a0, a1) as u32
}

unsafe fn f47(a0: *mut *mut ::rustls_ffi::verifier::rustls_server_cert_verifier) -> u32 {
    ::rustls_ffi::verifier::rustls_server_cert_verifier::rustls_platform_server_cert_verifier(a0)
        as u32
}
unsafe fn f48(a0: *mut ::rustls_ffi::verifier::rustls_server_cert_verifier) {
    ::rustls_ffi::verifier::rustls_server_cert_verifier::rustls_server_cert_verifier_free(a0)
}
unsafe fn f49(
    a0: *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder,
    a1: *const u8,
    a2: usize,
) -> u32 {
    unsafe extern "C" {
        fn rustls_web_pki_server_cert_verifier_builder_add_crl(
            builder: *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder,
            crl_pem: *const u8,
            crl_pem_len: usize,
        ) -> ::rustls_ffi::rustls_result;
    }
    rustls_web_pki_server_cert_verifier_builder_add_crl(a0, a1, a2) as u32
}
unsafe fn f50(
    a0: *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder,
    a1: *mut *mut ::rustls_ffi::verifier::rustls_server_cert_verifier,
) -> u32 {
    unsafe extern "C" {
        fn rustls_web_pki_server_cert_verifier_builder_build(
            builder: *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder,
            verifier_out: *mut *mut ::rustls_ffi::verifier::rustls_server_cert_verifier,
        ) -> ::rustls_ffi::rustls_result;
    }
    rustls_web_pki_server_cert_verifier_builder_build(a0, a1) as u32
}
unsafe fn f51(a0: *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder) {
    unsafe extern "C" {
        fn rustls_web_pki_server_cert_verifier_builder_free(
            builder: *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder,
        );
    }
    rustls_web_pki_server_cert_verifier_builder_free(a0)
}
unsafe fn f52(
    a0: *const ::rustls_ffi::certificate::rustls_root_cert_store,
) -> *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder {
    unsafe extern "C" {
        fn rustls_web_pki_server_cert_verifier_builder_new(
            store: *const ::rustls_ffi::certificate::rustls_root_cert_store,
        ) -> *mut ::rustls_ffi::verifier::rustls_web_pki_server_cert_verifier_builder;
    }
    rustls_web_pki_server_cert_verifier_builder_new(a0)
}

unsafe fn f53(a0: *const ::rustls_ffi::cipher::rustls_supported_ciphersuite) -> u16 {
    ::rustls_ffi::cipher::rustls_supported_ciphersuite::rustls_supported_ciphersuite_get_suite(a0)
}
unsafe fn f54(
    a0: *const ::rustls_ffi::cipher::rustls_supported_ciphersuite,
) -> ::rustls_ffi::enums::rustls_tls_version {
    ::rustls_ffi::cipher::rustls_supported_ciphersuite_protocol_version(a0)
}

unsafe fn f55() -> ::rustls_ffi::rslice::rustls_str<'static> {
    ::rustls_ffi::version::rustls_version()
}

unsafe fn f56(a0: u32, a1: *mut u8, a2: usize, a3: *mut usize) {
    ::rustls_ffi::rustls_result::rustls_error(a0, a1 as *mut ::std::ffi::c_char, a2, a3)
}
unsafe fn f57(a0: u32) -> bool {
    ::rustls_ffi::rustls_result::rustls_result_is_cert_error(a0)
}

unsafe fn f58(
    a0: *mut ::rustls_ffi::connection::rustls_connection,
    a1: unsafe fn(*mut ::libc::c_void, *mut u8, usize, *mut usize) -> i32,
    a2: *mut ::libc::c_void,
    a3: *mut usize,
) -> i32 {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_read_tls(
        a0,
        std::mem::transmute::<*const (), ::rustls_ffi::io::rustls_read_callback>(a1 as *const ()),
        a2,
        a3,
    )
    .0
}
unsafe fn f59(
    a0: *mut ::rustls_ffi::connection::rustls_connection,
    a1: unsafe fn(*mut ::libc::c_void, *const u8, usize, *mut usize) -> i32,
    a2: *mut ::libc::c_void,
    a3: *mut usize,
) -> i32 {
    ::rustls_ffi::connection::rustls_connection::rustls_connection_write_tls(
        a0,
        std::mem::transmute::<*const (), ::rustls_ffi::io::rustls_write_callback>(a1 as *const ()),
        a2,
        a3,
    )
    .0
}
unsafe fn f60(
    a0: *mut ::rustls_ffi::client::rustls_client_config_builder,
    a1: unsafe fn(::rustls_ffi::rslice::rustls_str<'static>, *const u8, usize, *const u8, usize),
    a2: Option<unsafe fn(::rustls_ffi::rslice::rustls_str<'static>) -> i32>,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_set_key_log(
        a0,
        std::mem::transmute::<*const (), ::rustls_ffi::keylog::rustls_keylog_log_callback>(
            a1 as *const (),
        ),
        std::mem::transmute::<
            Option<unsafe fn(::rustls_ffi::rslice::rustls_str<'static>) -> i32>,
            ::rustls_ffi::keylog::rustls_keylog_will_log_callback,
        >(a2),
    ) as u32
}
unsafe fn f61(
    a0: *mut ::rustls_ffi::client::rustls_client_config_builder,
    a1: unsafe fn(
        *mut ::libc::c_void,
        *const ::rustls_ffi::client::rustls_verify_server_cert_params<'static>,
    ) -> u32,
) -> u32 {
    ::rustls_ffi::client::rustls_client_config_builder::rustls_client_config_builder_dangerous_set_certificate_verifier(
        a0,
        std::mem::transmute::<*const (), ::rustls_ffi::client::rustls_verify_server_cert_callback>(
            a1 as *const (),
        ),
    ) as u32
}
