// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/models/converter_refcount.h"

#include <clang/Basic/OperatorKinds.h>

#include <format>
#include <regex>

#include "compiler.h"
#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/mapper.h"

namespace cpp2rust {
ConverterRefCount::ConverterRefCount(std::string &rs_code,
                                     clang::ASTContext &ctx)
    : Converter(rs_code, ctx, "", "", ".as_pointer()", ""),
      conversion_kind_({ConversionKind::Unboxed}) {}

void ConverterRefCount::EmitFilePreamble() {
  StrCat(R"(
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::{Rc, Weak};
use std::io::Seek;
use std::os::fd::AsFd;
use std::io::prelude::*;
)");
}

static bool IsBoxedType(std::string_view type) {
  return type.starts_with("Vec<") || type.starts_with("Box<");
}

static bool IsBoxedType(clang::QualType type) {
  return IsBoxedType(Mapper::Map(type.getUnqualifiedType()));
}

static bool NeedsMutAccess(const clang::CXXMethodDecl *method,
                           clang::QualType base_type) {
  return !method->isConst() && IsBoxedType(base_type);
}

static bool IsPointerType(clang::QualType type) {
  return type->isPointerType() ||
         GetStrongestIteratorCategory(type) == IteratorCategory::Contiguous;
}

bool ConverterRefCount::PendingDeref::compute_inner_boxed(clang::Expr *expr) {
  if (!expr) {
    return false;
  }
  if (!IsBoxedType(expr->getType().getNonReferenceType())) {
    return false;
  }
  if (auto *ase = clang::dyn_cast<clang::ArraySubscriptExpr>(expr)) {
    auto base_type = ase->getBase()->IgnoreCasts()->getType();
    if (base_type->isPointerType())
      return IsBoxedType(base_type->getPointeeType());
    return IsBoxedType(base_type.getNonReferenceType());
  }
  if (auto *oce = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    return IsBoxedType(oce->getArg(0)->getType().getNonReferenceType());
  }
  return false;
}

void ConverterRefCount::PendingDeref::set(std::string str, clang::Expr *expr) {
  assert_consumed();
  set_unchecked(std::move(str), expr);
}

void ConverterRefCount::PendingDeref::set_unchecked(std::string str,
                                                    clang::Expr *expr) {
  value = std::move(str);
  pointee_is_boxed = compute_inner_boxed(expr);
}

std::string ConverterRefCount::GetInnerType(clang::QualType type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto str = ToString(type);
  auto pos = str.find('<');
  auto end = str.rfind('>');
  if (str[pos + 1] == '[' && str[end - 1] == ']') {
    // Unwrap inner array type
    pos++;
    end--;
  }
  return std::move(str).substr(pos + 1, end - pos - 1);
}

ConverterRefCount::PushUnboxedIfSimple::PushUnboxedIfSimple(
    ConverterRefCount &c, std::string_view outer, clang::QualType inner_type)
    : c(c) {
  bool unboxed = outer == "Ptr<%>" || outer == "%";

  // Vectors are boxed until the last element
  if (!unboxed && (outer == "Vec<%>" || outer == "Box<%>")) {
    if (!IsBoxedType(inner_type)) {
      unboxed = true;
    }
  }

  c.conversion_kind_.push_back(unboxed ? ConversionKind::Unboxed
                                       : ConversionKind::FullRefCount);
}

std::string ConverterRefCount::BoxType(std::string &&str) const {
  switch (getConversionKind()) {
  case ConversionKind::Unboxed:
  case ConversionKind::Ptr:
    return std::move(str);
  case ConversionKind::FullRefCount:
    return std::format("Value<{}>", std::move(str));
  }
  std::unreachable();
}

std::string ConverterRefCount::BoxValue(std::string &&str) const {
  switch (getConversionKind()) {
  case ConversionKind::Unboxed:
  case ConversionKind::Ptr:
    return std::move(str);
  case ConversionKind::FullRefCount:
    return std::format("Rc::new(RefCell::new({}))", std::move(str));
  }
  std::unreachable();
}

bool ConverterRefCount::Convert(clang::QualType qual_type) {
  if (!Mapper::Contains(qual_type))
    qual_type = qual_type.getUnqualifiedType().getDesugaredType(ctx_);

  if (qual_type->isLValueReferenceType() ||
      qual_type->isIncompleteArrayType()) {
    return Converter::Convert(qual_type);
  }

  StrCat(BoxType(Converter::ToStringBase(qual_type)));
  return false;
}

bool ConverterRefCount::VisitIncompleteArrayType(
    clang::IncompleteArrayType *type) {
  std::string str;
  {
    PushUnboxedIfSimple push(*this, "Box<%>", type->getElementType());
    str = std::format("Box<[{}]>", ToString(type->getElementType()));
  }
  StrCat(BoxType(std::move(str)));
  return false;
}

bool ConverterRefCount::VisitLValueReferenceType(
    clang::LValueReferenceType *type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  StrCat("Ptr<");
  Convert(type->getPointeeType());
  StrCat(token::kGt);
  return false;
}

std::string ConverterRefCount::BuildFnAdapter(
    const clang::FunctionDecl *src_fn,
    const clang::FunctionProtoType *src_proto,
    const clang::FunctionProtoType *target_proto) {

  // UB: Incompatible arity
  if (src_proto->getNumParams() != target_proto->getNumParams()) {
    return "None";
  }

  PushConversionKind push(*this, ConversionKind::Unboxed);

  // Build adapter signature: |a0: T0, a1: T1, ...| -> Tr
  std::string closure = "(|";
  for (unsigned i = 0; i < target_proto->getNumParams(); ++i) {
    closure +=
        std::format("a{}: {},", i, ToString(target_proto->getParamType(i)));
  }
  closure += "|";
  if (!target_proto->getReturnType()->isVoidType()) {
    closure += std::format(" -> {} ", ToString(target_proto->getReturnType()));
  }
  closure += "{ ";

  // Build adapter body: src_fn(convert(a0), convert(a1), ...)
  closure += GetNamedDeclAsString(src_fn) + "(";
  for (unsigned i = 0; i < src_proto->getNumParams(); ++i) {
    auto src_pty = src_proto->getParamType(i);
    auto tgt_pty = target_proto->getParamType(i);
    if (ToString(src_pty) == ToString(tgt_pty)) {
      closure += std::format("a{}", i);
    } else if (src_pty->isPointerType() && tgt_pty->isPointerType()) {
      if (tgt_pty->isVoidPointerType()) {
        closure += std::format("a{}.cast::<{}>().unwrap()", i,
                               ToString(src_pty->getPointeeType()));
      } else if (src_pty->isVoidPointerType()) {
        closure += std::format("a{}.to_any()", i);
      } else if (tgt_pty->getPointeeType()->isCharType()) {
        closure += std::format("a{}.reinterpret_cast::<{}>()", i,
                               ToString(src_pty->getPointeeType()));
      } else if (src_pty->getPointeeType()->isCharType()) {
        closure += std::format("a{}.reinterpret_cast::<u8>()", i);
      }
    } else {
      // UB: Incompatible types
      return "None";
    }
    closure += ", ";
  }
  closure += ") })";

  return std::format("Some({} as {})", closure,
                     ConvertFunctionPointerType(target_proto));
}

std::string ConverterRefCount::ConvertFunctionPointerType(
    const clang::FunctionProtoType *proto, FnProtoType kind) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::ConvertFunctionPointerType(proto, kind);
}

bool ConverterRefCount::VisitPointerType(clang::PointerType *type) {
  if (auto proto = type->getPointeeType()->getAs<clang::FunctionProtoType>()) {
    StrCat(std::format("FnPtr<{}>", ConvertFunctionPointerType(proto)));
    return false;
  }

  if (IsVaListType(ctx_, clang::QualType(type, 0))) {
    StrCat("VaList");
    return false;
  }

  if (type->isVoidPointerType()) {
    StrCat("AnyPtr");
    return false;
  }

  auto pointee_type = type->getPointeeType();
  PushConversionKind push1(*this, ConversionKind::Ptr,
                           !pointee_type->isArrayType());
  PushConversionKind push2(*this, ConversionKind::FullRefCount,
                           pointee_type->isArrayType());
  if (pointee_type->isRecordType() &&
      abstract_structs_.contains(GetID(pointee_type->getAsRecordDecl()))) {
    StrCat("PtrDyn<dyn");
  } else {
    StrCat("Ptr<");
  }
  Convert(pointee_type);
  StrCat(token::kGt);
  return false;
}

bool ConverterRefCount::VisitRecordType(clang::RecordType *type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::VisitRecordType(type);
}

bool ConverterRefCount::VisitConstantArrayType(clang::ConstantArrayType *type) {
  auto conv = getConversionKind();
  PushConversionKind push(*this, ConversionKind::Unboxed);

  switch (conv) {
  case ConversionKind::Unboxed:
    StrCat("[");
    Convert(type->getElementType());
    StrCat(std::format("; {}]", GetNumAsString(type->getSize()).c_str()));
    break;
  case ConversionKind::Ptr:
    Convert(type->getElementType());
    break;
  case ConversionKind::FullRefCount:
    StrCat("Box<[");
    Convert(type->getElementType());
    StrCat("]>");
    break;
  }
  return false;
}

