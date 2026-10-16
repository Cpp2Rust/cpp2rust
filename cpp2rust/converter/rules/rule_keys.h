#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclarationName.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>
#include <llvm/ADT/Hashing.h>

#include <optional>
#include <variant>
#include <vector>

namespace cpp2rust::RuleKeys {
struct Member {
  const clang::Decl *parent;
  clang::DeclarationName name;

  bool operator==(const Member &) const = default;
};

// Constructors and conversion operators.
struct Constructor {
  const clang::Decl *parent;

  bool operator==(const Constructor &) const = default;
};

struct Unqualified {
  clang::DeclarationName name;

  bool operator==(const Unqualified &) const = default;
};

using ExprKey = std::variant<Member, Constructor, Unqualified>;

using TypeKey = std::variant<const clang::Decl *, const clang::Type *>;

inline const TypeKey kAnyClass = static_cast<const clang::Decl *>(nullptr);
inline const Constructor kAnyConstructor{nullptr};

std::optional<ExprKey> GetExprKey(clang::ASTContext &ctx,
                                  const clang::Expr *use);

std::vector<ExprKey> GetExprKeys(clang::ASTContext &ctx,
                                 const clang::NamedDecl *rule);

std::optional<ExprKey> GetDeclKey(const clang::NamedDecl *decl);

std::optional<TypeKey> GetTypeKey(clang::QualType type);

std::vector<TypeKey> GetTypeKeys(const clang::NamedDecl *rule);

std::vector<TypeKey> GetTypeKeys(clang::ASTContext &ctx, clang::QualType use);
} // namespace cpp2rust::RuleKeys

template <> struct std::hash<cpp2rust::RuleKeys::Member> {
  size_t operator()(const cpp2rust::RuleKeys::Member &key) const {
    return llvm::hash_combine(key.parent, key.name.getAsOpaquePtr());
  }
};

template <> struct std::hash<cpp2rust::RuleKeys::Constructor> {
  size_t operator()(const cpp2rust::RuleKeys::Constructor &key) const {
    return llvm::hash_value(key.parent);
  }
};

template <> struct std::hash<cpp2rust::RuleKeys::Unqualified> {
  size_t operator()(const cpp2rust::RuleKeys::Unqualified &key) const {
    return llvm::hash_value(key.name.getAsOpaquePtr());
  }
};
