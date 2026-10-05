#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <string>

namespace cpp2rust::Printer {
enum class ScalarSugar {
  kDesugar,
  kPreserve,
};

std::string ToString(clang::ASTContext &ctx, clang::QualType qual_type,
                     ScalarSugar sugar = ScalarSugar::kDesugar);
std::string ToString(clang::ASTContext &ctx, const clang::Expr *expr);
std::string ToString(clang::ASTContext &ctx, const clang::NamedDecl *decl);
std::string ToRustName(std::string name);
} // namespace cpp2rust::Printer
