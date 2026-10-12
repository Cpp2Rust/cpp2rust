#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <string>

#include "converter/translation_rule.h"

namespace cpp2rust {
class Converter;
}

namespace cpp2rust::Mapper {
bool Contains(clang::ASTContext &ctx, clang::QualType qual_type);
bool Contains(clang::ASTContext &ctx, const clang::Expr *expr);

std::string Map(clang::ASTContext &ctx, clang::QualType qual_type);
std::string Map(Converter &converter, clang::QualType qual_type);
std::string MapInitializer(Converter &converter, clang::QualType qual_type);
std::string MapFunctionName(clang::ASTContext &ctx,
                            const clang::FunctionDecl *decl);
std::string InstantiateTemplate(Converter &converter, const clang::Expr *expr,
                                unsigned n);
std::string GetParamType(Converter &converter, const clang::Expr *expr,
                         unsigned index);
} // namespace cpp2rust::Mapper
