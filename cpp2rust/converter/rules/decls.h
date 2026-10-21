#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/Expr.h>
#include <clang/AST/TemplateBase.h>
#include <clang/AST/Type.h>
#include <clang/Basic/Diagnostic.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Sema/Sema.h>

#include <optional>
#include <vector>

namespace cpp2rust::RuleDecls {
class Trial {
public:
  explicit Trial(clang::Sema &sema)
      : diagnostics_(sema.getDiagnostics()),
        suppressed_(diagnostics_.getSuppressAllDiagnostics()),
        trap_(sema, /*ForValidityCheck=*/true) {
    diagnostics_.setSuppressAllDiagnostics(true);
  }

  ~Trial() { diagnostics_.setSuppressAllDiagnostics(suppressed_); }

  bool failed() const { return trap_.hasErrorOccurred(); }

private:
  clang::DiagnosticsEngine &diagnostics_;
  bool suppressed_;
  clang::Sema::SFINAETrap trap_;
};

const clang::Expr *SkipImplicit(const clang::Expr *expr);

const clang::Expr *GetReturned(const clang::FunctionDecl *rule);

const clang::Expr *GetMacroMemberBase(const clang::SourceManager &sm,
                                      const clang::MemberExpr *member);

std::optional<std::vector<clang::TemplateArgument>>
DeduceFromType(clang::Sema &sema, clang::TemplateParameterList *params,
               clang::QualType pattern, clang::QualType actual);

bool IsInternal(const clang::NamedDecl *decl);

} // namespace cpp2rust::RuleDecls
