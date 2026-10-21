#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Decl.h>
#include <clang/AST/TemplateBase.h>
#include <clang/AST/Type.h>
#include <clang/Sema/Sema.h>

#include <optional>
#include <vector>

namespace cpp2rust::AstMatcher {
struct TypeMatch {
  std::vector<clang::TemplateArgument> args;
};

std::optional<TypeMatch> MatchType(clang::Sema &sema, clang::QualType use,
                                   const clang::NamedDecl *rule);

bool MatchPublicAlias(clang::ASTContext &ctx, clang::QualType use,
                      const clang::NamedDecl *rule);

clang::QualType SkipInternalTypedefs(clang::ASTContext &ctx,
                                     clang::QualType use);

const clang::TypedefNameDecl *SpelledTypedef(clang::ASTContext &ctx,
                                             clang::QualType use);

const clang::NamedDecl *MoreSpecializedType(clang::Sema &sema,
                                            const clang::NamedDecl *a,
                                            const clang::NamedDecl *b);
} // namespace cpp2rust::AstMatcher
