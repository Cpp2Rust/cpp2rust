// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/Sema/Initialization.h>

#include "converter/converter_lib.h"
#include "converter/mapper.h"
#include "converter/models/converter_refcount.h"
#include "logging.h"

namespace cpp2rust {

bool Converter::emplace_back_plugin_match(clang::CallExpr *call) {
  if (auto member_call = clang::dyn_cast<clang::CXXMemberCallExpr>(call)) {
    return Mapper::ToString(member_call).contains("emplace_back");
  }
  return false;
}

namespace {

clang::QualType getElemTypeFromEmplaceObj(clang::CXXMemberCallExpr *call) {
  auto *obj = GetCallObject(call);
  if (!obj) {
    return clang::QualType();
  }

  auto base = obj->IgnoreParenImpCasts()
                  ->getType()
                  .getNonReferenceType()
                  .getUnqualifiedType();
  while (base->isPointerType()) {
    base = base->getPointeeType().getUnqualifiedType();
  }

  if (auto record_ty = base->getAs<clang::RecordType>()) {
    if (auto template_specialization =
            clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                record_ty->getDecl())) {
      auto &args = template_specialization->getTemplateArgs();
      if (args.size() && args[0].getKind() == clang::TemplateArgument::Type) {
        return args[0].getAsType().getUnqualifiedType();
      }
    }
  }

  return clang::QualType();
}

clang::InitializationKind
buildInitKindAndArgs(clang::CXXMemberCallExpr *call,
                     llvm::SmallVector<clang::Expr *, 4> &args) {
  if (call->getNumArgs() == 1 &&
      clang::isa<clang::InitListExpr>(call->getArg(0)->IgnoreParenImpCasts())) {
    args.push_back(const_cast<clang::Expr *>(call->getArg(0)));
    return clang::InitializationKind::CreateDirectList(call->getExprLoc());
  } else {
    for (unsigned i = 0; i < call->getNumArgs(); ++i) {
      args.push_back(const_cast<clang::Expr *>(call->getArg(i)));
    }
    return clang::InitializationKind::CreateDirect(call->getExprLoc(), {}, {});
  }
}

clang::Expr *performInitAndPeel(clang::Sema &sema, clang::QualType elem_ty,
                                clang::InitializationKind &kind,
                                llvm::SmallVectorImpl<clang::Expr *> &args) {
  auto ent = clang::InitializedEntity::InitializeTemporary(elem_ty);
  clang::InitializationSequence seq(sema, ent, kind, args);
  if (!seq) {
    return nullptr;
  }

  auto result = seq.Perform(sema, ent, kind, args);
  if (result.isInvalid()) {
    return nullptr;
  }

  auto expr = result.get();
  while (true) {
    expr = expr->IgnoreParenImpCasts();
    if (auto materialize_temp =
            clang::dyn_cast<clang::MaterializeTemporaryExpr>(expr)) {
      expr = materialize_temp->getSubExpr();
      continue;
    }
    if (auto bind_temp = clang::dyn_cast<clang::CXXBindTemporaryExpr>(expr)) {
      expr = bind_temp->getSubExpr();
      continue;
    }
    break;
  }

  return expr;
}

std::pair<clang::QualType, const clang::CXXConstructorDecl *>
analyzeEmplaceCall(clang::CXXMemberCallExpr *call, clang::Sema &sema) {
  auto elem_ty = getElemTypeFromEmplaceObj(call);
  const clang::CXXConstructorDecl *ctor = nullptr;

  if (elem_ty.isNull()) {
    return {clang::QualType(), nullptr};
  }

  if (!elem_ty->isRecordType()) {
    return {elem_ty, nullptr};
  }

  llvm::SmallVector<clang::Expr *, 4> args;
  auto kind = buildInitKindAndArgs(call, args);

  if (auto expr = performInitAndPeel(sema, elem_ty, kind, args)) {
    if (auto ctor_expr = clang::dyn_cast<clang::CXXConstructExpr>(expr)) {
      ctor = ctor_expr->getConstructor();
    }
  }

  return {elem_ty, ctor};
}

clang::CXXConstructExpr *buildConstructExpr(clang::CXXMemberCallExpr *call,
                                            clang::Sema &sema) {
  auto [elem_ty, /*expected*/ ctor] = analyzeEmplaceCall(call, sema);
  if (elem_ty.isNull() || !elem_ty->isRecordType()) {
    return nullptr;
  }

  llvm::SmallVector<clang::Expr *, 4> args;
  auto kind = buildInitKindAndArgs(call, args);

  return clang::dyn_cast_or_null<clang::CXXConstructExpr>(
      performInitAndPeel(sema, elem_ty, kind, args));
}

} // namespace

void Converter::emplace_back_emit_push_open(clang::CXXMemberCallExpr *call) {
  {
    PushExprKind push(*this, ExprKind::LValue);
    StrCat(ReplaceAll(ToString(call->getCallee()), "emplace_back", "push"));
  }
  StrCat("(");
}

void Converter::emplace_back_emit_push_close(clang::CXXMemberCallExpr *call) {
  StrCat(")");
}

bool Converter::emplace_back_plugin_convert(clang::CallExpr *call) {
  auto member_call = clang::dyn_cast<clang::CXXMemberCallExpr>(call);
  assert(member_call);

  auto [elem_ty, ctor] = analyzeEmplaceCall(member_call, GetSema());
  assert(!elem_ty.isNull() && "Could not analyze emplace_back type");

  emplace_back_emit_push_open(member_call);

  if (ctor) {
    auto is_argument_moved = false;
    if (call->getNumArgs() > 0) {
      if (auto arg_call = clang::dyn_cast<clang::CallExpr>(call->getArg(0))) {
        is_argument_moved = arg_call->isCallToStdMove();
      }
    }

    if (is_argument_moved) {
      StrCat("std::mem::take(&mut");
    }
    emplace_back_plugin_construct_arg(
        elem_ty, buildConstructExpr(member_call, GetSema()));
    if (is_argument_moved) {
      StrCat(")");
    }
  } else if (elem_ty.isPODType(ctx_)) {
    if (call->getNumArgs() == 0) {
      StrCat(GetDefaultAsString(elem_ty));
    } else {
      assert(call->getNumArgs() == 1 &&
             "multiple arguments passed for building POD type");
      Convert(call->getArg(0));
      StrCat("as");
      StrCat(GetUnsafeTypeAsString(elem_ty));
    }
  } else {
    call->dump(verrs(), ctx_);
    assert(0 && "no ctor and no pod type");
    return false;
  }

  emplace_back_emit_push_close(member_call);
  return true;
}

} // namespace cpp2rust
