set(LIBC_DEP_DIR ${CMAKE_SOURCE_DIR}/libc-dep)
set(LIBC_DEP_OUTPUT ${LIBC_DEP_DIR}/target/release/libc_dep.rlib)

add_custom_command(
    OUTPUT ${LIBC_DEP_OUTPUT}
    COMMAND cargo +${RUST_STABLE_VERSION} build --release
    WORKING_DIRECTORY ${LIBC_DEP_DIR}
    DEPENDS "${RUST_STAMP_FILE}" ${LIBC_DEP_DIR}/Cargo.toml
)

add_custom_target(
    libc_dep ALL
    DEPENDS "${LIBC_DEP_OUTPUT}"
)
