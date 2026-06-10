// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rustls.h"

using t1 = rustls_connection *;
using t2 = const rustls_connection *;
using t3 = const rustls_certificate *;
using t4 = rustls_str;
using t5 = rustls_result;
using t6 = rustls_io_result;
using t7 = rustls_tls_version;

rustls_result f1() { return RUSTLS_RESULT_OK; }
rustls_result f2() { return RUSTLS_RESULT_NULL_PARAMETER; }
rustls_result f3() { return RUSTLS_RESULT_PLAINTEXT_EMPTY; }
rustls_result f4() { return RUSTLS_RESULT_UNEXPECTED_EOF; }

rustls_tls_version f5() { return RUSTLS_TLS_VERSION_TLSV1_2; }
rustls_tls_version f6() { return RUSTLS_TLS_VERSION_TLSV1_3; }

rustls_result f7(rustls_connection *conn, uint8_t *buf, size_t count,
                 size_t *out_n) {
  return rustls_connection_read(conn, buf, count, out_n);
}
rustls_result f8(rustls_connection *conn, const uint8_t *buf, size_t count,
                 size_t *out_n) {
  return rustls_connection_write(conn, buf, count, out_n);
}
rustls_result f9(rustls_connection *conn) {
  return rustls_connection_process_new_packets(conn);
}
bool f10(const rustls_connection *conn) {
  return rustls_connection_wants_read(conn);
}
bool f11(const rustls_connection *conn) {
  return rustls_connection_wants_write(conn);
}
bool f12(const rustls_connection *conn) {
  return rustls_connection_is_handshaking(conn);
}
void f13(rustls_connection *conn) {
  return rustls_connection_send_close_notify(conn);
}
void f14(rustls_connection *conn, void *userdata) {
  return rustls_connection_set_userdata(conn, userdata);
}
void f15(const rustls_connection *conn, const uint8_t **protocol_out,
         size_t *protocol_out_len) {
  return rustls_connection_get_alpn_protocol(conn, protocol_out,
                                             protocol_out_len);
}
uint16_t f16(const rustls_connection *conn) {
  return rustls_connection_get_protocol_version(conn);
}
rustls_str f17(const rustls_connection *conn) {
  return rustls_connection_get_negotiated_ciphersuite_name(conn);
}
rustls_str f18(const rustls_connection *conn) {
  return rustls_connection_get_negotiated_key_exchange_group_name(conn);
}
const rustls_certificate *f19(const rustls_connection *conn, size_t i) {
  return rustls_connection_get_peer_certificate(conn, i);
}
void f20(rustls_connection *conn) { return rustls_connection_free(conn); }

using t8 = rustls_client_config *;
using t9 = const rustls_client_config *;
using t10 = rustls_client_config_builder *;
using t11 = const rustls_client_config_builder *;
using t12 = rustls_certified_key *;
using t13 = const rustls_certified_key *;
using t14 = rustls_crypto_provider *;
using t15 = const rustls_crypto_provider *;
using t16 = rustls_crypto_provider_builder *;
using t17 = const rustls_crypto_provider_builder *;
using t18 = rustls_root_cert_store *;
using t19 = const rustls_root_cert_store *;
using t20 = rustls_root_cert_store_builder *;
using t21 = const rustls_root_cert_store_builder *;
using t22 = rustls_server_cert_verifier *;
using t23 = const rustls_server_cert_verifier *;
using t24 = rustls_web_pki_server_cert_verifier_builder *;
using t25 = const rustls_web_pki_server_cert_verifier_builder *;
using t26 = rustls_supported_ciphersuite *;
using t27 = const rustls_supported_ciphersuite *;
using t28 = rustls_slice_bytes;
using t29 = rustls_verify_server_cert_params;

rustls_result f21(rustls_client_config_builder *builder,
                  const rustls_client_config **config_out) {
  return rustls_client_config_builder_build(builder, config_out);
}
void f22(rustls_client_config_builder *config) {
  return rustls_client_config_builder_free(config);
}
rustls_result f23(const rustls_crypto_provider *provider,
                  const uint16_t *tls_versions, size_t tls_versions_len,
                  rustls_client_config_builder **builder_out) {
  return rustls_client_config_builder_new_custom(provider, tls_versions,
                                                 tls_versions_len, builder_out);
}
rustls_result f24(rustls_client_config_builder *builder,
                  const rustls_slice_bytes *protocols, size_t len) {
  return rustls_client_config_builder_set_alpn_protocols(builder, protocols,
                                                         len);
}
rustls_result f25(rustls_client_config_builder *builder,
                  const rustls_certified_key *const *certified_keys,
                  size_t certified_keys_len) {
  return rustls_client_config_builder_set_certified_key(builder, certified_keys,
                                                        certified_keys_len);
}
void f26(rustls_client_config_builder *builder,
         const rustls_server_cert_verifier *verifier) {
  return rustls_client_config_builder_set_server_verifier(builder, verifier);
}
void f27(const rustls_client_config *config) {
  return rustls_client_config_free(config);
}
rustls_result f28(const rustls_client_config *config, const char *server_name,
                  rustls_connection **conn_out) {
  return rustls_client_connection_new(config, server_name, conn_out);
}

