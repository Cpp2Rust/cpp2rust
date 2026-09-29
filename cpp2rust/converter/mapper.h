#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <string>

#include "converter/rules/matching.h"
#include "converter/translation_rule.h"

namespace cpp2rust::Mapper {
bool Contains(clang::ASTContext &ctx, clang::QualType qual_type);
bool Contains(clang::ASTContext &ctx, const clang::Expr *expr);

std::string Map(clang::ASTContext &ctx, clang::QualType qual_type);
std::string MapInitializer(clang::ASTContext &ctx, clang::QualType qual_type);
const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr);
bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr);
std::string MapFunctionName(clang::ASTContext &ctx,
                            const clang::FunctionDecl *decl);
std::string InstantiateTemplate(clang::ASTContext &ctx, const clang::Expr *expr,
                                unsigned n);
bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr);
std::string GetParamType(clang::ASTContext &ctx, const clang::Expr *expr,
                         unsigned index);
bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index);
bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type);
bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type);
const std::vector<std::string> *MappedDerives(clang::ASTContext &ctx,
                                              clang::QualType qual_type);
void SetDerives(clang::ASTContext &ctx, clang::QualType qual_type,
                std::vector<std::string> derives);

std::string InstantiateTgt(const Matching::Bindings &types,
                           const std::string &tgt_template);
} // namespace cpp2rust::Mapper
