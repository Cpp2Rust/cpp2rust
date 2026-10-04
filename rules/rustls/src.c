// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include "rustls.h"
#endif

typedef rustls_connection *CPP2RUST_TYPE_RULE(1);
typedef const rustls_connection *CPP2RUST_TYPE_RULE(2);
typedef const rustls_certificate *CPP2RUST_TYPE_RULE(3);
typedef rustls_str CPP2RUST_TYPE_RULE(4);
typedef rustls_result CPP2RUST_TYPE_RULE(5);
typedef rustls_io_result CPP2RUST_TYPE_RULE(6);
typedef rustls_tls_version CPP2RUST_TYPE_RULE(7);

rustls_result CPP2RUST_EXPR_RULE(1)() { return RUSTLS_RESULT_OK; }
rustls_result CPP2RUST_EXPR_RULE(2)() { return RUSTLS_RESULT_NULL_PARAMETER; }
rustls_result CPP2RUST_EXPR_RULE(3)() { return RUSTLS_RESULT_PLAINTEXT_EMPTY; }
rustls_result CPP2RUST_EXPR_RULE(4)() { return RUSTLS_RESULT_UNEXPECTED_EOF; }

rustls_tls_version CPP2RUST_EXPR_RULE(5)() {
  return RUSTLS_TLS_VERSION_TLSV1_2;
}
rustls_tls_version CPP2RUST_EXPR_RULE(6)() {
  return RUSTLS_TLS_VERSION_TLSV1_3;
}

rustls_result CPP2RUST_EXPR_RULE(7)(rustls_connection *conn, uint8_t *buf,
                                    size_t count, size_t *out_n) {
  return rustls_connection_read(conn, buf, count, out_n);
}
rustls_result CPP2RUST_EXPR_RULE(8)(rustls_connection *conn, const uint8_t *buf,
                                    size_t count, size_t *out_n) {
  return rustls_connection_write(conn, buf, count, out_n);
}
rustls_result CPP2RUST_EXPR_RULE(9)(rustls_connection *conn) {
  return rustls_connection_process_new_packets(conn);
}
bool CPP2RUST_EXPR_RULE(10)(const rustls_connection *conn) {
  return rustls_connection_wants_read(conn);
}
bool CPP2RUST_EXPR_RULE(11)(const rustls_connection *conn) {
  return rustls_connection_wants_write(conn);
}
bool CPP2RUST_EXPR_RULE(12)(const rustls_connection *conn) {
  return rustls_connection_is_handshaking(conn);
}
void CPP2RUST_EXPR_RULE(13)(rustls_connection *conn) {
  return rustls_connection_send_close_notify(conn);
}
void CPP2RUST_EXPR_RULE(14)(rustls_connection *conn, void *userdata) {
  return rustls_connection_set_userdata(conn, userdata);
}
void CPP2RUST_EXPR_RULE(15)(const rustls_connection *conn,
                            const uint8_t **protocol_out,
                            size_t *protocol_out_len) {
  return rustls_connection_get_alpn_protocol(conn, protocol_out,
                                             protocol_out_len);
}
uint16_t CPP2RUST_EXPR_RULE(16)(const rustls_connection *conn) {
  return rustls_connection_get_protocol_version(conn);
}
rustls_str CPP2RUST_EXPR_RULE(17)(const rustls_connection *conn) {
  return rustls_connection_get_negotiated_ciphersuite_name(conn);
}
rustls_str CPP2RUST_EXPR_RULE(18)(const rustls_connection *conn) {
  return rustls_connection_get_negotiated_key_exchange_group_name(conn);
}
const rustls_certificate *CPP2RUST_EXPR_RULE(19)(const rustls_connection *conn,
                                                 size_t i) {
  return rustls_connection_get_peer_certificate(conn, i);
}
void CPP2RUST_EXPR_RULE(20)(rustls_connection *conn) {
  return rustls_connection_free(conn);
}

typedef rustls_client_config *CPP2RUST_TYPE_RULE(8);
typedef const rustls_client_config *CPP2RUST_TYPE_RULE(9);
typedef rustls_client_config_builder *CPP2RUST_TYPE_RULE(10);
typedef const rustls_client_config_builder *CPP2RUST_TYPE_RULE(11);
typedef rustls_certified_key *CPP2RUST_TYPE_RULE(12);
typedef const rustls_certified_key *CPP2RUST_TYPE_RULE(13);
typedef rustls_crypto_provider *CPP2RUST_TYPE_RULE(14);
typedef const rustls_crypto_provider *CPP2RUST_TYPE_RULE(15);
typedef rustls_crypto_provider_builder *CPP2RUST_TYPE_RULE(16);
typedef const rustls_crypto_provider_builder *CPP2RUST_TYPE_RULE(17);
typedef rustls_root_cert_store *CPP2RUST_TYPE_RULE(18);
typedef const rustls_root_cert_store *CPP2RUST_TYPE_RULE(19);
typedef rustls_root_cert_store_builder *CPP2RUST_TYPE_RULE(20);
typedef const rustls_root_cert_store_builder *CPP2RUST_TYPE_RULE(21);
typedef rustls_server_cert_verifier *CPP2RUST_TYPE_RULE(22);
typedef const rustls_server_cert_verifier *CPP2RUST_TYPE_RULE(23);
typedef rustls_web_pki_server_cert_verifier_builder *CPP2RUST_TYPE_RULE(24);
typedef const rustls_web_pki_server_cert_verifier_builder *
    CPP2RUST_TYPE_RULE(25);