rustls_result f29(const rustls_certificate *cert, const uint8_t **out_der_data,
                  size_t *out_der_len) {
  return rustls_certificate_get_der(cert, out_der_data, out_der_len);
}
rustls_result f30(const uint8_t *cert_chain, size_t cert_chain_len,
                  const uint8_t *private_key, size_t private_key_len,
                  const rustls_certified_key **certified_key_out) {
  return rustls_certified_key_build(cert_chain, cert_chain_len, private_key,
                                    private_key_len, certified_key_out);
}
void f31(const rustls_certified_key *key) {
  return rustls_certified_key_free(key);
}
rustls_result f32(const rustls_certified_key *key) {
  return rustls_certified_key_keys_match(key);
}
rustls_result f33(rustls_root_cert_store_builder *builder, const uint8_t *pem,
                  size_t pem_len, bool strict) {
  return rustls_root_cert_store_builder_add_pem(builder, pem, pem_len, strict);
}
rustls_result f34(rustls_root_cert_store_builder *builder,
                  const rustls_root_cert_store **root_cert_store_out) {
  return rustls_root_cert_store_builder_build(builder, root_cert_store_out);
}
void f35(rustls_root_cert_store_builder *builder) {
  return rustls_root_cert_store_builder_free(builder);
}
rustls_result f36(rustls_root_cert_store_builder *builder, const char *filename,
                  bool strict) {
  return rustls_root_cert_store_builder_load_roots_from_file(builder, filename,
                                                             strict);
}
rustls_root_cert_store_builder *f37() {
  return rustls_root_cert_store_builder_new();
}
void f38(const rustls_root_cert_store *store) {
  return rustls_root_cert_store_free(store);
}

rustls_result f39(rustls_crypto_provider_builder *builder,
                  const rustls_crypto_provider **provider_out) {
  return rustls_crypto_provider_builder_build(builder, provider_out);
}
void f40(rustls_crypto_provider_builder *builder) {
  return rustls_crypto_provider_builder_free(builder);
}
rustls_result f41(rustls_crypto_provider_builder **builder_out) {
  return rustls_crypto_provider_builder_new_from_default(builder_out);
}
rustls_result f42(rustls_crypto_provider_builder *builder,
                  const rustls_supported_ciphersuite *const *cipher_suites,
                  size_t cipher_suites_len) {
  return rustls_crypto_provider_builder_set_cipher_suites(builder, cipher_suites,
                                                          cipher_suites_len);
}
void f43(const rustls_crypto_provider *provider) {
  return rustls_crypto_provider_free(provider);
}
const rustls_supported_ciphersuite *f44(size_t index) {
  return rustls_default_crypto_provider_ciphersuites_get(index);
}
size_t f45() { return rustls_default_crypto_provider_ciphersuites_len(); }
rustls_result f46(uint8_t *buff, size_t len) {
  return rustls_default_crypto_provider_random(buff, len);
}

rustls_result f47(rustls_server_cert_verifier **verifier_out) {
  return rustls_platform_server_cert_verifier(verifier_out);
}
void f48(rustls_server_cert_verifier *verifier) {
  return rustls_server_cert_verifier_free(verifier);
}
rustls_result f49(rustls_web_pki_server_cert_verifier_builder *builder,
                  const uint8_t *crl_pem, size_t crl_pem_len) {
  return rustls_web_pki_server_cert_verifier_builder_add_crl(builder, crl_pem,
                                                             crl_pem_len);
}
rustls_result f50(rustls_web_pki_server_cert_verifier_builder *builder,
                  rustls_server_cert_verifier **verifier_out) {
  return rustls_web_pki_server_cert_verifier_builder_build(builder,
                                                           verifier_out);
}
void f51(rustls_web_pki_server_cert_verifier_builder *builder) {
  return rustls_web_pki_server_cert_verifier_builder_free(builder);
}
rustls_web_pki_server_cert_verifier_builder *
f52(const rustls_root_cert_store *store) {
  return rustls_web_pki_server_cert_verifier_builder_new(store);
}

uint16_t f53(const rustls_supported_ciphersuite *supported_ciphersuite) {
  return rustls_supported_ciphersuite_get_suite(supported_ciphersuite);
}
rustls_tls_version
f54(const rustls_supported_ciphersuite *supported_ciphersuite) {
  return rustls_supported_ciphersuite_protocol_version(supported_ciphersuite);
}

rustls_str f55() { return rustls_version(); }

void f56(unsigned int result, char *buf, size_t len, size_t *out_n) {
  return rustls_error(result, buf, len, out_n);
}
bool f57(unsigned int result) { return rustls_result_is_cert_error(result); }

rustls_io_result f58(rustls_connection *conn, rustls_read_callback callback,
                     void *userdata, size_t *out_n) {
  return rustls_connection_read_tls(conn, callback, userdata, out_n);
}
rustls_io_result f59(rustls_connection *conn, rustls_write_callback callback,
                     void *userdata, size_t *out_n) {
  return rustls_connection_write_tls(conn, callback, userdata, out_n);
}
rustls_result f60(rustls_client_config_builder *builder,
                  rustls_keylog_log_callback log_cb,
                  rustls_keylog_will_log_callback will_log_cb) {
  return rustls_client_config_builder_set_key_log(builder, log_cb, will_log_cb);
}
rustls_result f61(rustls_client_config_builder *config_builder,
                  rustls_verify_server_cert_callback callback) {
  return rustls_client_config_builder_dangerous_set_certificate_verifier(
      config_builder, callback);
}
