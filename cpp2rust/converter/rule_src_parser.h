// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <llvm/Support/JSON.h>

#include <filesystem>

namespace cpp2rust::RuleSrcParser {

// Visit every f<n> / t<n> declaration in `src_path`:
// f<n> / t<n>: { "to_string": <Mapper::ToString> }
void Extract(const std::filesystem::path &src_path, llvm::json::Object &out);

} // namespace cpp2rust::RuleSrcParser
