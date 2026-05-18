#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>

#include <string>
#include <utility>

namespace cpp2rust {
enum class Model {
  kUnsafe = 0,
  kRefCount,
};

class Converter;

std::unique_ptr<Converter> CreateConverter(std::string &rs_code,
                                           clang::ASTContext &ctx, Model model,
                                           const std::string &rules_dir);
} // namespace cpp2rust
