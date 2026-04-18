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

if (NOT CMAKE_CXX_COMPILER MATCHES "clang")
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

set(RUST_STABLE_VERSION "1.94.0")

function(write_rust_toolchain DIR)
  set(_content "[toolchain]\nchannel = \"${RUST_STABLE_VERSION}\"\n")
  set(_file ${DIR}/rust-toolchain.toml)
  if (EXISTS ${_file})
    file(READ ${_file} _existing)
  endif()
  if (NOT "${_existing}" STREQUAL "${_content}")
    file(WRITE ${_file} ${_content})
  endif()
endfunction()

# rule-preprocessor uses nightly
foreach(_dir libcc2rs rules)
  write_rust_toolchain(${CMAKE_SOURCE_DIR}/${_dir})
endforeach()

add_custom_target("install-rust-toolchain"
                  COMMAND rustup toolchain install ${RUST_STABLE_VERSION}
                  USES_TERMINAL
                 )

add_subdirectory(cpp2rust)
add_subdirectory(libcc2rs)
add_subdirectory(tests)

find_program(CLANG_FORMAT NAMES clang-format-22 clang-format)

file(GLOB_RECURSE ALL_CXX_SOURCES
     cpp2rust/*.cpp cpp2rust/*.h tests/*.cpp tests/*.c)

add_custom_target("format"
                  COMMAND ${CLANG_FORMAT} -i ${ALL_CXX_SOURCES}
                  COMMAND rustup run ${RUST_STABLE_VERSION} cargo fmt --manifest-path ${PROJECT_SOURCE_DIR}/rules/Cargo.toml
                  COMMAND rustup run ${RUST_STABLE_VERSION} cargo fmt --manifest-path ${PROJECT_SOURCE_DIR}/rule-preprocessor/Cargo.toml
                  COMMAND rustup run ${RUST_STABLE_VERSION} cargo fmt --manifest-path ${PROJECT_SOURCE_DIR}/libcc2rs/Cargo.toml
                  DEPENDS "install-rust-toolchain"
                  COMMENT "Running clang-format and cargo fmt on all source files"
                  VERBATIM
                  )

if (NOT LIT_FLAGS)
  set(LIT_FLAGS "-s;--show-xfail;-vv")
endif()

add_custom_target("check-libcc2rs"
                  COMMAND cargo test
                  WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}/libcc2rs"
                  DEPENDS install-rust-toolchain
                  USES_TERMINAL
                 )

add_custom_target("check-unit"
                  COMMAND "python3"
                          "${PROJECT_SOURCE_DIR}/tests/lit/lit.py"
                          ${LIT_FLAGS}
                          "${PROJECT_SOURCE_DIR}/tests/unit"
                          "${PROJECT_SOURCE_DIR}/tests/ub"
                  DEPENDS cpp2rust libcc2rs
                  USES_TERMINAL
                 )

add_custom_target("check-benchmarks"
                  COMMAND "python3"
                          "${PROJECT_SOURCE_DIR}/tests/lit/lit.py"
                          ${LIT_FLAGS}
                          "${PROJECT_SOURCE_DIR}/tests/benchmarks"
                  DEPENDS cpp2rust libcc2rs
                  USES_TERMINAL
                 )

add_custom_target("check"
                  DEPENDS check-libcc2rs check-unit
                 )

add_custom_target("check-rules"
                  COMMAND find ${PROJECT_SOURCE_DIR}/rules -name "ir*.json" -delete
                  COMMAND cargo build
                  COMMAND ${CMAKE_COMMAND} -E chdir ${PROJECT_SOURCE_DIR}/rule-preprocessor
                          ${CMAKE_COMMAND} -E env
                              CARGO_TARGET_DIR=${PROJECT_SOURCE_DIR}/rule-preprocessor/target
                              cargo run
                  COMMAND ${CMAKE_COMMAND} -E chdir ${PROJECT_SOURCE_DIR}
                          git diff --exit-code -- rules/
                  WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}/rules"
                  DEPENDS install-rust-toolchain
                  USES_TERMINAL
                 )
