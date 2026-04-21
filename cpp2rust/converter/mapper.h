#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <filesystem>
#include <string>
#include <unordered_map>

#include "converter/factory.h"
#include "converter/translation_rule.h"

namespace cpp2rust::Mapper {
class PushASTContext {
public:
  explicit PushASTContext(clang::ASTContext &ctx);
  ~PushASTContext();
  PushASTContext(const PushASTContext &) = delete;
  PushASTContext &operator=(const PushASTContext &) = delete;

private:
  clang::ASTContext *prev_;
};

bool Contains(clang::QualType qual_type);
bool Contains(const clang::Expr *expr);

std::string Map(clang::QualType qual_type);
const TranslationRule::ExprTgt *GetExprTgt(const clang::Expr *expr);
std::string InstantiateTemplate(const clang::Expr *expr,
                                const std::string &text);
bool ReturnsPointer(const clang::Expr *expr);
std::string GetParamType(const clang::Expr *expr, unsigned index);
bool ParamIsPointer(const clang::Expr *expr, unsigned index);
bool MapsToPointer(clang::QualType qual_type);
bool MapsToRefcountPointer(clang::QualType qual_type);

std::string ToString(clang::QualType qual_type);
std::string ToString(const clang::Expr *expr);
std::string ToString(const clang::NamedDecl *decl);

void LoadTranslationRules(Model model, clang::ASTContext &ctx,
                          const std::string &rules_dir);
void AddRuleForUserDefinedType(clang::NamedDecl *decl);
} // namespace cpp2rust::Mapper