std::string ConverterRefCount::ConvertFreshLValue(clang::Expr *expr) {
  auto str = ConvertLValue(expr);
  if (isFresh()) {
    return str;
  }
  SetFresh();
  return std::format("({}).clone()", std::move(str));
}

std::string ConverterRefCount::ConvertObject(clang::Expr *expr) {
  PushExprKind push(*this, ExprKind::Object);
  auto str = ToString(expr);
  if (expr->getType()->isPointerType()) {
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return std::format("{}.to_strong().as_pointer()", std::move(str));
  }
  return str;
}

std::string ConverterRefCount::ConvertFreshObject(clang::Expr *expr) {
  auto str = ConvertObject(expr);
  if (isFresh()) {
    return str;
  }
  SetFresh();
  return std::format("({}).clone()", std::move(str));
}

std::string ConverterRefCount::ConvertFresh(clang::Expr *expr) {
  auto str = ToString(expr);
  if (isFresh() || expr->getType()->isVoidType() || isVoid()) {
    return str;
  }
  SetFresh();
  return std::format("({}).clone()", std::move(str));
}

std::string ConverterRefCount::ConvertFreshRValue(clang::Expr *expr) {
  auto str = ConvertRValue(expr);
  if (!isFresh() && !expr->getType()->isVoidType()) {
    SetFresh();
    return std::format("({}).clone()", std::move(str));
  }
  SetFresh();
  return str;
}

std::string ConverterRefCount::ConvertFreshPointer(clang::Expr *expr) {
  auto str = ConvertPointer(expr);
  if (isFresh()) {
    return str;
  }
  SetFresh();
  return std::format("({}).clone()", std::move(str));
}

std::pair<std::string, std::string>
ConverterRefCount::MaterializeTemp(const std::string &binding_name,
                                   clang::QualType param_type,
                                   clang::Expr *expr) {
  auto value = ConvertRValue(expr);
  auto type_str = ToStringBase(param_type.getNonReferenceType());
  std::string binding =
      std::format("let {} : Value < {} > = Rc::new(RefCell::new( {} )) ;",
                  binding_name, type_str, value);
  std::string ref = std::format("{}.as_pointer()", binding_name);
  return {binding, ref};
}

std::string ConverterRefCount::ConvertPtrType(clang::QualType type) {
  std::string str;
  // decays into Ptr; remove the outer type Vec<>
  if (IsBoxedType(type)) {
    str = GetInnerType(type);
  } else {
    PushConversionKind push(*this, ConversionKind::Ptr);
    str = ToString(type);
  }
  return std::format("Ptr<{}>", std::move(str));
}

bool ConverterRefCount::VisitArraySubscriptExpr(
    clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  if (base->IgnoreCasts()->getType()->isPointerType()) {
    ConvertPointerSubscript(expr);
  } else {
    if (!base->IgnoreCasts()->getType()->isArrayType()) {
      if (isLValue()) {
        pending_deref_.assert_consumed();
        Buffer buf(*this);
        ConvertArraySubscript(base, expr->getIdx(), expr->getType());
        pending_deref_.set_unchecked(std::move(buf).str(), expr);
        return false;
      }
      StrCat(token::kOpenParen, GetPointerDerefPrefix(expr->getType()));
      ConvertArraySubscript(base, expr->getIdx(), expr->getType());
      StrCat(GetPointerDerefSuffix(expr->getType()), token::kCloseParen);
    } else {
      ConvertArraySubscript(base, expr->getIdx(), expr->getType());
    }
  }
  return false;
}

bool ConverterRefCount::VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
  if (decl_ids_.count(GetID(decl))) {
    return false;
  }
  Converter::VisitCXXRecordDecl(decl);
  return false;
}

void ConverterRefCount::ConvertOrdAndPartialOrdTraits(
    const clang::CXXRecordDecl *decl, const clang::FunctionDecl *op) {
  std::string first_branch, second_branch, first_return, second_return;

  switch (op->getOverloadedOperator()) {
  case clang::OO_Less:
    if (clang::isa<clang::CXXMethodDecl>(op)) {
      first_branch = std::format(
          "self.{}(Rc::new(RefCell::new(other.clone())).as_pointer())",
          GetOverloadedOperator(op));
      second_branch = std::format(
          "other.{}(Rc::new(RefCell::new(self.clone())).as_pointer())",
          GetOverloadedOperator(op));
    } else {
      first_branch =
          std::format("{}(Rc::new(RefCell::new(self.clone())).as_pointer(), "
                      "Rc::new(RefCell::new(other.clone())).as_pointer())",
                      GetOverloadedOperator(op));
      second_branch =
          std::format("{}(Rc::new(RefCell::new(other.clone())).as_pointer(), "
                      "Rc::new(RefCell::new(self.clone())).as_pointer())",
                      GetOverloadedOperator(op));
    }
    first_return = "std::cmp::Ordering::Less";
    second_return = "std::cmp::Ordering::Greater";
    break;
  default:
    assert(0 && "Currently only supporting operator<");
  }

  ConvertOrdAndPartialOrdTraitsBase(first_branch, second_branch, first_return,
                                    second_return, GetRecordName(decl));
}

void ConverterRefCount::AddCloneTrait(const clang::CXXRecordDecl *decl) {
  if (decl->defaultedCopyConstructorIsDeleted()) {
    return;
  }

  auto record_name = GetRecordName(decl);

  StrCat(keyword::kImpl, "Clone for", record_name, "{");
  StrCat("fn clone(&self) -> Self {");

  for (auto ctor : decl->ctors()) {
    if (ctor->isCopyConstructor()) {
      PushConversionKind push(*this, ConversionKind::FullRefCount);
      ConvertCXXConstructorBody(ctor);
      break;
    }
  }

  StrCat("}");
  StrCat("}");
}

void ConverterRefCount::AddDefaultTrait(const clang::CXXRecordDecl *decl) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  Converter::AddDefaultTrait(decl);
}

void ConverterRefCount::AddDropTrait(const clang::CXXRecordDecl *decl) {
  if (!decl->hasUserDeclaredDestructor()) {
    return;
  }

  auto dtor = decl->getDestructor();
  if (!dtor) {
    return;
  }

  auto body = dtor->getBody();
  if (!body) {
    return;
  }

  if (auto stmt = llvm::dyn_cast<clang::CompoundStmt>(body)) {
    if (stmt->body_empty()) {
      return;
    }
  }

  auto record_name = GetRecordName(decl);

  StrCat(keyword::kImpl, "Drop for", record_name, "{");
  StrCat("fn drop(&mut self) {");
  Convert(body);
  StrCat("}");
  StrCat("}");
}

void ConverterRefCount::AddByteReprTrait(const clang::CXXRecordDecl *decl) {
  auto struct_name = GetRecordName(decl);
  StrCat(std::format("impl ByteRepr for {}", struct_name),
         token::kOpenCurlyBracket, token::kCloseCurlyBracket);
}

std::string
ConverterRefCount::GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) {
  return "&self";
}

bool ConverterRefCount::VisitCXXConstructorDecl(
    clang::CXXConstructorDecl *decl) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return Converter::VisitCXXConstructorDecl(decl);
}

bool ConverterRefCount::VisitFieldDecl(clang::FieldDecl *decl) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return Converter::VisitFieldDecl(decl);
}

void ConverterRefCount::EmitFunctionPreamble(clang::FunctionDecl *decl) {
  // In the header, the function might be declared as `int foo(int name_1)',
  // while in the source file the function might be defined as `int foo(int
  // name_2)'. We want to get the parameters from the definition if possible,
  // i.e. name_2.
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  auto params = decl->getDefinition() ? decl->getDefinition()->parameters()
                                      : decl->parameters();
  for (auto *param : params) {
    if (!param->getType()->isReferenceType()) {
      auto name = GetNamedDeclAsString(param);
      auto type = ToString(param->getType());
      auto init = name;

      if (param->hasDefaultArg()) {
        init = std::format("{}.unwrap_or({})", name,
                           ToString(param->getDefaultArg()));
      }

      StrCat(std::format("let {} : {} = Rc::new(RefCell::new({}))", name, type,
                         init),
             token::kSemiColon);
    }
  }
}

void ConverterRefCount::ConvertVaListVarDecl(clang::VarDecl *decl) {
  if (clang::isa<clang::ParmVarDecl>(decl)) {
    // va_list parameter (decayed to __va_list_tag *)
  } else {
    // va_list local variable
    StrCat(keyword::kLet);
  }

  StrCat(GetNamedDeclAsString(decl), token::kColon, "Value<VaList>");
}

bool ConverterRefCount::ConvertLambdaVarDecl(clang::VarDecl *decl) {
  return false;
}

void ConverterRefCount::ConvertGlobalVarDecl(clang::VarDecl *decl) {
  StrCat("thread_local!", token::kOpenParen);
  ConvertVarDecl(decl);
  StrCat(token::kCloseParen, token::kSemiColon);
}

bool ConverterRefCount::VisitVarDecl(clang::VarDecl *decl) {
  bool unboxed = in_function_formals_;
  PushConversionKind push(*this, unboxed ? ConversionKind::Unboxed
                                         : ConversionKind::FullRefCount);
  if (decl->getType()->isReferenceType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    Converter::VisitVarDecl(decl);
  } else {
    Converter::VisitVarDecl(decl);
  }
  return false;
}

