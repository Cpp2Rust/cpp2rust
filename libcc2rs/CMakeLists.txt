set(LIBCC2RC_LIB_DIR ${CMAKE_SOURCE_DIR}/libcc2rs)
set(LIBCC2RC_MACROS_LIB_DIR ${CMAKE_SOURCE_DIR}/libcc2rs-macros)
set(LIBCC2RS_RLIB ${LIBCC2RC_LIB_DIR}/target/release/liblibcc2rs.rlib)
set(LIBCC2RS_MACROS_RLIB ${LIBCC2RC_LIB_DIR}/target/release/liblibcc2rs-macros.rlib)
file(GLOB_RECURSE LIBCC2RS_SOURCES "${LIBCC2RC_LIB_DIR}/src/*.rs")
file(GLOB_RECURSE LIBCC2RS_MACROS_SOURCES "${LIBCC2RC_MACROS_LIB_DIR}/src/*.rs")

add_custom_command(
    OUTPUT ${LIBCC2RS_RLIB}
    COMMAND cargo +${RUST_STABLE_VERSION} build --release
    WORKING_DIRECTORY ${LIBCC2RC_LIB_DIR}
    DEPENDS
        "${RUST_STAMP_FILE}"
        ${LIBCC2RS_SOURCES}
        ${LIBCC2RS_MACROS_SOURCES}
        ${LIBCC2RC_LIB_DIR}/Cargo.toml
        ${LIBCC2RC_MACROS_LIB_DIR}/Cargo.toml
)

add_custom_command(
    OUTPUT ${LIBCC2RS_MACROS_RLIB}
    COMMAND cargo +${RUST_STABLE_VERSION} build --release
    WORKING_DIRECTORY ${LIBCC2RC_MACROS_LIB_DIR}
    DEPENDS
        "${RUST_STAMP_FILE}"
        ${LIBCC2RS_MACROS_SOURCES}
        ${LIBCC2RC_MACROS_LIB_DIR}/Cargo.toml
)

add_custom_target(libcc2rs ALL
    DEPENDS ${LIBCC2RS_RLIB} ${LIBCC2RS_MACROS_RLIB}
)
