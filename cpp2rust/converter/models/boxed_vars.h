#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>

#include <unordered_set>

namespace cpp2rust {
// The local variables and parameters that must be stored in a Value, as
// pointers to them are made: their address is taken, either explicitly with &
// or by binding a reference to them, e.g., when they are passed by reference
// to a function, or captured by a lambda. Any use that doesn't just read or
// write the value of a variable is taken to make a pointer to it.
//
// Only scalars, arrays and user-defined structs are stored directly; other
// variables are always boxed.
class BoxedVars {
public:
  explicit BoxedVars(clang::ASTContext &ctx);

  bool contains(const clang::VarDecl *decl) const;

private:
  std::unordered_set<const clang::VarDecl *> address_taken_;
};

// The reads of the value of variables in a returned expression that are the
// only reference to their variable in it. As the variables die on return,
// these reads can move the value instead of copying it. Excluded are the
// reads that may be translated more than once, or in a loop, such as the
// arguments of calls translated by rules.
std::unordered_set<const clang::DeclRefExpr *>
FindMovableReads(clang::ASTContext &ctx, clang::Expr *expr);
} // namespace cpp2rust