bool ConverterRefCount::ConvertIncAndDec(clang::UnaryOperator *expr) {
  auto opcode = expr->getOpcode();
  auto *sub_expr = expr->getSubExpr();

  const char *method = nullptr;
  switch (opcode) {
  case clang::UO_PostInc:
    method = "postfix_inc";
    break;
  case clang::UO_PostDec:
    method = "postfix_dec";
    break;
  case clang::UO_PreInc:
    method = "prefix_inc";
    break;
  case clang::UO_PreDec:
    method = "prefix_dec";
    break;
  default:
    return false;
  }

  auto str = ConvertLValue(sub_expr);
  if (!pending_deref_.empty()) {
    StrCat(pending_deref_.take(), ".with_mut(|__v| __v.", method, "())");
  } else {
    StrCat(str, ".", method, "()");
  }
  return true;
}

bool ConverterRefCount::VisitConditionalOperator(
    clang::ConditionalOperator *expr) {
  StrCat(keyword::kIf, ConvertRValue(expr->getCond()), token::kOpenCurlyBracket,
         ConvertFresh(expr->getTrueExpr()), token::kCloseCurlyBracket,
         keyword::kElse, token::kOpenCurlyBracket,
         ConvertFresh(expr->getFalseExpr()), token::kCloseCurlyBracket);
  return false;
}

bool ConverterRefCount::VisitDeclRefExpr(clang::DeclRefExpr *expr) {
  if (isAddrOf()) {
    clang::Expr *addrof_op = ToAddrOf(ctx_, expr);
    if (Mapper::Contains(addrof_op)) {
      StrCat(GetMappedAsString(addrof_op));
      return false;
    }
  }

  if (Mapper::Contains(expr)) {
    StrCat(GetMappedAsString(expr));
    return false;
  }

  auto str = ConvertDeclRefExpr(expr);
  auto decl = expr->getDecl();

  if (auto fn_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (isAddrOf()) {
      ConvertFunctionToFunctionPointer(fn_decl);
    } else {
      StrCat(str);
    }
    return false;
  }

  if (clang::isa<clang::EnumConstantDecl>(decl)) {
    StrCat(str);
    return false;
  }

  if (IsGlobalVar(expr)) {
    str = std::format("{}.with(Value::clone)", str);
  }

  if (auto *ref = decl->getType()->getAs<clang::ReferenceType>()) {
    if (map_iter_decls_.contains(clang::dyn_cast<clang::VarDecl>(decl))) {
      StrCat(str);
      return false;
    }

    // std::vector<T>& gets converted to Ptr<vec<T>>
    // So we need to make a pointer to the vector itself
    if (isObject()) {
      if (IsBoxedType(ref->getPointeeType())) {
        StrCat(str, ".to_strong().as_pointer()");
        computed_expr_type_ = ComputedExprType::FreshPointer;
        return false;
      }
    }

    // references are not boxed
    if (isAddrOf()) {
      StrCat(str);
      computed_expr_type_ = ComputedExprType::Pointer;
    } else {
      if (str == "self") {
        StrCat(str);
      } else {
        if (isLValue()) {
          pending_deref_.set(str);
          return false;
        }
        StrCat(DerefPtrExpr(str, ref->getPointeeType()));
      }
      SetValueFreshness(expr->getType());
    }
    return false;
  }

  if (isAddrOf()) {
    StrCat(str, ".as_pointer()");
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return false;
  }

  if (isRValue()) {
    StrCat(std::format("(*{}.borrow())", std::move(str)));
  } else {
    StrCat(std::format("(*{}.borrow_mut())", std::move(str)));
  }

  if (auto *decl = clang::dyn_cast<clang::VarDecl>(expr->getDecl())) {
    if (decl->getType()->isPointerType()) {
      computed_expr_type_ = ComputedExprType::Pointer;
      return false;
    }
  }
  SetValueFreshness(expr->getType());
  return false;
}

static std::vector<const char *> printf2fmt(std::string &format) {
  std::vector<const char *> types;
  size_t pos = 0;
  while ((pos = format.find("%", pos)) != std::string::npos) {
    if (pos + 1 >= format.size())
      break;

    switch (auto c = format[pos + 1]) {
    case 'c':
      types.emplace_back("u8 as char");
      format.replace(pos, 2, "{}");
      pos += 2;
      continue;
    case 'd':
    case 'i':
    case 's':
    case 'u':
      types.emplace_back();
      format.replace(pos, 2, "{}");
      pos += 2;
      continue;
    case 'p':
      types.emplace_back();
      format.replace(pos, 2, "{:?}");
      pos += 2;
      continue;
    case '%':
      types.emplace_back();
      format.replace(pos, 2, "%");
      pos += 2;
      continue;
    case 'l':
      if (pos + 2 < format.size() &&
          (format[pos + 2] == 'd' || format[pos + 2] == 'u')) {
        types.emplace_back();
        format.replace(pos, 3, "{}");
        pos += 2;
        continue;
      }
      if (pos + 3 < format.size() && format[pos + 2] == 'l' &&
          (format[pos + 3] == 'd' || format[pos + 3] == 'u')) {
        types.emplace_back();
        format.replace(pos, 4, "{}");
        pos += 2;
        continue;
      }
      break;
    case 'z':
      if (pos + 2 < format.size() &&
          (format[pos + 2] == 'd' || format[pos + 2] == 'u')) {
        types.emplace_back();
        format.replace(pos, 3, "{}");
        pos += 2;
        continue;
      }
      break;
    case '.':
      if (pos + 3 < format.size() && format[pos + 2] == '0') {
        auto end = format.find_first_not_of("0123456789", pos + 3);
        if (end != std::string::npos && format[end] == 'f') {
          auto repl = "{:." + format.substr(pos + 3, end - pos - 3) + '}';
          format.replace(pos, end - pos + 1, repl);
          pos += repl.size();
          types.emplace_back();
          continue;
        }
      }
      break;
    default:
      if (c >= '0' && c <= '9') {
        auto end = format.find_first_not_of("0123456789", pos + 2);
        if (end != std::string::npos) {
          auto repl = "{:" + format.substr(pos + 1, end - pos - 1);
          bool ok = true;
          switch (c = format[end]) {
          case 'd':
            break;
          case 'x':
            repl += c;
            break;
          case 'z':
            if (end + 1 < format.size() && format[end + 1] == 'u') {
              ++end;
            } else {
              ok = false;
            }
            break;
          default:
            ok = false;
            break;
          }
          if (ok) {
            repl += '}';
            format.replace(pos, end - pos + 1, repl);
            pos += repl.size();
            types.emplace_back();
            continue;
          }
        }
      }
    }
    llvm::errs() << "Unknown printf format: " << format << "\n";
    assert(0);
  }
  return types;
}

void ConverterRefCount::ConvertPrintf(clang::CallExpr *expr) {
  bool is_fprintf =
      Mapper::ToString(expr->getCallee()).starts_with("int fprintf");
  std::string format;
  if (auto *str = clang::dyn_cast<clang::StringLiteral>(
          expr->getArg(is_fprintf)->IgnoreImplicit())) {
    format = GetEscapedStringLiteral(str);
  } else {
    llvm::errs() << "Uknown fprintf format: ";
    expr->getArg(1)->dump();
    llvm::errs() << "\n";
    exit(1);
  }
  bool ends_newline = format.ends_with("\\n\"");

  auto fd = is_fprintf ? Mapper::ToString(expr->getArg(0)) : "stdout";
  if (fd == "stdout" || fd == "__stdoutp") {
    StrCat(ends_newline ? "println!(" : "print!(");
  } else if (fd == "stderr" || fd == "__stderrp") {
    StrCat(ends_newline ? "eprintln!(" : "eprint!(");
  } else {
    llvm::errs() << "Uknown fprintf fd: " << fd << "\n";
    exit(1);
  }
  if (ends_newline) {
    format.replace(format.size() - 3, 2, "");
  }
  auto types = printf2fmt(format);
  StrCat(format);

  unsigned j = 0;
  for (unsigned i = is_fprintf + 1, e = expr->getNumArgs(); i < e; ++i) {
    StrCat(token::kComma);
    Convert(expr->getArg(i));
    if (types[j])
      StrCat(keyword::kAs, types[j++]);
  }
  StrCat(")");
}

