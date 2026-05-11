file(GLOB_RECURSE CORE_SOURCES "*.cpp")
list(REMOVE_ITEM CORE_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/cpp2rust.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/cpp_rule_preprocessor.cpp"
)

add_library(cpp2rust_core STATIC ${CORE_SOURCES})

target_link_libraries(cpp2rust_core PUBLIC
    clangAST
    clangASTMatchers
    clangBasic
    clangFrontend
    clangLex
    clangSema
    clangSerialization
    clangTooling
)
target_include_directories(cpp2rust_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(cpp2rust_core SYSTEM PUBLIC ${CLANG_INCLUDE_DIRS} ${LLVM_INCLUDE_DIRS})
target_compile_definitions(cpp2rust_core PUBLIC ${LLVM_DEFINITIONS})
target_compile_definitions(cpp2rust_core PUBLIC "-DCLANG_C_COMPILER=\"${CMAKE_C_COMPILER}\"")
target_compile_definitions(cpp2rust_core PUBLIC "-DCLANG_CXX_COMPILER=\"${CMAKE_CXX_COMPILER}\"")
target_compile_definitions(cpp2rust_core PUBLIC "-DCLANG_RESOURCE_DIR=\"${LLVM_LIBRARY_DIR}/clang/${LLVM_VERSION_MAJOR}\"")
target_compile_definitions(cpp2rust_core PUBLIC "-DCOMPAT_INCLUDE_DIR=\"${CMAKE_CURRENT_SOURCE_DIR}/compat\"")

if (APPLE)
  execute_process(
    COMMAND xcrun --show-sdk-path
    OUTPUT_VARIABLE MACOS_SDK_PATH
    OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  target_compile_definitions(cpp2rust_core PUBLIC "-DMACOS_SDK_PATH=\"${MACOS_SDK_PATH}\"")
endif()

llvm_map_components_to_libnames(llvm_libs support)
target_link_libraries(cpp2rust_core PUBLIC ${llvm_libs})

add_clang_executable(cpp2rust PARTIAL_SOURCES_INTENDED cpp2rust.cpp)
target_link_libraries(cpp2rust PRIVATE cpp2rust_core)

add_clang_executable(cpp-rule-preprocessor PARTIAL_SOURCES_INTENDED cpp_rule_preprocessor.cpp)
target_link_libraries(cpp-rule-preprocessor PRIVATE cpp2rust_core)
