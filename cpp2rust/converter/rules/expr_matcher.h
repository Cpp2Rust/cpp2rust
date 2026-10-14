#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/TemplateBase.h>
#include <clang/Sema/Sema.h>
#include <llvm/ADT/ArrayRef.h>

#include <optional>

namespace cpp2rust::AstMatcher {
std::optional<llvm::ArrayRef<clang::TemplateArgument>>
Match(clang::Sema &sema, const clang::Expr *use,
      const clang::NamedDecl *rule_decl);

const clang::NamedDecl *MoreSpecialized(clang::Sema &sema,
                                        const clang::Expr *use,
                                        const clang::NamedDecl *a,
                                        const clang::NamedDecl *b);
} // namespace cpp2rust::AstMatcher