bool ConverterRefCount::VisitCallExpr(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr) || IsBuiltinVaEnd(expr) || IsBuiltinVaCopy(expr)) {
    ConvertVAArgCall(expr);
    return false;
  }

  if (expr->isCallToStdMove()) {
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      StrCat(std::format("{}.take()", ConvertLValue(expr->getArg(0))));
    } else {
      PushExprKind push(*this, ExprKind::XValue);
      Convert(expr->getArg(0));
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
    return false;
  }

  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
      opcall && !Mapper::Contains(expr->getCallee())) {
    return ConvertCXXOperatorCallExpr(opcall);
  }

  std::optional<TempMaterializationCtx> ctx;
  std::string str;
  if (auto plugin_str = TryPluginConvert(expr)) {
    StrCat(*plugin_str);
    return false;
  } else {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    Buffer buf(*this);
    ctx = Converter::ConvertCallExpr(expr);
    str = std::move(buf).str();
  }

  auto ty = GetReturnTypeOfFunction(expr);
  auto ref = clang::dyn_cast<clang::ReferenceType>(ty);

  if (ref && !isAddrOf() && !isVoid()) {
    if (isLValue()) {
      if (ctx && !ctx->temporary_bindings.empty()) {
        str = std::format("{{ {} {} }}", ctx->temporary_bindings, str);
      }
      pending_deref_.set(str);
      return false;
    }
    // Apply deref before block wrapping so temporaries are still alive.
    str = DerefPtrExpr(str, ref->getPointeeType());
    if (ctx && !ctx->temporary_bindings.empty()) {
      str = std::format("{{ {} {} }}", ctx->temporary_bindings, str);
    }
    StrCat(str);
    SetValueFreshness(ref->getPointeeType());
    return false;
  }

  if (isAddrOf() && !ty->isReferenceType() && !IsPointerType(ty)) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    StrCat(BoxValue(std::move(str)), ".as_pointer()");
    return false;
  }

  if (isObject()) {
    StrCat(std::format("{}.to_strong().as_pointer()", std::move(str)));
    return false;
  }

  if (ctx && !ctx->temporary_bindings.empty()) {
    str = std::format("({{ {} {} }})", ctx->temporary_bindings, str);
  }
  StrCat(str);
  if (IsPointerType(ty) || ty->isReferenceType()) {
    computed_expr_type_ = ComputedExprType::FreshPointer;
  } else {
    computed_expr_type_ = ComputedExprType::FreshValue;
  }
  return false;
}

bool ConverterRefCount::VisitStringLiteral(clang::StringLiteral *expr) {
  StrCat(GetEscapedStringLiteral(expr));
  return false;
}

bool ConverterRefCount::VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
  auto *sub_expr = expr->getSubExpr();

  // return unique_ptr (implicit xvalue cast)
  if (expr->isXValue() && sub_expr->isLValue()) {
    Convert(sub_expr);
    if (IsUniquePtr(sub_expr->getType())) {
      StrCat(".take()");
      computed_expr_type_ = ComputedExprType::FreshValue;
    } else {
      computed_expr_type_ = ComputedExprType::Value;
    }
    return false;
  }

  if (auto *unary = clang::dyn_cast<clang::UnaryOperator>(sub_expr);
      expr->getCastKind() == clang::CastKind::CK_LValueToRValue && unary &&
      (unary->isPostfix() || unary->isPrefix())) {
    return Convert(sub_expr);
  }

  if (expr->getCastKind() == clang::CastKind::CK_BitCast) {
    if (expr->getType()->isVoidPointerType()) {
      PushConversionKind push(*this, ConversionKind::Unboxed);
      if (sub_expr->getType()->isPointerType() &&
          sub_expr->getType()->getPointeeType()->isArrayType()) {
        StrCat(std::format("({} as Ptr<{}>).to_any()",
                           ConvertFreshPointer(sub_expr),
                           ToString(sub_expr->getType()
                                        ->getPointeeType()
                                        ->getAsArrayTypeUnsafe()
                                        ->getElementType())));
      } else {
        StrCat(std::format("({} as {}).to_any()", ConvertFreshPointer(sub_expr),
                           ToString(sub_expr->getType())));
      }
      computed_expr_type_ = ComputedExprType::FreshPointer;
    } else {
      Convert(sub_expr);
    }
    return false;
  }

  if (expr->getCastKind() == clang::CastKind::CK_DerivedToBase) {
    if (expr->getType()->isPointerType()) {
      auto ptype = clang::dyn_cast<clang::PointerType>(expr->getType());
      auto pointee_type = ptype->getPointeeType()->getAsCXXRecordDecl();

      if (pointee_type && abstract_structs_.contains(GetID(pointee_type))) {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        StrCat(std::format("({}.to_strong() as Value<dyn {}>).as_pointer_dyn()",
                           ToString(sub_expr->IgnoreCasts()),
                           ToString(expr->getType()->getPointeeType())));
        computed_expr_type_ = ComputedExprType::FreshPointer;
        return false;
      }
    }
  }

  if (expr->getCastKind() == clang::CastKind::CK_ArrayToPointerDecay) {
    if (IsVaListType(ctx_, sub_expr->getType())) {
      Convert(sub_expr);
      return false;
    }
    if (clang::isa<clang::StringLiteral>(sub_expr) ||
        clang::isa<clang::PredefinedExpr>(sub_expr)) {
      StrCat(std::format("Ptr::from_string_literal({})", ToString(sub_expr)));
      return false;
    } else {
      // we need to write (var.as_pointer as Ptr<T>) because Rust isn't
      // smart enough to pick the right specialization
      PushConversionKind push(*this, ConversionKind::Unboxed);
      StrCat(token::kOpenParen, ConvertPointer(sub_expr), keyword::kAs,
             ToString(expr->getType()), token::kCloseParen);
      return false;
    }
  }

  if (expr->getCastKind() == clang::CastKind::CK_NullToPointer &&
      expr->getType()->isFunctionPointerType()) {
    StrCat("FnPtr::null()");
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return false;
  }

  return Converter::VisitImplicitCastExpr(expr);
}

void ConverterRefCount::EmitFnPtrCall(clang::Expr *callee) {
  StrCat("(*");
  Convert(callee);
  StrCat(")");
}

void ConverterRefCount::ConvertFunctionToFunctionPointer(
    const clang::FunctionDecl *fn_decl) {
  StrCat(
      std::format("fn_ptr!({}, {})", GetNamedDeclAsString(fn_decl),
                  ConvertFunctionPointerType(
                      fn_decl->getType()->getAs<clang::FunctionProtoType>())));
}

void ConverterRefCount::ConvertEqualsNullPtr(clang::Expr *expr) {
  StrCat("(");
  Convert(expr);
  StrCat(").is_null()");
}

bool ConverterRefCount::VisitFunctionPointerCast(
    clang::ExplicitCastExpr *expr) {
  if (expr->getType()->isFunctionPointerType() ||
      expr->getSubExpr()->getType()->isFunctionPointerType()) {
    if (expr->getSubExpr()->getType()->isFunctionPointerType() &&
        expr->getType()->isFunctionPointerType()) {
      auto target_proto =
          expr->getType()->getPointeeType()->getAs<clang::FunctionProtoType>();
      auto src_proto = expr->getSubExpr()
                           ->getType()
                           ->getPointeeType()
                           ->getAs<clang::FunctionProtoType>();
      auto fn_type = ConvertFunctionPointerType(target_proto);

      std::string adapter = "None";
      // Only accept direct references to the casted function. Otherwise the
      // closure would be capturing and would not coerce into a fn pointer.
      if (auto *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(
              expr->getSubExpr()->IgnoreImplicit())) {
        if (auto *fn_decl =
                clang::dyn_cast<clang::FunctionDecl>(decl_ref->getDecl())) {
          adapter = BuildFnAdapter(fn_decl, src_proto, target_proto);
        }
      }

      StrCat(std::format("{}.cast::<{}>({})", ToString(expr->getSubExpr()),
                         fn_type, adapter));
    } else if (expr->getSubExpr()->getType()->isFunctionPointerType() ||
               expr->getType()->isVoidPointerType()) {
      Convert(expr->getSubExpr());
      StrCat(".to_any()");
    } else if (expr->getSubExpr()->getType()->isVoidPointerType() ||
               expr->getType()->isFunctionPointerType()) {
      auto target_proto =
          expr->getType()->getPointeeType()->getAs<clang::FunctionProtoType>();
      auto fn_type = ConvertFunctionPointerType(target_proto);
      StrCat(std::format("{}.cast_fn::<{}>().expect(\"ub:wrong fn type\")",
                         ToString(expr->getSubExpr()), fn_type));
    } else {
      assert(0 && "Unhandled function pointer cast");
    }
    return false;
  }

  return true;
}

bool ConverterRefCount::VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
  if (expr->getTypeAsWritten()->isVoidType()) {
    return false;
  }
  switch (expr->getStmtClass()) {
  case clang::Stmt::CXXReinterpretCastExprClass:
    assert(expr->getType()->isPointerType() &&
           "Only pointer casts are supported in reinterpret_cast");
    StrCat(
        std::format("{}.reinterpret_cast::<{}>()", ToString(expr->getSubExpr()),
                    GetUnsafeTypeAsString(expr->getType()->getPointeeType())));
    return false;
  case clang::Stmt::CStyleCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass:
    if (!VisitFunctionPointerCast(expr)) {
      return false;
    } else if (expr->getSubExpr()->getType()->isVoidPointerType()) {
      Convert(expr->getSubExpr());
      PushConversionKind push(*this, ConversionKind::Unboxed);
      StrCat(std::format(".cast::<{}>().expect(\"ub:wrong type\")",
                         ToString(expr->getType()->getPointeeType())));
      return false;
    } else if (expr->getSubExpr()->getType()->isPointerType() &&
               !expr->getSubExpr()->isNullPointerConstant(
                   ctx_, clang::Expr::NPC_ValueDependentIsNull)) {
      StrCat(std::format("({}.to_strong().as_pointer() as {})",
                         ToString(expr->getSubExpr()),
                         ToString(expr->getType())));
      return false;
    }
    return Converter::VisitExplicitCastExpr(expr);
  default:
    return Convert(expr->getSubExpr());
  }
}

