cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(Cpp2Rust)

if (NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()

if (MSVC)
  message(FATAL_ERROR "MSVC is not supported")
else()
  # Set up the flags for GCC/Clang
  set(CMAKE_CXX_FLAGS_RELEASE        "-O3 ${CMAKE_CXX_FLAGS_RELEASE}")
  set(CMAKE_CXX_FLAGS_DEBUG          "-g -Og ${CMAKE_CXX_FLAGS_DEBUG}")
  set(CMAKE_CXX_FLAGS_RELWITHDEBINFO "-O3 -g ${CMAKE_CXX_FLAGS_RELWITHDEBINFO}")
  set(CMAKE_CXX_FLAGS                "-Wall -Werror ${CMAKE_CXX_FLAGS}")

  # Sanitizers
  # set(CMAKE_CXX_FLAGS_DEBUG        "-fsanitize=address -fsanitize=undefined -fsanitize=leak ${CMAKE_CXX_FLAGS_DEBUG}")
  # set(CMAKE_EXE_LINKER_FLAGS_DEBUG "-fsanitize=address -fsanitize=undefined -fsanitize=leak ${CMAKE_EXE_LINKER_FLAGS_DEBUG}")
endif()

find_package(LLVM REQUIRED CONFIG)
find_package(Clang REQUIRED CONFIG)
message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using ClangConfig.cmake in: ${Clang_DIR}")

list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${CLANG_CMAKE_DIR}")
include(AddLLVM)
include(AddClang)

if (NOT CMAKE_CXX_COMPILER MATCHES "clang" AND NOT DEFINED ENV{CODEQL_EXTRACTOR_CPP_ROOT})
  set(CMAKE_CXX_COMPILER "${CLANG_CMAKE_DIR}/../../../bin/clang++")
endif()

message(STATUS "C++ flags: ${CMAKE_CXX_FLAGS}")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C Compiler: ${CMAKE_C_COMPILER}")
message(STATUS "C++ Compiler: ${CMAKE_CXX_COMPILER}")

execute_process(
  COMMAND ${CMAKE_CXX_COMPILER} --version
  OUTPUT_VARIABLE CLANG_OUTPUT
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
message(STATUS ${CLANG_OUTPUT})

execute_process(
  COMMAND rustc --version --verbose
  OUTPUT_VARIABLE RUSTC_OUTPUT
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
string(REGEX MATCH "LLVM version: .*" RUSTC_LLVM_VERSION_LINE "${RUSTC_OUTPUT}")
string(REGEX REPLACE "LLVM version: " "" RUSTC_LLVM_VERSION "${RUSTC_LLVM_VERSION_LINE}")
message(STATUS "rustc using LLVM ${RUSTC_LLVM_VERSION}")

if (NOT RUSTC_LLVM_VERSION STREQUAL LLVM_PACKAGE_VERSION)
  message(WARNING "The LLVM versions used by rustc and clang are different!")
endif()

include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/rust-toolchain.cmake)
set(RUST_STAMP_FILE "${CMAKE_BINARY_DIR}/.rust-toolchain.stamp")
set(RUST_NIGHTLY_VERSION nightly-2026-05-13)

add_custom_command(
    OUTPUT "${RUST_STAMP_FILE}"
    COMMAND rustup toolchain install ${RUST_STABLE_VERSION}
    COMMAND rustup toolchain install ${RUST_NIGHTLY_VERSION} --component rustc-dev --component llvm-tools
    COMMAND ${CMAKE_COMMAND} -E touch "${RUST_STAMP_FILE}"
    DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/cmake/rust-toolchain.cmake"
    USES_TERMINAL
    VERBATIM
)

add_subdirectory(cpp2rust)
add_subdirectory(libcc2rs)
add_subdirectory(tests)

find_program(CLANG_FORMAT NAMES clang-format-22 clang-format)
find_program(RUFF NAMES ruff)

file(GLOB_RECURSE ALL_CXX_SOURCES
     cpp2rust/*.cpp cpp2rust/*.h tests/*.cpp tests/*.c)

set(PYTHON_SOURCES ${PROJECT_SOURCE_DIR}/tests/lit/lit/formats/Cpp2RustTest.py)

add_custom_target("format"
                  COMMAND ${CLANG_FORMAT} -i ${ALL_CXX_SOURCES}
                  COMMAND cargo +${RUST_STABLE_VERSION} fmt --manifest-path ${PROJECT_SOURCE_DIR}/rules/Cargo.toml
                  COMMAND cargo +${RUST_STABLE_VERSION}  --manifest-path ${PROJECT_SOURCE_DIR}/rule-preprocessor/Cargo.toml
                  COMMAND cargo +${RUST_STABLE_VERSION} fmt --manifest-path ${PROJECT_SOURCE_DIR}/libcc2rs/Cargo.toml
                  COMMAND cargo +${RUST_STABLE_VERSION} fmt --manifest-path ${PROJECT_SOURCE_DIR}/libcc2rs-macros/Cargo.toml
                  COMMAND cargo +${RUST_STABLE_VERSION} clippy --fix --allow-dirty --allow-staged --all-targets --all-features
                                --manifest-path ${PROJECT_SOURCE_DIR}/rules/Cargo.toml -- -Dwarnings
                  COMMAND cargo +${RUST_STABLE_VERSION} clippy --fix --allow-dirty --allow-staged --all-targets --all-features
                                --manifest-path chdir ${PROJECT_SOURCE_DIR}/rule-preprocessor/Cargo.toml -- -Dwarnings
                  COMMAND cargo +${RUST_STABLE_VERSION} clippy --fix --allow-dirty --allow-staged --all-targets --all-features
                                --manifest-path $${PROJECT_SOURCE_DIR}/libcc2rs/Cargo.toml -- -Dwarnings
                  COMMAND cargo +${RUST_STABLE_VERSION} clippy --fix --allow-dirty --allow-staged --all-targets --all-features
                                --manifest-path ${PROJECT_SOURCE_DIR}/libcc2rs-macros/Cargo.toml -- -Dwarnings
                  COMMAND ${RUFF} --silent check --fix ${PYTHON_SOURCES}
                  COMMAND ${RUFF} --silent format ${PYTHON_SOURCES}
                  DEPENDS "${RUST_STAMP_FILE}"
                  COMMENT "Running clang-format, cargo fmt, cargo clippy, and ruff on all source files"
                  VERBATIM
                  )

if (NOT LIT_FLAGS)
  set(LIT_FLAGS "-s;--show-xfail;-vv")
endif()

add_custom_target("check-libcc2rs"
                  COMMAND cargo +${RUST_STABLE_VERSION} test
                  WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}/libcc2rs"
                  DEPENDS "${RUST_STAMP_FILE}"
                  USES_TERMINAL
                 )

add_custom_target("check-unit"
                  COMMAND ${CMAKE_COMMAND} -E env
                          "CC=${CMAKE_C_COMPILER}"
                          "CXX=${CMAKE_CXX_COMPILER}"
                          "python3"
                          "${PROJECT_SOURCE_DIR}/tests/lit/lit.py"
                          ${LIT_FLAGS}
                          "${PROJECT_SOURCE_DIR}/tests/unit"
                          "${PROJECT_SOURCE_DIR}/tests/ub"
                  DEPENDS cpp2rust libcc2rs
                  USES_TERMINAL
                 )

add_custom_target("check-benchmarks"
                  COMMAND ${CMAKE_COMMAND} -E env
                          "CC=${CMAKE_C_COMPILER}"
                          "CXX=${CMAKE_CXX_COMPILER}"
                          "python3"
                          "${PROJECT_SOURCE_DIR}/tests/lit/lit.py"
                          ${LIT_FLAGS}
                          "${PROJECT_SOURCE_DIR}/tests/benchmarks"
                  DEPENDS cpp2rust libcc2rs
                  USES_TERMINAL
                 )

add_custom_target("check"
                  DEPENDS check-libcc2rs check-unit
                 )

file(GLOB rule_subdirs ${PROJECT_SOURCE_DIR}/rules/*)
set(cpp_rules_ir_outputs)
set(rust_rules_ir_outputs)
set(rust_rules_inputs)
foreach(_rule_dir IN LISTS rule_subdirs)
  file(GLOB _srcs ${_rule_dir}/src.c ${_rule_dir}/src.cpp)
  if(NOT _srcs)
    continue()
  endif()
  set(_out ${_rule_dir}/ir_src.json)
  add_custom_command(
    OUTPUT ${_out}
    COMMAND $<TARGET_FILE:cpp-rule-preprocessor> --dir ${_rule_dir}
    DEPENDS ${_srcs} ${PROJECT_SOURCE_DIR}/cpp2rust/cpp_rule_preprocessor.cpp
    VERBATIM
  )
  list(APPEND cpp_rules_ir_outputs ${_out})
  if(EXISTS ${_rule_dir}/tgt_unsafe.rs)
    list(APPEND rust_rules_ir_outputs ${_rule_dir}/ir_unsafe.json)
    list(APPEND rust_rules_inputs ${_rule_dir}/tgt_unsafe.rs)
  endif()
  if(EXISTS ${_rule_dir}/tgt_refcount.rs)
    list(APPEND rust_rules_ir_outputs ${_rule_dir}/ir_refcount.json)
    list(APPEND rust_rules_inputs ${_rule_dir}/tgt_refcount.rs)
  endif()
endforeach()

add_custom_target("preprocess-cpp-rules" ALL DEPENDS ${cpp_rules_ir_outputs})

file(GLOB_RECURSE rule_preprocessor_sources
     ${PROJECT_SOURCE_DIR}/rule-preprocessor/src/*.rs)

add_custom_command(
  OUTPUT ${rust_rules_ir_outputs}
  COMMAND cargo +${RUST_STABLE_VERSION} build --release --manifest-path "${PROJECT_SOURCE_DIR}/rules/Cargo.toml"
  COMMAND ${CMAKE_COMMAND} -E env
          CARGO_TARGET_DIR="${CMAKE_CURRENT_BINARY_DIR}/target_preprocessor"
          cargo +${RUST_NIGHTLY_VERSION} run --manifest-path "${PROJECT_SOURCE_DIR}/rule-preprocessor/Cargo.toml"
  DEPENDS ${rust_rules_inputs} ${rule_preprocessor_sources} "${RUST_STAMP_FILE}"
  VERBATIM
)

add_custom_target("preprocess-rust-rules" ALL
                  DEPENDS ${rust_rules_ir_outputs})

add_custom_target("check-rules"
                  COMMAND find ${PROJECT_SOURCE_DIR}/rules -name "ir_unsafe.json" -delete
                  COMMAND find ${PROJECT_SOURCE_DIR}/rules -name "ir_refcount.json" -delete
                  COMMAND find ${PROJECT_SOURCE_DIR}/rules -name "ir_src.json" -delete
                  COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target preprocess-cpp-rules
                  COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target preprocess-rust-rules
                  COMMAND ${CMAKE_COMMAND} -E chdir ${PROJECT_SOURCE_DIR}
                          git diff --exit-code -- rules/
                 )

add_custom_target("check-all"
                  DEPENDS check-libcc2rs check-rules check-unit check-benchmarks
                 )
