set(RUST_LIB_DIR ${CMAKE_SOURCE_DIR}/libcc2rs)
set(RUST_MACROS_DIR ${CMAKE_SOURCE_DIR}/libcc2rs-macros)
set(RUST_BUILD_DIR ${CMAKE_BINARY_DIR}/rust)
file(GLOB_RECURSE LIBCC2RS_SOURCE "${RUST_LIB_DIR}/*.rs")
file(GLOB_RECURSE LIBCC2RS_MACROS_SOURCE "${RUST_MACROS_DIR}/*.rs")

add_custom_command(
    OUTPUT ${RUST_BUILD_DIR}/release/liblibcc2rs.rlib
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${RUST_BUILD_DIR} cargo build --release
    WORKING_DIRECTORY ${RUST_LIB_DIR}
    DEPENDS
        ${LIBCC2RS_SOURCE}
        ${LIBCC2RS_MACROS_SOURCE}
        ${RUST_LIB_DIR}/Cargo.toml
        ${RUST_MACROS_DIR}/Cargo.toml
    COMMENT "Building libcc2rs"
)

add_custom_target(libcc2rs ALL
    DEPENDS ${RUST_BUILD_DIR}/release/liblibcc2rs.rlib "${RUST_STAMP_FILE}"
)