bool ConverterRefCount::VisitBinaryOperator(clang::BinaryOperator *expr) {
  auto *lhs = expr->getLHS();
  auto *rhs = expr->getRHS();
  auto lhs_type = lhs->getType();
  auto rhs_type = rhs->getType();
  std::string opcode_as_string = expr->getOpcodeStr().str();

  if (auto *assign = llvm::dyn_cast<clang::CompoundAssignOperator>(expr);
      assign && lhs_type != assign->getComputationResultType()) {
    auto computation_result_type = assign->getComputationResultType();
    StrCat(keyword::kLet, "rhs_0", token::kAssign);
    if (IsUnsignedArithOp(assign)) {
      StrCat(token::kOpenParen, token::kOpenParen);
      StrCat(ConvertRValue(lhs));
      ConvertCast(computation_result_type);
      StrCat(token::kCloseParen);
      ConvertUnsignedArithBinaryOperator(expr, rhs);
      StrCat(token::kCloseParen);
    } else {
      StrCat(token::kOpenParen);
      StrCat(token::kOpenParen);
      StrCat(ConvertRValue(lhs));
      ConvertCast(computation_result_type);
      StrCat(token::kCloseParen);
      StrCat(std::regex_replace(opcode_as_string, std::regex("="), ""));
      Convert(rhs);
      StrCat(token::kCloseParen);
    }
    if (lhs_type->isBooleanType()) {
      StrCat(token::kDiff, token::kZero);
    } else {
      ConvertCast(lhs_type);
    }
    StrCat(token::kSemiColon);
    EmitSetOrAssign(lhs, "rhs_0");
    return false;
  }

  if (IsUnsignedArithOp(expr)) {
    if (expr->isCompoundAssignmentOp()) {
      StrCat(keyword::kLet, "rhs_0", token::kAssign);
    }
    StrCat(token::kOpenParen);
    if (expr->isCompoundAssignmentOp() && lhs->isLValue()) {
      StrCat(ConvertRValue(lhs));
    } else {
      ConvertUnsignedArithOperand(lhs, expr->getType());
    }
    StrCat(token::kCloseParen);
    ConvertUnsignedArithBinaryOperator(expr, rhs);
    if (expr->isCompoundAssignmentOp()) {
      StrCat(token::kSemiColon);
      EmitSetOrAssign(lhs, "rhs_0");
    }
    return false;
  }

  // pointer subtraction. The Sub trait gets elements by Value, so we need
  // fresh pointers
  if (expr->isAdditiveOp() && lhs_type->isPointerType() &&
      rhs_type->isPointerType()) {
    StrCat(ConvertFreshPointer(lhs), expr->getOpcodeStr(),
           ConvertFreshPointer(rhs));
    computed_expr_type_ = ComputedExprType::FreshValue;
    return false;
  }

  if (expr->isAssignmentOp()) {
    ConvertAssignment(lhs, rhs, opcode_as_string);
    return false;
  }

  return Converter::VisitBinaryOperator(expr);
}

bool ConverterRefCount::VisitInitListExpr(clang::InitListExpr *expr) {
  if (auto form = expr->getSemanticForm())
    expr = form;

  auto qual_type = expr->getType();
  if (qual_type->isScalarType()) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    Converter::VisitInitListExpr(expr);
    computed_expr_type_ = ComputedExprType::FreshValue;
    return false;
  }

  if (qual_type->isRecordType()) {
    const auto *record = qual_type->getAsRecordDecl();
    if (record->getQualifiedNameAsString() == "std::array") {
      StrCat("vec!");
      if (auto init = clang::dyn_cast<clang::InitListExpr>(expr->getInit(0))) {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        ConverterRefCount::VisitInitListExpr(init);
      } else {
        StrCat("[]");
      }
      computed_expr_type_ = ComputedExprType::FreshValue;
      return false;
    }

    StrCat(GetUnsafeTypeAsString(qual_type), token::kOpenCurlyBracket);
    int i = 0;
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    for (const auto *field : record->fields()) {
      StrCat(GetNamedDeclAsString(field), token::kColon);
      ConvertVarInit(field->getType(), expr->getInit(i++));
      StrCat(token::kComma);
    }
    StrCat(token::kCloseCurlyBracket);
    computed_expr_type_ = ComputedExprType::FreshValue;
    return false;
  }

  auto conv = getConversionKind();
  // 2D arrays are FullRefCount'ed on the second level as well.
  PushConversionKind push(
      *this, ConversionKind::Unboxed,
      !(expr->getNumInits() > 0 && expr->getInit(0)->getType()->isArrayType()));

  switch (conv) {
  case ConversionKind::Unboxed:
  case ConversionKind::Ptr:
    Converter::VisitInitListExpr(expr);
    break;
  case ConversionKind::FullRefCount:
    StrCat("Box::new(");
    Converter::VisitInitListExpr(expr);
    StrCat(')');
    break;
  }
  computed_expr_type_ = ComputedExprType::FreshValue;
  return false;
}

bool ConverterRefCount::VisitMemberExpr(clang::MemberExpr *expr) {
  auto *member = expr->getMemberDecl();
  bool known = Mapper::Contains(expr);

  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && !known) {
    // User-defined types have Value<T> fields; the struct itself is read-only
    // and only needs an immutable borrow. Non-user-defined types (STL)
    // need a mutable borrow for non-const methods
    auto base_type = expr->getBase()->getType().getNonReferenceType();
    if (base_type->isPointerType()) {
      base_type = base_type->getPointeeType();
    }
    bool needs_mut = NeedsMutAccess(method, base_type);
    PushExprKind push(*this, needs_mut ? ExprKind::LValue : ExprKind::RValue);
    Converter::ConvertMemberExpr(expr);
    return false;
  }

  std::string str;
  if (known) {
    str = GetMappedAsString(expr);
  } else {
    Buffer buf(*this);
    PushExprKind push(*this, ExprKind::RValue);
    Converter::ConvertMemberExpr(expr);
    str = std::move(buf).str();
  }

  if (isAddrOf()) {
    StrCat(str);
    if (member->getType()->isReferenceType()) {
      computed_expr_type_ = ComputedExprType::Pointer;
    } else {
      StrCat(".as_pointer()");
      computed_expr_type_ = ComputedExprType::FreshPointer;
    }
    return false;
  }

  if (member->getType()->isReferenceType()) {
    if (isLValue()) {
      pending_deref_.set(str);
      return false;
    }
    StrCat(DerefPtrExpr(str, member->getType().getNonReferenceType()));
  } else if (isRValue()) {
    StrCat(std::format("(*{}.borrow())", std::move(str)));
  } else {
    StrCat(std::format("(*{}.borrow_mut())", std::move(str)));
  }
  SetValueFreshness(expr->getType());
  return false;
}

bool ConverterRefCount::VisitCXXNewExpr(clang::CXXNewExpr *expr) {
  if (expr->isArray()) {
    if (auto *init = llvm::dyn_cast_or_null<clang::InitListExpr>(
            expr->getInitializer())) {
      StrCat("Ptr::alloc_array(");
      Convert(init);
      StrCat(")");
    } else {
      auto array_size_as_string = ToString(*expr->getArraySize());
      auto alloc_type = expr->getAllocatedType();
      PushConversionKind push(*this, ConversionKind::Unboxed);
      auto alloc_type_as_string = ToString(alloc_type);
      auto default_alloc_type_as_string = GetDefaultAsString(alloc_type);

      StrCat(std::format(
          "Ptr::alloc_array((0..{}).map(|_| {}).collect::<Box<[{}]>>())",
          array_size_as_string, default_alloc_type_as_string,
          alloc_type_as_string));
    }
  } else {
    StrCat("Ptr::alloc(");
    if (expr->getInitializer() == nullptr) {
      StrCat("Default::default()");
    } else {
      Convert(expr->getInitializer());
    }
    StrCat(")");
  }
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return false;
}

bool ConverterRefCount::VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) {
  Convert(expr->getArgument());
  if (expr->isArrayForm()) {
    StrCat(".delete_array()");
  } else {
    StrCat(".delete()");
  }
  return false;
}

void ConverterRefCount::EmitByValueShadow(const std::string &loop_var_name,
                                          clang::QualType type,
                                          std::string box_expr,
                                          const std::string &type_override) {
  if (!type->isReferenceType()) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    auto type_str = type_override.empty() ? ToString(type) : type_override;
    StrCat(keyword::kLet, loop_var_name, token::kColon, type_str,
           token::kAssign);
    StrCat(BoxValue(std::move(box_expr)), token::kSemiColon);
  }
}

bool ConverterRefCount::VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  StrCat("'loop_:");
  StrCat(keyword::kFor, loop_var_name, keyword::kIn, "RefcountMapIter::begin(",
         ConvertObject(stmt->getRangeInit()), ")");
  StrCat(token::kOpenCurlyBracket);

  EmitByValueShadow(
      loop_var_name, loop_var->getType(), std::string(loop_var_name),
      "Value<" + Mapper::Map(GetForRangeIteratorType(stmt)) + ">");

  ConvertForRangeBody(stmt, loop_var);
  StrCat(token::kCloseCurlyBracket);

  return false;
}

