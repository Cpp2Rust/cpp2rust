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
inline constexpr unsigned kCanonicalDepth = -1u;

struct TypeMatch {
  std::vector<clang::TemplateArgument> args;
  unsigned depth;
};

std::optional<TypeMatch> MatchType(clang::Sema &sema, clang::QualType use,
                                   const clang::NamedDecl *rule);

const clang::NamedDecl *MoreSpecializedType(clang::Sema &sema,
                                            const clang::NamedDecl *a,
                                            const clang::NamedDecl *b);
} // namespace cpp2rust::AstMatcher
