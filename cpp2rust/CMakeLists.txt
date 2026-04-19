file(GLOB_RECURSE SOURCES "*.cpp")
add_clang_executable(cpp2rust ${SOURCES})

target_link_libraries(cpp2rust PRIVATE
    clangAST
    clangASTMatchers
    clangBasic
    clangFrontend
    clangSema
    clangSerialization
    clangTooling
)
target_include_directories(cpp2rust PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(cpp2rust PUBLIC SYSTEM ${CLANG_INCLUDE_DIRS} ${LLVM_INCLUDE_DIRS})
target_compile_definitions(cpp2rust PUBLIC ${LLVM_DEFINITIONS})
target_compile_definitions(cpp2rust PUBLIC "-DCLANG_C_COMPILER=\"${CMAKE_C_COMPILER}\"")
target_compile_definitions(cpp2rust PUBLIC "-DCLANG_CXX_COMPILER=\"${CMAKE_CXX_COMPILER}\"")
target_compile_definitions(cpp2rust PUBLIC "-DCLANG_RESOURCE_DIR=\"${LLVM_LIBRARY_DIR}/clang/${LLVM_VERSION_MAJOR}\"")
target_compile_definitions(cpp2rust PUBLIC "-DCOMPAT_INCLUDE_DIR=\"${CMAKE_CURRENT_SOURCE_DIR}/compat\"")

if (APPLE)
  execute_process(
    COMMAND xcrun --show-sdk-path
    OUTPUT_VARIABLE MACOS_SDK_PATH
    OUTPUT_STRIP_TRAILING_WHITESPACE
  )
  target_compile_definitions(cpp2rust PUBLIC "-DMACOS_SDK_PATH=\"${MACOS_SDK_PATH}\"")
endif()

llvm_map_components_to_libnames(llvm_libs support)
target_link_libraries(cpp2rust PRIVATE ${llvm_libs})