bool ConverterRefCount::VisitCXXForRangeStmtVector(
    clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  StrCat("'loop_:");
  StrCat(keyword::kFor,
         stmt->getLoopVariable()->getType().isConstQualified() ? "" : "mut",
         loop_var_name, keyword::kIn, ConvertObject(stmt->getRangeInit()));
  StrCat(keyword::kAs, ConvertPtrType(stmt->getRangeInit()->getType()));

  StrCat(token::kOpenCurlyBracket);

  // handle multi-level types such as Vec<Value<Vec<T>>>
  if (IsBoxedType(stmt->getRangeInit()->getType()) &&
      GetInnerType(stmt->getRangeInit()->getType()).starts_with("Value<")) {
    StrCat(keyword::kLet, loop_var_name, token::kColon);

    if (loop_var->getType()->isReferenceType()) {
      StrCat(ToString(loop_var->getType()), token::kAssign, loop_var_name,
             GetPointerDerefSuffix(loop_var->getType().getNonReferenceType()),
             ".as_pointer()");
    } else {
      PushConversionKind push(*this, ConversionKind::FullRefCount);
      StrCat(ToString(loop_var->getType()), token::kAssign,
             "Rc::new(RefCell::new(", loop_var_name,
             GetPointerDerefSuffix(loop_var->getType()), ".borrow().clone()))");
    }
    StrCat(token::kSemiColon);
  } else {
    EmitByValueShadow(loop_var_name, loop_var->getType(),
                      loop_var_name +
                          GetPointerDerefSuffix(loop_var->getType()) +
                          ".clone()");
  }

  ConvertForRangeBody(stmt);
  StrCat(token::kCloseCurlyBracket);

  return false;
}

bool ConverterRefCount::VisitCXXForRangeStmtString(
    clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  StrCat("'loop_:");
  StrCat(keyword::kFor,
         stmt->getLoopVariable()->getType().isConstQualified() ? "" : "mut",
         loop_var_name, keyword::kIn, ConvertObject(stmt->getRangeInit()));
  StrCat(".to_string_iterator() as StringIterator<",
         ToString(loop_var->getType().getNonReferenceType()), ">");

  StrCat(token::kOpenCurlyBracket);

  EmitByValueShadow(loop_var_name, loop_var->getType(),
                    loop_var_name + GetPointerDerefSuffix(loop_var->getType()) +
                        ".clone()");
  ConvertForRangeBody(stmt);
  StrCat(token::kCloseCurlyBracket);

  return false;
}

void ConverterRefCount::ConvertArrayCXXConstructExpr(
    clang::CXXConstructExpr *expr) {
  StrCat(std::format("Box::new(std::array::from_fn::<_, {}, _>(|_|",
                     GetArraySize(expr->getType())));
  ConvertCXXConstructExprArgs(expr);
  StrCat(token::kCloseParen, token::kCloseParen);
}

std::string ConverterRefCount::ConvertStream(clang::Expr *expr) {
  return ConvertPointer(expr);
}

bool ConverterRefCount::VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);

  if (Mapper::Contains(expr)) {
    auto str = GetMappedAsString(expr, expr->getArgs(), expr->getNumArgs());
    if (isAddrOf()) {
      StrCat(std::format("Rc::new(RefCell::new({})).as_pointer()",
                         std::move(str)));
      computed_expr_type_ = ComputedExprType::FreshPointer;
    } else {
      StrCat(str);
      computed_expr_type_ = ComputedExprType::FreshValue;
    }
    return false;
  }

  auto *ctor = expr->getConstructor();
  if (ctor->isMoveConstructor() ||
      (ctor->isConvertingConstructor(false) && ctor->getNumParams() == 1 &&
       ctor->getParamDecl(0)->getType()->isRValueReferenceType())) {
    StrCat(ConvertLValue(expr->getArg(0)));
    return false;
  }

  if (ctor->isCopyConstructor()) {
    StrCat(ConvertRValue(expr->getArg(0)));
    return false;
  }

  if (ctor->isDefaultConstructor() && !ctor->isUserProvided()) {
    auto ty = expr->getType();
    StrCat(GetDefaultAsString(ty));
    SetFreshType(ty);
    return false;
  }

  assert(ctor->isUserProvided());
  if (expr->getType()->isArrayType()) {
    ConvertArrayCXXConstructExpr(expr);
  } else {
    ConvertCXXConstructExprArgs(expr);
  }
  SetFreshType(expr->getType());

  return false;
}

bool ConverterRefCount::VisitImplicitValueInitExpr(
    clang::ImplicitValueInitExpr *expr) {
  if (auto arr_ty = clang::dyn_cast<clang::ArrayType>(
          expr->getType()->getCanonicalTypeInternal().getTypePtr())) {
    if (clang::isa<clang::ConstantArrayType>(arr_ty)) {
      StrCat("Box::new(");
      Converter::VisitImplicitValueInitExpr(expr);
      StrCat(")");
      return false;
    }
  }

  return Converter::VisitImplicitValueInitExpr(expr);
}

bool ConverterRefCount::VisitVAArgExpr(clang::VAArgExpr *expr) {
  auto va_list_expr = expr->getSubExpr();
  if (auto *cast = clang::dyn_cast<clang::ImplicitCastExpr>(va_list_expr)) {
    va_list_expr = cast->getSubExpr();
  }
  StrCat(ConvertLValue(va_list_expr));
  StrCat(".arg::<");
  StrCat(GetUnsafeTypeAsString(expr->getType()));
  StrCat(">()");
  return false;
}

bool ConverterRefCount::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) {
  return Converter::VisitCXXDefaultArgExpr(expr);
}

std::string ConverterRefCount::GetDefaultAsString(clang::QualType qual_type) {
  std::string ret;
  if (qual_type->isPointerType()) {
    auto pointee_type = qual_type->getPointeeType();
    if (pointee_type->isFunctionType()) {
      ret = "FnPtr::null()";
    } else {
      if (pointee_type->isVoidType()) {
        ret = "AnyPtr::default()";
      } else {
        ret = std::format("Ptr::<{}>::null()",
                          GetUnsafeTypeAsString(pointee_type));
      }
    }
  } else if (auto *array_type =
                 clang::dyn_cast<clang::ConstantArrayType>(qual_type)) {
    const auto &size = array_type->getSize();
    auto size_as_string = GetNumAsString(size);
    auto element_type = array_type->getElementType();
    PushConversionKind push(*this, ConversionKind::Unboxed);
    auto element_type_as_string = ToString(element_type);
    auto default_as_string = GetDefaultAsString(element_type);
    ret = std::format("(0..{}).map(|_| {}).collect::<Box<[{}]>>()",
                      size_as_string.c_str(), default_as_string,
                      element_type_as_string);
  } else if (Mapper::ToString(qual_type) == "struct std::pair") {
    auto template_args = *GetTemplateArgs(qual_type);
    auto first_type = template_args[0].getAsType();
    auto second_type = template_args[1].getAsType();
    ret = std::format("(Rc::new(RefCell::new({})), Rc::new(RefCell::new({})))",
                      GetDefaultAsString(first_type),
                      GetDefaultAsString(second_type));
  } else if (Mapper::ToString(qual_type).contains("std::array")) {
    ret = Converter::GetDefaultAsString(qual_type);
  } else {
    return Converter::GetDefaultAsString(qual_type);
  }
  return BoxValue(std::move(ret));
}

std::string
ConverterRefCount::GetDefaultAsStringFallback(clang::QualType qual_type) {
  return std::format("<{}>::default()", ToString(qual_type));
}

std::string
ConverterRefCount::ConvertVarDefaultInit(clang::QualType qual_type) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return GetDefaultAsString(qual_type);
}

std::vector<const char *>
ConverterRefCount::GetStructAttributes(const clang::CXXRecordDecl *decl,
                                       bool &out_impl_default) {
  std::vector<const char *> attrs = {};

  if (out_impl_default) {
    attrs.emplace_back("Default");
  }
  return attrs;
}

void ConverterRefCount::ConvertVarInit(clang::QualType qual_type,
                                       clang::Expr *expr) {
  if (auto lambda = clang::dyn_cast<clang::LambdaExpr>(
          expr->IgnoreUnlessSpelledInSource())) {
    std::string str;
    {
      Buffer buf(*this);
      PushConversionKind push(*this, ConversionKind::Unboxed);
      if (qual_type->isFunctionPointerType() && lambda->capture_size() == 0) {
        StrCat("FnPtr::new((");
        VisitLambdaExpr(lambda);
        StrCat("), 0)");
      } else {
        VisitLambdaExpr(lambda);
      }
      str = std::move(buf).str();
    }
    StrCat(BoxValue(std::move(str)));
    return;
  }

  bool is_ref = qual_type->isReferenceType();
  in_function_formals_ = true;
  PushConversionKind push(*this, ConversionKind::Unboxed, is_ref);
  StrCat(BoxValue((is_ref || qual_type->isFunctionPointerType())
                      ? ConvertFreshPointer(expr)
                      : ConvertFreshRValue(expr)));
  in_function_formals_ = false;
}