typedef rustls_supported_ciphersuite *CPP2RUST_TYPE_RULE(26);
typedef const rustls_supported_ciphersuite *CPP2RUST_TYPE_RULE(27);
typedef rustls_slice_bytes CPP2RUST_TYPE_RULE(28);
typedef rustls_verify_server_cert_params CPP2RUST_TYPE_RULE(29);

rustls_result CPP2RUST_EXPR_RULE(21)(rustls_client_config_builder *builder,
                                     const rustls_client_config **config_out) {
  return rustls_client_config_builder_build(builder, config_out);
}
void CPP2RUST_EXPR_RULE(22)(rustls_client_config_builder *config) {
  return rustls_client_config_builder_free(config);
}
rustls_result
CPP2RUST_EXPR_RULE(23)(const rustls_crypto_provider *provider,
                       const uint16_t *tls_versions, size_t tls_versions_len,
                       rustls_client_config_builder **builder_out) {
  return rustls_client_config_builder_new_custom(provider, tls_versions,
                                                 tls_versions_len, builder_out);
}
rustls_result CPP2RUST_EXPR_RULE(24)(rustls_client_config_builder *builder,
                                     const rustls_slice_bytes *protocols,
                                     size_t len) {
  return rustls_client_config_builder_set_alpn_protocols(builder, protocols,
                                                         len);
}
rustls_result
CPP2RUST_EXPR_RULE(25)(rustls_client_config_builder *builder,
                       const rustls_certified_key *const *certified_keys,
                       size_t certified_keys_len) {
  return rustls_client_config_builder_set_certified_key(builder, certified_keys,
                                                        certified_keys_len);
}
void CPP2RUST_EXPR_RULE(26)(rustls_client_config_builder *builder,
                            const rustls_server_cert_verifier *verifier) {
  return rustls_client_config_builder_set_server_verifier(builder, verifier);
}
void CPP2RUST_EXPR_RULE(27)(const rustls_client_config *config) {
  return rustls_client_config_free(config);
}
rustls_result CPP2RUST_EXPR_RULE(28)(const rustls_client_config *config,
                                     const char *server_name,
                                     rustls_connection **conn_out) {
  return rustls_client_connection_new(config, server_name, conn_out);
}

rustls_result CPP2RUST_EXPR_RULE(29)(const rustls_certificate *cert,
                                     const uint8_t **out_der_data,
                                     size_t *out_der_len) {
  return rustls_certificate_get_der(cert, out_der_data, out_der_len);
}
rustls_result
CPP2RUST_EXPR_RULE(30)(const uint8_t *cert_chain, size_t cert_chain_len,
                       const uint8_t *private_key, size_t private_key_len,
                       const rustls_certified_key **certified_key_out) {
  return rustls_certified_key_build(cert_chain, cert_chain_len, private_key,
                                    private_key_len, certified_key_out);
}
void CPP2RUST_EXPR_RULE(31)(const rustls_certified_key *key) {
  return rustls_certified_key_free(key);
}
rustls_result CPP2RUST_EXPR_RULE(32)(const rustls_certified_key *key) {
  return rustls_certified_key_keys_match(key);
}
rustls_result CPP2RUST_EXPR_RULE(33)(rustls_root_cert_store_builder *builder,
                                     const uint8_t *pem, size_t pem_len,
                                     bool strict) {
  return rustls_root_cert_store_builder_add_pem(builder, pem, pem_len, strict);
}
rustls_result
CPP2RUST_EXPR_RULE(34)(rustls_root_cert_store_builder *builder,
                       const rustls_root_cert_store **root_cert_store_out) {
  return rustls_root_cert_store_builder_build(builder, root_cert_store_out);
}
void CPP2RUST_EXPR_RULE(35)(rustls_root_cert_store_builder *builder) {
  return rustls_root_cert_store_builder_free(builder);
}
rustls_result CPP2RUST_EXPR_RULE(36)(rustls_root_cert_store_builder *builder,
                                     const char *filename, bool strict) {
  return rustls_root_cert_store_builder_load_roots_from_file(builder, filename,
                                                             strict);
}
rustls_root_cert_store_builder *CPP2RUST_EXPR_RULE(37)() {
  return rustls_root_cert_store_builder_new();
}
void CPP2RUST_EXPR_RULE(38)(const rustls_root_cert_store *store) {
  return rustls_root_cert_store_free(store);
}

