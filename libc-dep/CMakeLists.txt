set(LIBC_DEP_DIR ${CMAKE_SOURCE_DIR}/libc-dep)

add_custom_command(
    OUTPUT ${LIBCC2RC_LIB_DIR}/target/release/libc_dep.rlib ALL
    COMMAND cargo +${RUST_STABLE_VERSION} build --release
    WORKING_DIRECTORY ${LIBC_DEP_DIR}
    DEPENDS "${RUST_STAMP_FILE}" ${LIBC_DEP_DIR}/Cargo.toml
)