static std::unordered_set<const clang::ValueDecl *>
GetAllVars(const clang::Stmt *stmt) {
  std::unordered_set<const clang::ValueDecl *> vars;

  if (auto *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(stmt)) {
    vars.insert(decl_ref->getDecl());
  } else if (auto *member = clang::dyn_cast<clang::MemberExpr>(stmt)) {
    vars.insert(member->getMemberDecl());
    auto child_vars = GetAllVars(member->getBase());
    vars.insert(child_vars.begin(), child_vars.end());
  }

  for (auto *child : stmt->children()) {
    auto child_vars = GetAllVars(child);
    vars.insert(child_vars.begin(), child_vars.end());
  }
  return vars;
}

bool ConverterRefCount::MayCauseBorrowMutError(const clang::Expr *lhs,
                                               const clang::Expr *rhs) {
  auto lhs_vars = GetAllVars(lhs);
  auto rhs_vars = GetAllVars(rhs);

  auto predicate = [lhs](auto *var) {
    auto qual_type = var->getType();
    return (qual_type->isPointerType() || qual_type->isReferenceType()) &&
           qual_type->getPointeeType()
                   .getCanonicalType()
                   .getUnqualifiedType() ==
               lhs->getType().getCanonicalType().getUnqualifiedType();
  };

  if (std::ranges::any_of(rhs_vars, predicate) ||
      (std::ranges::any_of(lhs_vars, predicate) && !rhs_vars.empty())) {
    return true;
  }

  for (auto *lhs_var : lhs_vars) {
    if (rhs_vars.count(lhs_var))
      return true;
  }
  return false;
}

void ConverterRefCount::EmitSetOrAssign(clang::Expr *lhs,
                                        std::string_view rhs) {
  auto lhs_str = ConvertLValue(lhs);
  if (!pending_deref_.empty()) {
    auto ptr = pending_deref_.take();
    StrCat(ptr, ".write(", rhs, ")");
  } else {
    StrCat(lhs_str, token::kAssign, rhs);
  }
}

void ConverterRefCount::ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                          std::string_view assign_operator) {
  auto rhs_as_string = ConvertFreshRValue(rhs);

  if (isRValue()) {
    StrCat(token::kOpenCurlyBracket);
  }

  if (MayCauseBorrowMutError(lhs, rhs)) {
    StrCat(keyword::kLet, "__rhs", token::kAssign, rhs_as_string,
           token::kSemiColon);
    rhs_as_string = "__rhs";
  }

  if (assign_operator == "=") {
    EmitSetOrAssign(lhs, rhs_as_string);
  } else {
    auto lhs_str = ConvertLValue(lhs);
    if (!pending_deref_.empty()) {
      auto ptr = pending_deref_.take();
      // *ptr += val => { let __ptr = ptr.clone(); let __tmp = __ptr.read() +
      // val;
      // __ptr.write(__tmp) }
      auto op = assign_operator;
      op.remove_suffix(1); // remove '='
      StrCat("{ let __ptr = ", ptr, ".clone(); let __tmp = __ptr.read() ", op,
             " ", rhs_as_string, "; __ptr.write(__tmp) }");
    } else {
      StrCat(lhs_str, assign_operator, rhs_as_string);
    }
  }

  if (isRValue()) {
    StrCat(token::kSemiColon, ConvertRValue(lhs), token::kCloseCurlyBracket);
  }
}

void ConverterRefCount::ConvertGenericBinaryOperator(
    clang::BinaryOperator *expr) {
  auto lhs = expr->getLHS();
  auto rhs = expr->getRHS();
  auto opcode = expr->getOpcodeStr().str();

  auto lhs_vars = GetAllVars(lhs);
  auto rhs_vars = GetAllVars(rhs);

  auto predicate = [](auto *var) {
    return var->getType()->isPointerType() || var->getType()->isReferenceType();
  };

  auto sides_contains_literal = rhs_vars.empty() || lhs_vars.empty();
  auto same_var_on_both_sides = lhs_vars == rhs_vars;
  auto sides_contain_ptr_or_deref = std::ranges::any_of(rhs_vars, predicate) ||
                                    std::ranges::any_of(lhs_vars, predicate);

  auto both_sides_have_va_arg = same_var_on_both_sides &&
                                ContainsVAArgExpr(lhs) &&
                                ContainsVAArgExpr(rhs);

  auto may_cause_borrow_mut_err =
      both_sides_have_va_arg ||
      (!sides_contains_literal && !same_var_on_both_sides &&
       sides_contain_ptr_or_deref);

  if (may_cause_borrow_mut_err) {
    StrCat(std::format("{{ let _lhs = {}; _lhs {} {} }}",
                       ConvertFreshRValue(lhs), opcode,
                       ConvertFreshRValue(rhs)));
    return;
  }

  StrCat(token::kOpenParen);
  Convert(lhs);
  StrCat(opcode);
  Convert(rhs);
  StrCat(token::kCloseParen);
}

void ConverterRefCount::ConvertUniquePtrDeref(
    clang::CXXOperatorCallExpr *expr) {
  if (isAddrOf()) {
    StrCat(ConvertRValue(expr->getArg(0)), ".as_pointer()");
    computed_expr_type_ = ComputedExprType::FreshPointer;
  } else {
    StrCat(std::format("(*{}.as_ref().unwrap().borrow{}())",
                       ToString(expr->getArg(0)), isRValue() ? "" : "_mut"));
    SetValueFreshness(expr->getType());
  }
}

bool ConverterRefCount::ConvertCXXOperatorCallExpr(
    clang::CXXOperatorCallExpr *expr) {
  switch (expr->getOperator()) {
  case clang::OverloadedOperatorKind::OO_Equal:
    ConvertAssignment(expr->getArg(0), expr->getArg(1), token::kAssign);
    break;

  case clang::OverloadedOperatorKind::OO_Arrow:
  case clang::OverloadedOperatorKind::OO_Star:
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      ConvertUniquePtrDeref(expr);
      break;
    }

    if (isLValue()) {
      pending_deref_.set(ToString(expr->getArg(0)));
      break;
    }

    if (GetStrongestIteratorCategory(expr->getArg(0)->getType()) ==
        IteratorCategory::Bidirectional) {
      Convert(expr->getArg(0));
      break;
    }

    if (!isAddrOf()) {
      StrCat(token::kOpenParen, GetPointerDerefPrefix(expr->getType()));
    }
    Convert(expr->getArg(0));
    if (!isAddrOf()) {
      StrCat(GetPointerDerefSuffix(expr->getType()), token::kCloseParen);
      SetValueFreshness(expr->getType());
    }
    break;

  case clang::OverloadedOperatorKind::OO_Subscript: {
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      StrCat(
          std::format("{}.as_ref().unwrap()", ConvertRValue(expr->getArg(0))));
      if (isAddrOf()) {
        StrCat(std::format(".as_pointer().offset(({}) as isize)",
                           ConvertRValue(expr->getArg(1))));
      } else {
        if (isRValue()) {
          StrCat(".borrow()");
        } else {
          StrCat(".borrow_mut()");
        }
        StrCat(std::format("[({}) as usize]", ConvertRValue(expr->getArg(1))));
      }
      SetValueFreshness(expr->getType());
      break;
    }

    bool is_inner_boxed =
        IsBoxedType(expr->getType().getNonReferenceType()) &&
        IsBoxedType(expr->getArg(0)->getType().getNonReferenceType());

    if (isLValue()) {
      PushConversionKind push_ck(*this, ConversionKind::Unboxed);
      pending_deref_.set(std::format("({} as {}).offset({} as isize)",
                                     ConvertObject(expr->getArg(0)),
                                     ConvertPtrType(expr->getArg(0)->getType()),
                                     ConvertRValue(expr->getArg(1))),
                         expr);
      break;
    }

    if (!isAddrOf()) {
      StrCat(token::kOpenParen, GetPointerDerefPrefix(expr->getType()));
    }

    if (is_inner_boxed && !isObject()) {
      StrCat("(");
    }

    PushConversionKind push(*this, ConversionKind::Unboxed);
    StrCat(std::format("({} as {}).offset({} as isize)",
                       ConvertObject(expr->getArg(0)),
                       ConvertPtrType(expr->getArg(0)->getType()),
                       ConvertRValue(expr->getArg(1))));

    if (is_inner_boxed) {
      StrCat(GetPointerDerefSuffix(expr->getType()), ".as_pointer()");
      if (!isObject()) {
        StrCat(std::format("as Ptr<{}>)", ToString(expr->getType())));
      }
    }

    if (isAddrOf()) {
      computed_expr_type_ = ComputedExprType::FreshPointer;
    } else {
      StrCat(GetPointerDerefSuffix(expr->getType()), token::kCloseParen);
      SetValueFreshness(expr->getType());
    }
    break;
  }
  default:
    return Converter::ConvertCXXOperatorCallExpr(expr);
  }
  return false;
}

void ConverterRefCount::ConvertFunctionParameters(clang::FunctionDecl *decl) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  if (decl->isMain() && (decl->getNumParams() != 0U)) {
    StrCat(std::format("{}: i32, {}: Ptr<Ptr<u8>>",
                       GetNamedDeclAsString(decl->getParamDecl(0)),
                       GetNamedDeclAsString(decl->getParamDecl(1))));
  } else {
    Converter::ConvertFunctionParameters(decl);
  }
}