rustls_result
CPP2RUST_EXPR_RULE(39)(rustls_crypto_provider_builder *builder,
                       const rustls_crypto_provider **provider_out) {
  return rustls_crypto_provider_builder_build(builder, provider_out);
}
void CPP2RUST_EXPR_RULE(40)(rustls_crypto_provider_builder *builder) {
  return rustls_crypto_provider_builder_free(builder);
}
rustls_result
CPP2RUST_EXPR_RULE(41)(rustls_crypto_provider_builder **builder_out) {
  return rustls_crypto_provider_builder_new_from_default(builder_out);
}
rustls_result
CPP2RUST_EXPR_RULE(42)(rustls_crypto_provider_builder *builder,
                       const rustls_supported_ciphersuite *const *cipher_suites,
                       size_t cipher_suites_len) {
  return rustls_crypto_provider_builder_set_cipher_suites(builder, cipher_suites,
                                                          cipher_suites_len);
}
void CPP2RUST_EXPR_RULE(43)(const rustls_crypto_provider *provider) {
  return rustls_crypto_provider_free(provider);
}
const rustls_supported_ciphersuite *CPP2RUST_EXPR_RULE(44)(size_t index) {
  return rustls_default_crypto_provider_ciphersuites_get(index);
}
size_t CPP2RUST_EXPR_RULE(45)() {
  return rustls_default_crypto_provider_ciphersuites_len();
}
rustls_result CPP2RUST_EXPR_RULE(46)(uint8_t *buff, size_t len) {
  return rustls_default_crypto_provider_random(buff, len);
}

rustls_result
CPP2RUST_EXPR_RULE(47)(rustls_server_cert_verifier **verifier_out) {
  return rustls_platform_server_cert_verifier(verifier_out);
}
void CPP2RUST_EXPR_RULE(48)(rustls_server_cert_verifier *verifier) {
  return rustls_server_cert_verifier_free(verifier);
}
rustls_result
CPP2RUST_EXPR_RULE(49)(rustls_web_pki_server_cert_verifier_builder *builder,
                       const uint8_t *crl_pem, size_t crl_pem_len) {
  return rustls_web_pki_server_cert_verifier_builder_add_crl(builder, crl_pem,
                                                             crl_pem_len);
}
rustls_result
CPP2RUST_EXPR_RULE(50)(rustls_web_pki_server_cert_verifier_builder *builder,
                       rustls_server_cert_verifier **verifier_out) {
  return rustls_web_pki_server_cert_verifier_builder_build(builder,
                                                           verifier_out);
}
void CPP2RUST_EXPR_RULE(51)(
    rustls_web_pki_server_cert_verifier_builder *builder) {
  return rustls_web_pki_server_cert_verifier_builder_free(builder);
}
rustls_web_pki_server_cert_verifier_builder *
CPP2RUST_EXPR_RULE(52)(const rustls_root_cert_store *store) {
  return rustls_web_pki_server_cert_verifier_builder_new(store);
}

uint16_t CPP2RUST_EXPR_RULE(53)(
    const rustls_supported_ciphersuite *supported_ciphersuite) {
  return rustls_supported_ciphersuite_get_suite(supported_ciphersuite);
}
rustls_tls_version CPP2RUST_EXPR_RULE(54)(
    const rustls_supported_ciphersuite *supported_ciphersuite) {
  return rustls_supported_ciphersuite_protocol_version(supported_ciphersuite);
}

rustls_str CPP2RUST_EXPR_RULE(55)() { return rustls_version(); }

void CPP2RUST_EXPR_RULE(56)(unsigned int result, char *buf, size_t len,
                            size_t *out_n) {
  return rustls_error(result, buf, len, out_n);
}
bool CPP2RUST_EXPR_RULE(57)(unsigned int result) {
  return rustls_result_is_cert_error(result);
}

rustls_io_result CPP2RUST_EXPR_RULE(58)(rustls_connection *conn,
                                        rustls_read_callback callback,
                                        void *userdata, size_t *out_n) {
  return rustls_connection_read_tls(conn, callback, userdata, out_n);
}
rustls_io_result CPP2RUST_EXPR_RULE(59)(rustls_connection *conn,
                                        rustls_write_callback callback,
                                        void *userdata, size_t *out_n) {
  return rustls_connection_write_tls(conn, callback, userdata, out_n);
}
rustls_result
CPP2RUST_EXPR_RULE(60)(rustls_client_config_builder *builder,
                       rustls_keylog_log_callback log_cb,
                       rustls_keylog_will_log_callback will_log_cb) {
  return rustls_client_config_builder_set_key_log(builder, log_cb, will_log_cb);
}
rustls_result
CPP2RUST_EXPR_RULE(61)(rustls_client_config_builder *config_builder,
                       rustls_verify_server_cert_callback callback) {
  return rustls_client_config_builder_dangerous_set_certificate_verifier(
      config_builder, callback);
}