void ConverterRefCount::ConvertArraySubscript(clang::Expr *base,
                                              clang::Expr *idx,
                                              clang::QualType type) {
  if (isAddrOf()) {
    bool is_inner_boxed = false;
    if (auto base_arr_ty = clang::dyn_cast<clang::ArrayType>(
            base->IgnoreImplicit()->getType().getTypePtr())) {
      is_inner_boxed = clang::isa<clang::ArrayType>(
          base_arr_ty->getElementType().getTypePtr());
    }

    if (is_inner_boxed) {
      StrCat("(");
    }

    StrCat(std::format(
        "({} as {}).offset({} as isize)", ToString(base->IgnoreImplicit()),
        ConvertPtrType(base->IgnoreImplicit()->getType()), ConvertRValue(idx)));

    if (is_inner_boxed) {
      StrCat(GetPointerDerefSuffix(type), ".as_pointer()", token::kCloseParen);
    }

    computed_expr_type_ = ComputedExprType::FreshPointer;
  } else {
    if (isLValue() &&
        clang::isa<clang::ArraySubscriptExpr>(base->IgnoreImplicit())) {
      PushExprKind push(*this, ExprKind::RValue);
      Convert(base->IgnoreImplicit());
    } else {
      Convert(base->IgnoreImplicit());
    }
    if (clang::isa<clang::ArraySubscriptExpr>(base->IgnoreImplicit())) {
      if (isRValue()) {
        StrCat(".borrow()");
      } else {
        StrCat(".borrow_mut()");
      }
    }
    StrCat(std::format("[({}) as usize]", ConvertRValue(idx)));
    SetValueFreshness(type);
  }
}

void ConverterRefCount::ConvertPointerSubscript(
    clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  auto *idx = expr->getIdx();

  if (isLValue()) {
    pending_deref_.assert_consumed();
    Buffer buf(*this);
    ConvertPointerOffset(base, idx);
    pending_deref_.set_unchecked(std::move(buf).str(), expr);
    return;
  }

  if (!isAddrOf())
    StrCat(token::kOpenParen, GetPointerDerefPrefix(expr->getType()));
  ConvertPointerOffset(base, idx);
  if (!isAddrOf()) {
    StrCat(GetPointerDerefSuffix(expr->getType()), token::kCloseParen);
    SetValueFreshness(expr->getType());
  }
}

void ConverterRefCount::ConvertFunctionMain(
    const clang::FunctionDecl *decl,
    const std::string_view main_function_name) {
  if (decl->getNumParams() != 0U) {
    StrCat(std::format(R"(
pub fn main() {{
    let argv: Vec<Value<Vec<u8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.as_bytes().to_vec())))
        .collect();
    let mut argv: Value<Vec<Ptr<u8>>> = Rc::new(RefCell::new(
        argv.iter().map(|x| {{ x.borrow_mut().push(0); x.as_pointer() }}).collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    ::std::process::exit({}(::std::env::args().len() as i32,
                                argv.as_pointer()));
}})",
                       main_function_name));
  } else {
    StrCat(std::format("pub fn main() {{ std::process::exit({}()); }}",
                       main_function_name));
  }
}

void ConverterRefCount::ConvertAddrOf(clang::Expr *expr,
                                      clang::QualType pointer_type) {
  StrCat(ConvertPointer(expr));
}

void ConverterRefCount::ConvertDeref(clang::Expr *expr) {
  auto pointee_type = expr->getType()->getPointeeType();

  if (isLValue()) {
    pending_deref_.set(ToString(expr));
    return;
  }

  if (!isAddrOf()) {
    StrCat(token::kOpenParen, GetPointerDerefPrefix(pointee_type));
  }
  Convert(expr);
  if (!isAddrOf()) {
    StrCat(GetPointerDerefSuffix(pointee_type), token::kCloseParen);
    SetValueFreshness(expr->getType());
  }

  if (isObject()) {
    if (IsBoxedType(pointee_type)) {
      StrCat(".to_strong().as_pointer()");
      computed_expr_type_ = ComputedExprType::FreshPointer;
    }
  }
}

void ConverterRefCount::ConvertArrow(clang::Expr *expr) {
  auto *op = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
  bool is_overloaded_arrow =
      op && op->getOperator() == clang::OverloadedOperatorKind::OO_Arrow;

  if (!is_overloaded_arrow) {
    auto ptr = ToString(expr);
    StrCat(DerefPtrExpr(ptr, expr->getType()->getPointeeType()));
    return;
  }

  if (GetStrongestIteratorCategory(op->getArg(0)->getType()) ==
      IteratorCategory::Bidirectional) {
    Convert(op->getArg(0));
    return;
  }

  Convert(expr);
}

std::string ConverterRefCount::AccessLValueObject(clang::MemberExpr *member) {
  auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member->getMemberDecl());
  auto *object = member->getBase();

  bool is_mut = method && !method->isConst();
  if (member->isArrow()) {
    auto *op =
        clang::dyn_cast<clang::CXXOperatorCallExpr>(object->IgnoreImplicit());
    if (op && GetStrongestIteratorCategory(op->getArg(0)->getType()) ==
                  IteratorCategory::Bidirectional) {
      return ConvertRValue(op->getArg(0));
    }
    auto str = is_mut ? ConvertLValue(object) : ConvertRValue(object);
    auto pointee_type = object->getType()->getPointeeType();
    return DerefPtrExpr(str, pointee_type);
  }
  return is_mut ? ConvertLValue(object) : ConvertRValue(object);
}

void ConverterRefCount::emplace_back_plugin_construct_arg(
    clang::QualType elem_type, clang::CXXConstructExpr *ctor) {
  PushUnboxedIfSimple push(*this, "Vec<%>", elem_type);
  ConvertVarInit(elem_type, ctor);
}

void ConverterRefCount::emplace_back_emit_push_open(
    clang::CXXMemberCallExpr *call) {
  auto *obj = GetCallObject(call);
  auto obj_type = obj->getType().getNonReferenceType();
  if (obj_type->isPointerType()) {
    obj_type = obj_type->getPointeeType();
  }
  StrCat(ConvertObject(obj), ".with_mut(|__v: &mut ",
         ToString(obj_type.getNonReferenceType()), "| __v.push(");
}

void ConverterRefCount::emplace_back_emit_push_close(
    clang::CXXMemberCallExpr *call) {
  StrCat("))");
}

const char *
ConverterRefCount::GetPointerDerefSuffix(clang::QualType pointee_type) {
  if (pointee_type.isPODType(ctx_) && !pointee_type->isRecordType()) {
    return ".read()";
  }
  return ".upgrade().deref()";
}

const char *
ConverterRefCount::GetPointerDerefPrefix(clang::QualType pointee_type) {
  if (pointee_type.isPODType(ctx_) && !pointee_type->isRecordType()) {
    return "";
  }
  return token::kStar;
}

std::string ConverterRefCount::DerefPtrExpr(std::string_view ptr_expr,
                                            clang::QualType pointee_type) {
  return std::format("({}{}{})", GetPointerDerefPrefix(pointee_type), ptr_expr,
                     GetPointerDerefSuffix(pointee_type));
}

bool ConverterRefCount::IsReferenceType(const clang::Expr *expr) const {
  if (Converter::IsReferenceType(expr)) {
    return true;
  }
  if (auto *call =
          clang::dyn_cast<clang::CXXOperatorCallExpr>(expr->IgnoreCasts())) {
    return GetReturnTypeOfFunction(call)->isReferenceType();
  }
  return false;
}

std::string ConverterRefCount::ConvertMappedMethodCall(
    clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
    clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx) {
  auto receiver_ph = mc.getReceiverPlaceholder();
  if (!receiver_ph || receiver_ph->access == TranslationRule::Access::kRead) {
    return Converter::ConvertMappedMethodCall(expr, mc, args, num_args, ctx);
  }

  auto arg_idx = std::stoi(receiver_ph->arg.substr(1));
  auto *arg = BuildUnifiedArgs(expr, args, num_args)[arg_idx];

  if (!arg->getType()->isPointerType() && !IsReferenceType(arg)) {
    return Converter::ConvertMappedMethodCall(expr, mc, args, num_args, ctx);
  }

  auto param_type = Mapper::GetParamType(GetCalleeOrExpr(expr), arg_idx);

  if (arg->getType()->isPointerType()) {
    return std::format("{}.with_mut(|__v: {}| __v{})", ConvertPointer(arg),
                       param_type,
                       ConvertIRFragment(mc.body, expr, args, num_args, ctx));
  }

  ConvertIRFragment(mc.receiver, expr, args, num_args, ctx);
  assert(!pending_deref_.empty());

  bool is_boxed = pending_deref_.is_boxed();
  auto ptr = pending_deref_.take();
  auto body = ConvertIRFragment(mc.body, expr, args, num_args, ctx);

  if (is_boxed) {
    return std::format(
        "{}.with_mut(|__v: &mut Value<{}>| (*__v.borrow_mut()){})", ptr,
        ToString(arg->getType()), body);
  }

  return std::format("{}.with_mut(|__v: {}| __v{})", ptr, param_type, body);
}

} // namespace cpp2rust
