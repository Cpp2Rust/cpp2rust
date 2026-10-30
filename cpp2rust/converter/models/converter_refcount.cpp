// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/models/converter_refcount.h"

#include <clang/AST/RecordLayout.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Lex/Lexer.h>
#include <llvm/Support/ErrorHandling.h>

#include <algorithm>
#include <format>
#include <ranges>

#include "compiler.h"
#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/mapper.h"
#include "converter/rules/registry.h"

namespace cpp2rust {
std::map<std::string, ConverterRefCount::MethodsOnPtr>
    ConverterRefCount::methods_on_ptr_;

void ConverterRefCount::EmitMethodsOnPtr(std::string &out) {
  for (const auto &[name, methods] : methods_on_ptr_) {
    EmitDeferredBlock(methods.trait, out);
    EmitDeferredBlock(methods.impl, out);
  }
}

ConverterRefCount::ConverterRefCount(std::string &rs_code,
                                     clang::ASTContext &ctx)
    : ConverterRefCount(rs_code, ctx, std::make_shared<BoxedVars>(ctx)) {}

ConverterRefCount::ConverterRefCount(
    std::string &rs_code, clang::ASTContext &ctx,
    std::shared_ptr<const BoxedVars> boxed_vars)
    : Converter(rs_code, ctx, "", ""), boxed_vars_(std::move(boxed_vars)),
      conversion_kind_({ConversionKind::Unboxed}) {}

void ConverterRefCount::EmitFilePreamble() {
  StrCat(R"(
extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{Read, Write, Seek};
use std::io::prelude::*;
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
)");
}

static bool IsBoxedType(std::string_view type) {
  return type.starts_with("Vec<") || type.starts_with("Box<");
}

static bool IsBoxedType(clang::ASTContext &ctx, clang::QualType type) {
  return IsBoxedType(Mapper::Map(ctx, type.getUnqualifiedType()));
}

// A Rust type without whitespace, and with the paths into libcc2rs, which the
// rules import, unqualified, to compare it with another.
static std::string NormalizeType(std::string type) {
  std::erase_if(type, [](char c) { return std::isspace(c); });
  static constexpr std::string_view kLib = "libcc2rs::";
  for (size_t pos; (pos = type.find(kLib)) != std::string::npos;) {
    type.erase(pos, kLib.size());
  }
  return type;
}

// Virtual methods take &mut self, as the fields of the struct are stored in
// it.
static bool NeedsMutAccess(clang::ASTContext &ctx,
                           const clang::CXXMethodDecl *method,
                           clang::QualType base_type) {
  return !method->isConst() &&
         (IsBoxedType(ctx, base_type) || method->isVirtual());
}

// Whether `expr` is an array field whose elements are accessed through
// array_field_ptr!, such that they are found in the memory of the struct when
// the struct is reinterpreted from other memory.
static bool IsArrayFieldPtr(clang::ASTContext &ctx, const clang::Expr *expr) {
  auto *member =
      clang::dyn_cast<clang::MemberExpr>(expr->IgnoreParenImpCasts());
  auto *field = member
                    ? clang::dyn_cast<clang::FieldDecl>(member->getMemberDecl())
                    : nullptr;
  if (!field || field->getParent()->isUnion() ||
      Mapper::Contains(ctx, member)) {
    return false;
  }
  auto *type = field->getType()->getAsArrayTypeUnsafe();
  return type && !type->getElementType()->isArrayType();
}

// Whether `expr` is a field of an element of a std::vector reached through
// a pointer, whose arrays and vectors are accessed through the place of the
// element, which is never reinterpreted memory.
bool ConverterRefCount::IsFieldOfVectorElem(
    const clang::MemberExpr *expr) const {
  auto *op = clang::dyn_cast<clang::CXXOperatorCallExpr>(
      expr->getBase()->IgnoreParenImpCasts());
  return !expr->isArrow() && op && op->getOperator() == clang::OO_Subscript &&
         !Mapper::Contains(ctx_, op) && !IsElemOfBorrowedValue(op);
}

// Whether the element that a subscript accesses is reached by borrowing the
// container, which is stored in a Value, unless computing the index may
// access the container too, as it stays borrowed meanwhile.
bool ConverterRefCount::IsElemOfBorrowedValue(
    const clang::CXXOperatorCallExpr *expr) const {
  auto *idx = expr->getArg(1);
  return IsValueLocal(expr->getArg(0)) && !idx->HasSideEffects(ctx_) &&
         !RefersToPointer(idx) && !ReferToCommonVar(idx, expr->getArg(0));
}

static bool IsPointerType(clang::ASTContext &ctx, clang::QualType type) {
  return type->isPointerType() ||
         RuleRegistry::MapsToRefcountPointer(ctx, type);
}

void ConverterRefCount::PendingDeref::set(std::string str, bool fresh) {
  assert_consumed();
  set_unchecked(std::move(str), fresh);
}

void ConverterRefCount::PendingDeref::set_unchecked(std::string str,
                                                    bool fresh) {
  value = std::move(str);
  ptr_is_fresh = fresh;
  type = ComputedExprType::Pending;
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

std::string
ConverterRefCount::GetSafeTypeAsString(clang::QualType qual_type) const {
  std::string type_as_string;
  ConverterRefCount converter(type_as_string, ctx_, boxed_vars_);
  converter.Convert(qual_type);
  return std::string(Trim(type_as_string));
}

bool ConverterRefCount::NeedsMut(const clang::VarDecl *decl,
                                 clang::QualType type,
                                 llvm::StringRef /*name*/) const {
  if (hoisted_decls_.contains(decl) && type->isReferenceType()) {
    return true;
  }
  if (!IsUnboxedVar(decl)) {
    return false;
  }
  if (!in_function_formals_) {
    return true;
  }
  // Parameters are modified in place, except for those with a default value,
  // which are unwrapped into a variable, and those of a function declared
  // without a body, which can't be mutable.
  auto *param = clang::cast<clang::ParmVarDecl>(decl);
  auto *function = clang::cast<clang::FunctionDecl>(param->getDeclContext());
  return !HasUsableDefaultArg(param) && !function->isPureVirtual() &&
         method_target_ != MethodTarget::TraitDecl;
}

bool ConverterRefCount::IsUnboxedVar(const clang::ValueDecl *decl) const {
  auto *var = clang::dyn_cast<clang::VarDecl>(decl);
  return var && !boxed_vars_->contains(var);
}

bool ConverterRefCount::IsUnboxedPlace(const clang::Expr *expr) const {
  expr = expr->IgnoreParenImpCasts();
  if (auto *ref = clang::dyn_cast<clang::DeclRefExpr>(expr)) {
    return IsUnboxedVar(ref->getDecl());
  }
  // References are accessed through pointers.
  if (auto *member = clang::dyn_cast<clang::MemberExpr>(expr);
      member && member->getMemberDecl()->getType()->isReferenceType()) {
    return false;
  }
  auto *object = GetEnclosingObject(expr);
  return object && IsUnboxedPlace(object);
}

bool ConverterRefCount::IsValueLocal(const clang::Expr *expr) const {
  auto *ref = clang::dyn_cast<clang::DeclRefExpr>(expr->IgnoreParenImpCasts());
  auto *var = ref ? clang::dyn_cast<clang::VarDecl>(ref->getDecl()) : nullptr;
  // Variables captured by reference are accessed through pointers.
  return var && !IsGlobalVar(var) && !IsUnboxedVar(var) &&
         !GetDeclRefType(curr_function_, ref, var)->isReferenceType();
}

bool ConverterRefCount::IsFieldOfUnboxedPlace(
    const clang::MemberExpr *expr) const {
  auto *object = GetEnclosingObject(expr);
  return object && IsUnboxedPlace(object);
}

ConverterRefCount::ConversionKind
ConverterRefCount::VarConversionKind(const clang::VarDecl *decl) const {
  if (in_function_formals_) {
    return ConversionKind::Unboxed;
  }
  return IsUnboxedVar(decl) ? ConversionKind::Unboxed
                            : ConversionKind::FullRefCount;
}

std::string ConverterRefCount::BoxType(std::string &&str) const {
  if (getConversionKind() != ConversionKind::FullRefCount) {
    return std::move(str);
  }
  return std::format("Value<{}>", std::move(str));
}

std::string ConverterRefCount::BoxValue(std::string &&str) const {
  if (getConversionKind() != ConversionKind::FullRefCount) {
    return std::move(str);
  }
  return std::format("Rc::new(RefCell::new({}))", std::move(str));
}

std::string
ConverterRefCount::GetUnboxedTypeAsString(clang::QualType qual_type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return ToString(qual_type);
}

std::string
ConverterRefCount::GetUnboxedDefaultAsString(clang::QualType qual_type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return GetDefaultAsString(qual_type);
}

bool ConverterRefCount::Convert(clang::QualType qual_type) {
  // Catch va_list before desugaring
  if (IsVaListType(qual_type)) {
    StrCat(BoxType("Ptr<VaArg>"));
    return false;
  }

  if (!Mapper::Contains(ctx_, qual_type))
    qual_type = qual_type.getUnqualifiedType().getDesugaredType(ctx_);

  if (qual_type->isReferenceType() || qual_type->isIncompleteArrayType()) {
    return Converter::Convert(qual_type);
  }

  StrCat(BoxType(Converter::ToStringBase(qual_type)));
  return false;
}

bool ConverterRefCount::VisitIncompleteArrayType(
    clang::IncompleteArrayType *type) {
  std::string str;
  {
    PushConversionKind push(*this, type->getElementType()->isArrayType()
                                       ? ConversionKind::Pointee
                                       : ConversionKind::Unboxed);
    str = std::format("Box<[{}]>", ToString(type->getElementType()));
  }
  StrCat(BoxType(std::move(str)));
  return false;
}

bool ConverterRefCount::VisitReferenceType(clang::ReferenceType *type) {
  auto pointee_type = type->getPointeeType();
  if (pointee_type->isFunctionType()) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    return Convert(ctx_.getPointerType(pointee_type));
  }
  if (pointee_type->isArrayType() && !IsVaListType(pointee_type)) {
    // A reference to an array decays straight to a pointer to its first
    // element, the same way a by-value array parameter would, instead of
    // going through a pointer to the whole boxed array.
    auto element_type = pointee_type->getAsArrayTypeUnsafe()->getElementType();
    PushConversionKind push(*this, element_type->isArrayType()
                                       ? ConversionKind::Pointee
                                       : ConversionKind::Ptr);
    StrCat("Ptr<");
    Convert(element_type);
    StrCat(token::kGt);
    return false;
  }
  PushConversionKind push(*this, ConversionKind::Pointee);
  StrCat("Ptr<");
  Convert(pointee_type);
  StrCat(token::kGt);
  return false;
}

std::string ConverterRefCount::ConvertFunctionPointerType(
    const clang::FunctionProtoType *proto) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::ConvertFunctionPointerType(proto);
}

bool ConverterRefCount::VisitPointerType(clang::PointerType *type) {
  if (auto proto = type->getPointeeType()->getAs<clang::FunctionProtoType>()) {
    StrCat(std::format("FnPtr<{}>", ConvertFunctionPointerType(proto)));
    return false;
  }

  if (type->isVoidPointerType()) {
    StrCat("AnyPtr");
    return false;
  }

  auto pointee_type = type->getPointeeType();
  bool pointee_is_array =
      pointee_type->isArrayType() && !IsVaListType(pointee_type);
  PushConversionKind push(*this, pointee_is_array ? ConversionKind::Pointee
                                                  : ConversionKind::Ptr);
  if (pointee_type->isRecordType() &&
      IsAbstractUserClass(pointee_type->getAsRecordDecl())) {
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
    StrCat('[');
    Convert(type->getElementType());
    StrCat(std::format("; {}]", GetNumAsString(type->getSize()).c_str()));
    break;
  case ConversionKind::Ptr: {
    PushConversionKind elem(*this, ConversionKind::Pointee,
                            type->getElementType()->isArrayType());
    Convert(type->getElementType());
    break;
  }
  case ConversionKind::Pointee:
  case ConversionKind::FullRefCount: {
    PushConversionKind elem(*this, ConversionKind::Pointee,
                            type->getElementType()->isArrayType());
    StrCat("Box<[");
    Convert(type->getElementType());
    StrCat("]>");
    break;
  }
  }
  return false;
}

std::string ConverterRefCount::ConvertObject(clang::Expr *expr,
                                             ObjectShape shape) {
  PushExprKind push(*this, ExprKind::Object);
  auto saved_shape = std::exchange(object_shape_, shape);
  auto str = ToString(expr);
  // Unless the visitor already made a pointer to the elements.
  bool decay = object_shape_ == ObjectShape::Element;
  object_shape_ = saved_shape;
  if (decay) {
    auto type = expr->getType().getNonReferenceType();
    bool is_ptr = type->isPointerType();
    if (is_ptr) {
      type = type->getPointeeType();
    }
    if (IsBoxedType(ctx_, type) || (is_ptr && type->isArrayType())) {
      MarkFresh();
      return std::move(str) + ".decay()";
    }
  }
  return str;
}

std::string ConverterRefCount::AsPointerMethod(clang::QualType type) {
  if (isObject() && object_shape_ == ObjectShape::Element &&
      type->isRecordType() && IsBoxedType(ctx_, type)) {
    object_shape_ = ObjectShape::Whole;
    return ".decay()";
  }
  return ".as_pointer()";
}

std::string ConverterRefCount::ConvertPointerReceiver(clang::Expr *arg,
                                                      std::string_view) {
  return isObject() ? ConvertObject(arg) : ConvertFreshObject(arg);
}

bool ConverterRefCount::ParamIsElementPtr(clang::Expr *expr, clang::Expr *arg,
                                          unsigned index) {
  if (!RuleRegistry::ParamIsPointer(ctx_, expr, index)) {
    return false;
  }
  auto type = arg->getType().getNonReferenceType();
  if (type->isPointerType()) {
    type = type->getPointeeType();
  }
  type = type.getUnqualifiedType();
  if (!IsBoxedType(ctx_, type)) {
    return false;
  }
  // The C++ parameter is a reference or pointer to the container, which the
  // rule takes either as a pointer to it or as one to its elements.
  auto param = NormalizeType(Mapper::GetParamType(*this, expr, index));
  auto whole = NormalizeType(ToString(ctx_.getPointerType(type)));
  if (param == whole) {
    return false;
  }
  auto element = NormalizeType(ConvertPtrType(type));
  if (param != element) {
    llvm::errs() << "rule parameter " << index << " of type " << param
                 << " is neither " << whole << " nor " << element << '\n';
    llvm::report_fatal_error("rule parameter does not match the C++ one");
  }
  return true;
}

std::string ConverterRefCount::ConvertElementPtr(clang::Expr *arg) {
  return ConvertObject(arg, ObjectShape::Element);
}

void ConverterRefCount::CheckRuleReturnType(clang::Expr *expr) {
  auto *call = clang::dyn_cast<clang::CallExpr>(expr);
  if (!call ||
      !RuleRegistry::GetExprRule(ctx_, expr)->return_type.is_refcount_pointer) {
    return;
  }
  auto type = RuleRegistry::GetDeclaredReturnType(expr);
  if (auto ref = type->getAs<clang::ReferenceType>()) {
    auto pointee = ref->getPointeeType();
    type = pointee->isRecordType() &&
                   RuleRegistry::MapsToRefcountPointer(ctx_, pointee)
               ? pointee
               : ctx_.getPointerType(pointee);
  }
  auto ret = NormalizeType(Mapper::GetReturnType(*this, expr));
  auto expected = NormalizeType(ToString(type));
  if (ret != expected) {
    llvm::errs() << "rule return type " << ret << " is not " << expected
                 << '\n';
    llvm::report_fatal_error("rule return type does not match the C++ one");
  }
}

std::string ConverterRefCount::ConvertFreshObject(clang::Expr *expr) {
  return MakeFresh(ConvertObject(expr));
}

std::string ConverterRefCount::ConvertTemporaryPointer(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  return std::format("Rc::new(RefCell::new({})).as_pointer()",
                     ConvertFreshRValue(expr, implicit_convert_to));
}

std::string ConverterRefCount::ConvertFresh(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  auto str = ToString(expr, implicit_convert_to);
  if (expr->getType()->isVoidType() || isVoid()) {
    return str;
  }
  return MakeFresh(std::move(str));
}

std::string ConverterRefCount::ConvertFreshRValue(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  std::string str;
  {
    ScopedValue<const clang::Expr *> copied(copied_expr_,
                                            expr->IgnoreParenImpCasts());
    str = ConvertRValue(expr, implicit_convert_to);
  }
  if (expr->getType()->isVoidType()) {
    MarkFresh();
    return str;
  }
  return MakeFresh(std::move(str));
}

std::string ConverterRefCount::ConvertFreshPointer(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  return MakeFresh(ConvertPointer(expr, implicit_convert_to));
}

std::string ConverterRefCount::ConvertPointeeCast(std::string str,
                                                  const clang::Expr *from,
                                                  clang::QualType to) {
  if (to->isFunctionPointerType()) {
    MarkFresh();
    return std::format(
        "({}).cast::<{}>()", str,
        ConvertFunctionPointerType(
            to->getPointeeType()->getAs<clang::FunctionProtoType>()));
  }
  if (to->isReferenceType() && !isAddrOf()) {
    return str;
  }
  if (auto pointee = GetExprPointee(ctx_, from, to);
      to->isReferenceType() && pointee->isArrayType()) {
    str = std::format("{} as {}", str,
                      ToString(ctx_.getLValueReferenceType(pointee)));
  }
  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto element = ctx_.getBaseElementType(to->getPointeeType());
  MarkFresh();
  return std::format("({}).reinterpret_cast::<{}>()", str,
                     ToString(element.getUnqualifiedType()));
}

std::pair<std::string, std::string>
ConverterRefCount::MaterializeTemp(const std::string &binding_name,
                                   clang::QualType param_type,
                                   clang::Expr *expr) {
  auto pointee = param_type.getNonReferenceType();
  const auto *decl = in_const_initializer_ ? keyword::kStatic : keyword::kLet;
  std::string binding;
  if (pointee->isConstantArrayType()) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    binding =
        std::format("{} {} : {} = {};", decl, binding_name, ToString(pointee),
                    BoxValue(ConvertVarInitValue(pointee, expr)));
  } else {
    auto value = ConvertFreshRValue(expr, pointee);
    auto type_str = ToStringBase(pointee);
    binding = std::format("{} {} : Value<{}> = Rc::new(RefCell::new({}));",
                          decl, binding_name, type_str, value);
  }
  auto ref =
      in_const_initializer_ ? ".with(Value::as_pointer)" : ".as_pointer()";
  return {binding, binding_name + ref};
}

std::string ConverterRefCount::ConvertPtrType(clang::QualType type) {
  std::string str;
  // decays into Ptr; remove the outer type Vec<>
  if (IsBoxedType(ctx_, type)) {
    str = GetInnerType(type);
  } else {
    PushConversionKind push(*this, ConversionKind::Ptr);
    str = ToString(type);
  }
  return std::format("Ptr<{}>", std::move(str));
}

bool ConverterRefCount::IsPointerSubscript(
    const clang::ArraySubscriptExpr *expr) const {
  auto *base = expr->getBase();
  if (base->IgnoreCasts()->getType()->isPointerType() ||
      IsUnionArrayMember(base) ||
      (IsArrayFieldPtr(ctx_, base) && !IsUnboxedPlace(base)) ||
      (IsReferenceType(base) &&
       base->IgnoreCasts()->getType()->isArrayType())) {
    return true;
  }
  auto *row = clang::dyn_cast<clang::ArraySubscriptExpr>(base->IgnoreCasts());
  return row && IsPointerSubscript(row);
}

bool ConverterRefCount::VisitArraySubscriptExpr(
    clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  if (IsPointerSubscript(expr)) {
    ConvertPointerSubscript(expr);
  } else {
    if (!base->IgnoreCasts()->getType()->isArrayType()) {
      if (isLValue()) {
        CapturePendingDeref([&] {
          ConvertArraySubscript(base, expr->getIdx(), expr->getType());
        });
        return false;
      }
      PushDeref deref(*this, expr->getType());
      ConvertArraySubscript(base, expr->getIdx(), expr->getType());
    } else {
      ConvertArraySubscript(base, expr->getIdx(), expr->getType());
    }
  }
  return false;
}

bool ConverterRefCount::VisitOffsetOfExpr(clang::OffsetOfExpr *expr) {
  clang::Expr::EvalResult result;
  ENSURE(expr->EvaluateAsInt(result, ctx_));
  EmitFresh(std::format("{}_usize", result.Val.getInt().getZExtValue()));
  return false;
}

// The operands of comparison operators are pointers to the values (&Ptr).
std::string ConverterRefCount::GetComparisonArg(const clang::ParmVarDecl *param,
                                                const clang::CXXRecordDecl *,
                                                std::string_view value) {
  if (param->getType()->isReferenceType()) {
    return std::format("{}.clone()", value);
  }
  return std::format("{}.read()", value);
}

std::string ConverterRefCount::GetComparisonResultReceiver(
    const clang::CXXMethodDecl *, bool returns_ref, std::string_view result) {
  if (returns_ref) {
    return std::format("&{}", result);
  }
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return std::format("&{}.as_pointer()", BoxValue(std::string(result)));
}

std::string
ConverterRefCount::GetComparisonReceiver(const clang::CXXMethodDecl *,
                                         std::string_view lhs) {
  return std::string(lhs);
}

// Records implement the comparisons of the standard library (CppLess and
// CppEq) with their comparison operators, which get pointers to the values.
// They don't implement Ord and PartialEq, which get references to the values,
// as passing those to the operators would require copying the values.
void ConverterRefCount::ConvertOrdAndPartialOrdTraits(
    const clang::CXXRecordDecl *decl, const clang::FunctionDecl *eq,
    const clang::FunctionDecl *lt, const clang::FunctionDecl *cmp) {
  auto record_name = GetRecordName(decl);
  auto emit = [&](std::string_view trait, std::string_view fn,
                  std::string_view body) {
    StrCat(std::format("impl {} for {} {{ fn {}(__lhs: &Ptr<Self>, "
                       "__rhs: &Ptr<Self>) -> bool {{ {} }} }}",
                       trait, record_name, fn, body));
  };
  if (lt) {
    emit("CppLess", "less", GetComparisonBool(lt, decl, "__lhs", "__rhs"));
  } else if (cmp) {
    emit("CppLess", "less",
         std::format("{} == std::cmp::Ordering::Less",
                     GetComparisonCall(cmp, decl, "__lhs", "__rhs")));
  }
  emit("CppEq", "equal", GetEqBody(decl, eq, lt, cmp, "__lhs", "__rhs"));
}

std::string ConverterRefCount::GetShallowCopy(const clang::RecordDecl *decl,
                                              std::string_view src) {
  std::string fields;
  for (auto *field : decl->fields()) {
    auto name = GetNamedDeclAsString(field);
    auto value = std::format("{}.{}", src, name);
    // Fields are copied without running their copy constructors.
    if (auto *record = field->getType()->getAsRecordDecl();
        record && IsUserDefinedDecl(record)) {
      value = GetShallowCopy(record, value);
    } else {
      value += ".clone()";
    }
    fields += std::format("{}: {},", name, value);
  }
  return std::format("{} {{ {} }}", GetRecordName(decl), fields);
}

// Whether decl is copied by copying its bytes or the pointers it holds.
static bool IsClonedShallowly(const clang::RecordDecl *decl) {
  return decl->isUnion() || (HasDefaultedCopyConstructor(decl) &&
                             RecordHasOnlyReferenceFields(decl));
}

bool ConverterRefCount::RecordImplementsClone(const clang::RecordDecl *decl) {
  if (IsClonedShallowly(decl)) {
    return true;
  }
  return !clang::isa<clang::CXXRecordDecl>(decl) ||
         HasCallableCopyConstructor(decl);
}

// Bases are translated to traits, so they hold no state of the struct; they
// only need to be copied without side effects.
static bool HasDefaultedBaseCopies(const clang::CXXRecordDecl *decl) {
  return std::ranges::all_of(decl->bases(), [](auto &base) {
    auto *record = base.getType()->getAsCXXRecordDecl();
    return record && HasDefaultedCopyConstructor(record) &&
           HasDefaultedBaseCopies(record);
  });
}

bool ConverterRefCount::RecordDerivesClone(const clang::RecordDecl *decl) {
  if (IsClonedShallowly(decl)) {
    return true;
  }
  // The copy of a struct copies each field, which a derived Clone does too.
  auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl);
  return !cxx ||
         (HasCallableCopyConstructor(cxx) &&
          !GetUserDefinedCopyConstructor(cxx) && HasDefaultedBaseCopies(cxx));
}

void ConverterRefCount::AddCloneTrait(const clang::RecordDecl *decl) {
  if (RecordDerivesClone(decl) || !RecordImplementsClone(decl)) {
    return;
  }

  auto record_name = GetRecordName(decl);
  auto *cxx = clang::cast<clang::CXXRecordDecl>(decl);
  StrCat(keyword::kImpl, "Clone for", record_name, '{');
  StrCat("fn clone(&self) -> Self {");

  if (auto *ctor = GetUserDefinedCopyConstructor(cxx)) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    StrCat(std::format("let __src: Value<{}> = {};", record_name,
                       BoxValue(GetShallowCopy(decl, "self"))));
    StrCat(std::format("{}::{}(__src.as_pointer())", record_name,
                       GetCtorName(ctor)));
  } else {
    for (auto ctor : cxx->ctors()) {
      if (ctor->isCopyConstructor()) {
        PushConversionKind push(*this, ConversionKind::FullRefCount);
        ConvertCXXConstructorBody(ctor);
        break;
      }
    }
  }

  StrCat('}');
  StrCat('}');
}

void ConverterRefCount::AddDefaultTrait(const clang::RecordDecl *decl) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  Converter::AddDefaultTrait(decl);
}

void ConverterRefCount::AddDefaultTraitForUnion(const clang::RecordDecl *decl) {
  EmitDefaultImpl(decl, [&] {
    StrCat(std::format("{} {{ __bytes: Box::from([0u8; {}]) }}",
                       GetRecordName(decl),
                       ctx_.getASTRecordLayout(decl).getSize().getQuantity()));
  });
}

void ConverterRefCount::EmitRustUnion(clang::RecordDecl *decl) {
  auto name = GetRecordName(decl);

  EmitDerives(decl);

  auto size = ctx_.getTypeSizeInChars(ctx_.getCanonicalTagType(decl));
  StrCat(std::format(
      "#[derive(ByteRepr, Clone)] #[byte_size({0})] pub struct {1} {{ "
      "#[offset(0)] #[byte_size({0})] __bytes: Box<[u8]> }}",
      size.getQuantity(), name));

  StrCat("impl", name);
  {
    PushBrace impl_brace(*this);
    for (auto *field : decl->fields()) {
      PushConversionKind push(*this, ConversionKind::Unboxed);
      auto type = field->getType();
      if (type->isArrayType()) {
        type = type->getAsArrayTypeUnsafe()->getElementType();
      }
      auto ty = ToString(type);
      StrCat(std::format(
          "pub fn {}(this: Ptr<Self>) -> Ptr<{}> {{ this.reinterpret_cast() }}",
          GetNamedDeclAsString(field), ty));
    }
  }

  AddCloneTrait(decl);
  AddDefaultTrait(decl);
}

void ConverterRefCount::EmitByteSizeAttr(const clang::RecordDecl *decl) {
  StrCat(std::format(
      "#[byte_size({})]",
      ctx_.getTypeSizeInChars(ctx_.getCanonicalTagType(decl)).getQuantity()));
}

std::string
ConverterRefCount::GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) {
  return NeedsMutAccess(ctx_, decl, decl->getThisType()->getPointeeType())
             ? "&mut self"
             : "&self";
}

bool ConverterRefCount::VisitCXXConstructorDecl(
    clang::CXXConstructorDecl *decl) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return Converter::VisitCXXConstructorDecl(decl);
}

bool ConverterRefCount::VisitFieldDecl(clang::FieldDecl *decl) {
  // The C offset of the field, which locates it for pointers to it.
  const auto &layout = ctx_.getASTRecordLayout(decl->getParent());
  StrCat(std::format("#[offset({})]",
                     layout.getFieldOffset(decl->getFieldIndex()) / 8));
  // Only arithmetic types have the same size in Rust as in C.
  auto type = decl->getType().getCanonicalType();
  if (!type->isIntegerType() &&
      !type->isSpecificBuiltinType(clang::BuiltinType::Float) &&
      !type->isSpecificBuiltinType(clang::BuiltinType::Double)) {
    StrCat(std::format("#[byte_size({})]",
                       ctx_.getTypeSizeInChars(decl->getType()).getQuantity()));
  }
  PushConversionKind push(*this, ConversionKind::Pointee);
  return Converter::VisitFieldDecl(decl);
}

void ConverterRefCount::EmitFunctionPreamble(clang::FunctionDecl *decl) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  if (decl->isVariadic()) {
    StrCat("let __args: Value<Box<[VaArg]>> = "
           "Rc::new(RefCell::new(__args.into()));");
  }
  for (auto *param : DefinitionParameters(decl)) {
    auto name = GetNamedDeclAsString(param);
    if (param->getType()->isReferenceType()) {
      if (!HasUsableDefaultArg(param)) {
        continue;
      }
      auto type = ToString(param->getType());
      std::string value;
      if (auto *temp = GetDefaultArgTemporary(param)) {
        auto storage = EmitDefaultArgStorage(param, temp);
        value = std::format("{}.insert(Rc::new(RefCell::new({}))).as_pointer()",
                            storage, ToString(temp->getSubExpr()));
      } else {
        Buffer buf(*this);
        ConvertVarInit(param->getType(), param->getDefaultArg());
        value = std::move(buf).str();
      }
      StrCat(std::format("let {} : {} = {}.unwrap_or_else(|| {})", name, type,
                         name, value),
             token::kSemiColon);
      continue;
    }

    // Unboxed parameters are used in place.
    bool unboxed = IsUnboxedVar(param);
    if (unboxed && !HasUsableDefaultArg(param)) {
      continue;
    }
    auto init = HasUsableDefaultArg(param)
                    ? std::format("{}.unwrap_or_else(|| {})", name,
                                  ToString(param->getDefaultArg()))
                    : name;
    PushConversionKind push(*this, ConversionKind::Unboxed, unboxed);
    StrCat(std::format("let {}{} : {} = {}", unboxed ? "mut " : "", name,
                       ToString(param->getType()), BoxValue(std::move(init))),
           token::kSemiColon);
  }
}

bool ConverterRefCount::ConvertVarDeclSkipInit(clang::VarDecl *decl) {
  PushConversionKind push(*this, VarConversionKind(decl));
  return Converter::ConvertVarDeclSkipInit(decl);
}

void ConverterRefCount::EmitHoistedInArmAssignment(clang::VarDecl *decl) {
  if (!decl->hasInit()) {
    return;
  }

  const auto type = decl->getType();
  if (type->isReferenceType() || IsUnboxedVar(decl)) {
    StrCat(GetNamedDeclAsString(decl));
  } else {
    StrCat(token::kStar, GetNamedDeclAsString(decl), ".borrow_mut()");
  }

  PushConversionKind push(*this, VarConversionKind(decl));
  StrCat(token::kAssign);
  StrCat(ConvertVarInitValue(type, decl->getInit()));
  StrCat(token::kSemiColon);
}

void ConverterRefCount::ConvertGlobalVarDecl(clang::VarDecl *decl) {
  std::string str;
  {
    Buffer buf(*this);
    ConvertVarDecl(decl);
    str = std::move(buf).str();
    if (str.empty()) {
      return;
    }
  }
  StrCat("thread_local!");
  {
    PushParen paren(*this);
    StrCat(str);
  }
  StrCat(token::kSemiColon);
}

bool ConverterRefCount::VisitVarDecl(clang::VarDecl *decl) {
  PushConversionKind push(*this, VarConversionKind(decl));
  PushExprKind push_kind(*this, ExprKind::AddrOf,
                         decl->getType()->isReferenceType());
  Converter::VisitVarDecl(decl);
  return false;
}

void ConverterRefCount::EmitScopedDestructor(const clang::VarDecl *decl) {
  if (!NeedsScopedDestructor(decl)) {
    return;
  }
  auto destroy = AsLambdaClass(decl->getType())
                     ? std::string("__p.with(|__f| __f.destroy())")
                     : std::format("__p.{}()", kDestructorName);
  StrCat(token::kSemiColon,
         std::format("let _dtor_{0} = ScopedDestructor::new(&{0}, |__p| {1})",
                     GetNamedDeclAsString(decl), destroy));
}

bool ConverterRefCount::ConvertIncAndDec(clang::UnaryOperator *expr) {
  auto *sub_expr = expr->getSubExpr();
  const char *method = GetIncDecMethod(expr->getOpcode());
  if (!method) {
    return false;
  }

  auto str = ConvertLValue(sub_expr);
  if (!pending_deref_.empty()) {
    StrCat(pending_deref_.take(), ".with_mut(|__v| __v.", method, "())");
  } else if (!isVoid() &&
             clang::isa<clang::MemberExpr>(sub_expr->IgnoreParens()) &&
             !IsUnboxedPlace(sub_expr)) {
    // The struct is borrowed in a block, which ends the borrow before the
    // result is used, e.g., to write through a pointer into the struct.
    StrCat(std::format("{{ {}.{}() }}", str, method));
  } else {
    StrCat(str, '.', method, "()");
  }
  MarkFresh();
  return true;
}

void ConverterRefCount::ConvertConditionalBranch(clang::Expr *expr,
                                                 clang::QualType type) {
  StrCat(ConvertFresh(expr, type));
}

void ConverterRefCount::ConvertDeclRefValue(clang::Expr *expr,
                                            clang::ValueDecl *decl) {
  if (isAddrOf()) {
    clang::Expr *addrof_op = ToAddrOf(ctx_, expr);
    if (auto str = GetMappedAsString(addrof_op); !str.empty()) {
      EmitFresh(str);
      return;
    }
  }

  if (ShouldReplaceWithMappedBody(decl)) {
    if (auto str = GetMappedAsString(expr); !str.empty()) {
      EmitFresh(str);
      return;
    }
  }

  auto str = ConvertDeclRef(expr, decl);

  if (auto fn_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (isAddrOf()) {
      ConvertFunctionToFunctionPointer(fn_decl);
    } else {
      EmitFresh(str);
    }
    return;
  }

  if (clang::isa<clang::EnumConstantDecl>(decl)) {
    EmitFresh(str);
    return;
  }

  const auto decl_t = GetDeclRefType(curr_function_, expr, decl);
  bool is_global_value = false;
  if (auto *var = clang::dyn_cast<clang::VarDecl>(decl);
      var && IsGlobalVar(var)) {
    if (decl_t->isReferenceType()) {
      str += ".with(Ptr::clone)";
    } else {
      is_global_value = true;
    }
  }

  if (auto *ref = decl_t->getAs<clang::ReferenceType>()) {
    // references are not boxed
    if (isAddrOf()) {
      StrCat(str);
      MarkNotFresh();
    } else {
      if (str == "self") {
        StrCat(str);
      } else {
        EmitPointee(str, ref->getPointeeType(), /*fresh=*/false);
        return;
      }
      SetValueFreshness(expr->getType());
    }
    return;
  }

  if (IsUnboxedVar(decl)) {
    assert(!isAddrOf() && "pointer to an unboxed variable");
    StrCat(str);
    if (isRValue() &&
        moved_reads_.contains(clang::dyn_cast<clang::DeclRefExpr>(expr))) {
      MarkFresh();
    } else {
      SetValueFreshness(expr->getType());
    }
    return;
  }

  if (isAddrOf()) {
    if (is_global_value) {
      StrCat(std::format("{}.with(|v| v{})", std::move(str),
                         AsPointerMethod(decl_t)));
    } else {
      StrCat(str, AsPointerMethod(decl_t));
    }
    MarkFresh();
    return;
  }

  if (isRValue()) {
    if (is_global_value && TypeIsCopyable(decl_t)) {
      StrCat(str, ".with(|rc| *rc.borrow())");
    } else if (is_global_value) {
      // Borrow the global in place, like a local. Cloning it would be
      // expensive for arrays, and writes through its fields would be lost.
      StrCat(std::format("(*{}.with(Value::clone).borrow())", std::move(str)));
    } else {
      StrCat(std::format("(*{}.borrow())", std::move(str)));
    }
  } else {
    if (is_global_value) {
      str += ".with(Value::clone)";
    }
    StrCat(std::format("(*{}.borrow_mut())", std::move(str)));
  }

  // A copied global is read into a new value.
  SetValueFreshness(expr->getType());
}

static std::vector<const char *> printf2fmt(std::string &format) {
  std::vector<const char *> types;
  std::string escaped;
  for (char c : format) {
    escaped += c;
    if (c == '{' || c == '}') {
      escaped += c;
    }
  }
  format = std::move(escaped);
  size_t pos = 0;
  while ((pos = format.find('%', pos)) != std::string::npos) {
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
      format.replace(pos, 2, "%");
      pos += 1;
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
    case 'j':
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
    llvm::errs() << "Unknown printf format: " << format << '\n';
    assert(0);
  }
  return types;
}

void ConverterRefCount::ConvertPrintf(clang::CallExpr *expr) {
  bool is_fprintf = IsCDecl(expr->getDirectCallee(), "fprintf");
  auto format = GetEscapedStringLiteral(clang::cast<clang::StringLiteral>(
      expr->getArg(is_fprintf)->IgnoreImplicit()));
  bool ends_newline = format.ends_with("\\n\"");

  llvm::StringRef stream = "stdout";
  if (is_fprintf) {
    auto loc = expr->getArg(0)->IgnoreParenImpCasts()->getBeginLoc();
    stream = loc.isMacroID()
                 ? clang::Lexer::getImmediateMacroName(
                       loc, ctx_.getSourceManager(), ctx_.getLangOpts())
                 : "";
  }
  if (stream == "stdout") {
    StrCat(ends_newline ? "println!(" : "print!(");
  } else if (stream == "stderr") {
    StrCat(ends_newline ? "eprintln!(" : "eprint!(");
  } else {
    llvm::errs() << "Unknown fprintf fd:\n";
    expr->getArg(0)->dump(llvm::errs(), ctx_);
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
    if (auto *type = types[j++])
      StrCat(keyword::kAs, type);
  }
  StrCat(')');
}

void ConverterRefCount::ConvertVAArgCall(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr)) {
    StrCat(ToString(expr->getArg(0)->IgnoreImpCasts()),
           "= __args.as_pointer()");
    return;
  }
  Converter::ConvertVAArgCall(expr);
}

std::optional<Converter::TempMaterializationCtx>
ConverterRefCount::ConvertCallExpr(clang::CallExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  bool is_fprintf = IsCDecl(expr->getDirectCallee(), "fprintf");
  if ((is_fprintf || IsCDecl(expr->getDirectCallee(), "printf")) &&
      clang::isa<clang::StringLiteral>(
          expr->getArg(is_fprintf)->IgnoreImplicit())) {
    ConvertPrintf(expr);
    return std::nullopt;
  }
  return Converter::ConvertCallExpr(expr);
}

void ConverterRefCount::EmitCallResult(clang::CallExpr *expr, std::string str,
                                       const TempMaterializationCtx *temps) {
  auto wrap = [temps](std::string s) {
    return temps ? temps->Wrap(std::move(s)) : s;
  };
  auto ty = GetReturnTypeOfFunction(expr);
  if (auto *ref = ty->getAs<clang::ReferenceType>();
      ref && !isAddrOf() && !isVoid()) {
    if (isLValue()) {
      pending_deref_.set(wrap(std::move(str)), /*fresh=*/true);
      return;
    }
    // The dereference is in the block of the temporaries, so they are still
    // alive, as is the read of a field of the result.
    auto deref = DerefPtrExpr(str, ref->getPointeeType());
    StrCat(DeferTemporaries(expr, temps) ? deref : wrap(std::move(deref)));
    SetDerefFreshness(ref->getPointeeType());
    return;
  }

  if (isAddrOf() && !ty->isReferenceType() && !IsPointerType(ctx_, ty)) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    StrCat(BoxValue(wrap(std::move(str))), AsPointerMethod(ty));
    return;
  }

  if (temps && !temps->temporary_bindings.empty()) {
    str = std::format("({})", temps->Wrap(std::move(str)));
  }
  StrCat(str);
  if (!IsPassThroughRule(expr)) {
    MarkFresh();
  }
}

std::string ConverterRefCount::GetSourceFileAsString(clang::QualType type) {
  return std::format("Ptr::<{}>::from_string_literal(file!().as_bytes())",
                     ToStringBase(type->getPointeeType().getUnqualifiedType()));
}

bool ConverterRefCount::VisitStringLiteral(clang::StringLiteral *expr) {
  if (IsCodeUnitStringLiteral(expr)) {
    auto arr = GetCodeUnitArrayLiteral(expr);
    if (!IsArrayInitContext()) {
      arr = '&' + arr;
    } else if (getConversionKind() != ConversionKind::Unboxed) {
      arr = std::format("Box::from({})", arr);
    }
    EmitFresh(arr);
    return false;
  }

  if (IsArrayInitContext()) {
    // The array may have a different element type than the literal, e.g.,
    // unsigned char s[] = "abc".
    auto elem_ty =
        ctx_.getAsArrayType(curr_init_type_.back())->getElementType();
    std::string elem(Trim(ToStringBase(elem_ty.getUnqualifiedType())));
    uint64_t pad = 1;
    if (auto *arr_ty = ctx_.getAsConstantArrayType(curr_init_type_.back())) {
      uint64_t arr_size = arr_ty->getSize().getZExtValue();
      if (expr->getString().empty()) {
        auto zeros = std::format("[0{}; {}]", elem, arr_size);
        EmitFresh(getConversionKind() == ConversionKind::Unboxed
                      ? zeros
                      : std::format("vec!{}.into_boxed_slice()", zeros));
        return false;
      }
      pad = arr_size > expr->getString().size()
                ? arr_size - expr->getString().size()
                : 0;
    }
    auto literal = GetEscapedStringLiteral(expr, pad);
    EmitFresh(getConversionKind() == ConversionKind::Unboxed
                  ? std::format("b{}.map({}::from_byte)", literal, elem)
                  : std::format("{}::array_from_literal(b{})", elem, literal));
    return false;
  }
  EmitFresh(std::format("b{}", GetEscapedStringLiteral(expr, 0)));
  return false;
}

bool ConverterRefCount::VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
  auto *sub_expr = expr->getSubExpr();

  if (expr->isXValue() && sub_expr->isLValue()) {
    Convert(sub_expr);
    if (!isFresh()) {
      MarkNotFresh();
    }
    return false;
  }

  if (expr->getCastKind() == clang::CastKind::CK_BitCast) {
    if (expr->getType()->isVoidPointerType()) {
      if (sub_expr->getType()->isVoidPointerType()) {
        return Convert(sub_expr);
      }
      PushConversionKind push(*this, ConversionKind::Unboxed);
      auto ptr = ConvertPointer(sub_expr);
      if (!isFresh()) {
        StrCat(std::format("({}).to_any()", ptr));
      } else if (sub_expr->getType()->isPointerType() &&
                 sub_expr->getType()->getPointeeType()->isArrayType()) {
        // A pointer to an array points to its first element.
        StrCat(std::format(
            "({} as {}).to_any()", ptr,
            ToString(ctx_.getPointerType(sub_expr->getType()
                                             ->getPointeeType()
                                             ->getAsArrayTypeUnsafe()
                                             ->getElementType()))));
      } else if (IsStringLiteralExpr(sub_expr)) {
        StrCat(std::format("{}.to_any()", ptr));
      } else {
        StrCat(std::format("({} as {}).to_any()", ptr,
                           ToString(sub_expr->getType())));
      }
      MarkFresh();
    } else if (sub_expr->getType()->isVoidPointerType() &&
               expr->getType()->isPointerType()) {
      Convert(sub_expr);
      EmitFresh(std::format(".reinterpret_cast::<{}>()",
                            ConvertPointeeType(expr->getType())));
    } else {
      Convert(sub_expr);
    }
    return false;
  }

  if (expr->getCastKind() == clang::CastKind::CK_DerivedToBase) {
    if (expr->getType()->isPointerType()) {
      auto ptype = clang::dyn_cast<clang::PointerType>(expr->getType());
      auto pointee_type = ptype->getPointeeType()->getAsCXXRecordDecl();

      if (pointee_type && IsAbstractUserClass(pointee_type)) {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        EmitFresh(std::format("{}.to_dyn::<{}>(|w| w)",
                              ToString(sub_expr->IgnoreCasts()),
                              ConvertPointeeType(expr->getType())));
        return false;
      }
    }
  }

  if (expr->getCastKind() == clang::CastKind::CK_ArrayToPointerDecay) {
    if (IsVaListType(sub_expr->getType())) {
      Convert(sub_expr);
      return false;
    }
    if (IsStringLiteralExpr(sub_expr)) {
      EmitFresh(ConvertStringLiteralPtr(sub_expr));
      return false;
    } else {
      // we need to write (var.as_pointer as Ptr<T>) because Rust isn't
      // smart enough to pick the right specialization
      PushConversionKind push(*this, ConversionKind::Unboxed);
      PushParen paren(*this);
      StrCat(ConvertPointer(sub_expr));
      if (!IsReferenceType(sub_expr)) {
        StrCat(keyword::kAs);
        if (expr->getType()->getPointeeType()->isBuiltinType()) {
          StrCat(std::format("Ptr::<{}>", ConvertPointeeType(expr->getType())));
        } else {
          StrCat(ToString(expr->getType()));
        }
      }
      return false;
    }
  }

  if (expr->getCastKind() == clang::CastKind::CK_NullToPointer) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    EmitFresh(GetDefaultAsString(expr->getType()));
    return false;
  }

  if (expr->getCastKind() == clang::CastKind::CK_NoOp) {
    Convert(sub_expr);
    return false;
  }

  return Converter::VisitImplicitCastExpr(expr);
}

void ConverterRefCount::EmitFnPtrCall(clang::CallExpr *expr) {
  auto callee = GetCallee(expr);
  if (auto lambda = AsLambdaClass(callee->getType())) {
    ConvertLambdaCall(expr, lambda);
    return;
  }
  Convert(callee);
  StrCat(".call");
}

bool ConverterRefCount::VisitLambdaExpr(clang::LambdaExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::VisitLambdaExpr(expr);
}

void ConverterRefCount::ConvertCapturelessLambda(
    const clang::CXXRecordDecl *decl) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  Converter::ConvertCapturelessLambda(decl);
}

void ConverterRefCount::ConvertLambdaCapture(const clang::FieldDecl *field,
                                             clang::Expr *init) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  Converter::ConvertLambdaCapture(field, init);
}

void ConverterRefCount::ConvertLambdaToFunctionPointer(
    clang::Expr *lambda, const clang::CXXConversionDecl *conversion) {
  StrCat(ConvertFreshRValue(lambda));
  auto decl = AsLambdaClass(lambda->getType());
  if (decl->isGenericLambda()) {
    auto call_operator = AsLambdaCallOperator(conversion);
    auto proto = call_operator->getType()->castAs<clang::FunctionProtoType>();
    StrCat(std::format(".spec::<{}>({})", ConvertFunctionPointerType(proto),
                       GetLambdaSpecializationIndex(call_operator)));
  }
}

void ConverterRefCount::ConvertFunctionToFunctionPointer(
    const clang::FunctionDecl *fn_decl) {
  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(fn_decl);
      method && method->isLambdaStaticInvoker()) {
    ConvertCapturelessLambda(method->getParent());
    MarkFresh();
    return;
  }
  EmitFresh(
      std::format("FnPtr::<{}>::new({})",
                  ConvertFunctionPointerType(
                      fn_decl->getType()->getAs<clang::FunctionProtoType>()),
                  GetFunctionRefName(fn_decl)));
}

std::string ConverterRefCount::ConvertFnPtrPlaceholder(clang::Expr *arg) {
  return ConvertFnPtrCallee(arg);
}

void ConverterRefCount::ConvertEqualsNullPtr(clang::Expr *expr) {
  StrCat('(');
  Convert(expr);
  EmitFresh(").is_null()");
}

bool ConverterRefCount::ConvertFunctionPointerCast(
    clang::ExplicitCastExpr *expr) {
  bool from_fn = expr->getSubExpr()->getType()->isFunctionPointerType();
  bool to_fn = expr->getType()->isFunctionPointerType();
  if (!from_fn && !to_fn) {
    return false;
  }
  auto *target_proto =
      to_fn
          ? expr->getType()->getPointeeType()->getAs<clang::FunctionProtoType>()
          : nullptr;
  if (from_fn && to_fn) {
    StrCat(std::format("{}.cast::<{}>()", ToString(expr->getSubExpr()),
                       ConvertFunctionPointerType(target_proto)));
  } else if (from_fn) {
    Convert(expr->getSubExpr());
    StrCat(".to_any()");
  } else {
    StrCat(std::format("{}.cast_fn::<{}>().expect(\"ub:wrong fn type\")",
                       ToString(expr->getSubExpr()),
                       ConvertFunctionPointerType(target_proto)));
  }
  MarkFresh();
  return true;
}

bool ConverterRefCount::VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
  if (expr->getTypeAsWritten()->isVoidType()) {
    return Converter::VisitExplicitCastExpr(expr);
  }
  if (expr->getCastKind() == clang::CK_NullToPointer) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    EmitFresh(GetDefaultAsString(expr->getType()));
    return false;
  }
  switch (expr->getStmtClass()) {
  case clang::Stmt::CXXReinterpretCastExprClass:
    assert(expr->getType()->isPointerType() &&
           "Only pointer casts are supported in reinterpret_cast");
    EmitFresh(
        std::format("{}.reinterpret_cast::<{}>()", ToString(expr->getSubExpr()),
                    GetUnsafeTypeAsString(expr->getType()->getPointeeType())));
    return false;
  case clang::Stmt::CStyleCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass:
    if (expr->getCastKind() == clang::CastKind::CK_PointerToIntegral ||
        expr->getCastKind() == clang::CastKind::CK_IntegralToPointer) {
      std::string dst_type;
      {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        dst_type = ToString(expr->getType());
      }
      if (expr->getCastKind() == clang::CastKind::CK_PointerToIntegral) {
        EmitFresh(std::format("{}.to_int()", ToString(expr->getSubExpr())));
      } else {
        EmitFresh(std::format("<{}>::from_int({})", dst_type,
                              ToString(expr->getSubExpr())));
      }
      return false;
    }

    if (ConvertFunctionPointerCast(expr)) {
      return false;
    } else if (expr->getSubExpr()->getType()->isVoidPointerType() &&
               expr->getType()->isVoidPointerType()) {
      return Convert(expr->getSubExpr());
    } else if (expr->getSubExpr()->getType()->isVoidPointerType() &&
               expr->getType()->isPointerType()) {
      Convert(expr->getSubExpr());
      EmitFresh(std::format(".reinterpret_cast::<{}>()",
                            ConvertPointeeType(expr->getType())));
      return false;
    } else if (expr->getType()->isVoidPointerType() &&
               expr->getSubExpr()->getType()->isPointerType()) {
      EmitFresh(
          std::format("({}).to_any()", ConvertPointer(expr->getSubExpr())));
      return false;
    } else if (expr->getSubExpr()->getType()->isPointerType() &&
               !expr->getSubExpr()->isNullPointerConstant(
                   ctx_, clang::Expr::NPC_ValueDependentIsNull)) {
      auto sub_expr = expr->getSubExpr();
      if (auto *array_type =
              ctx_.getAsArrayType(sub_expr->getType()->getPointeeType())) {
        // A pointer to an array is a pointer to its first element.
        auto element_type = array_type->getElementType();
        PushConversionKind push(*this, ConversionKind::Unboxed);
        StrCat(std::format("({} as Ptr<{}>)", ToString(sub_expr),
                           ToString(element_type)));
        if (!ctx_.hasSameUnqualifiedType(element_type,
                                         expr->getType()->getPointeeType())) {
          StrCat(std::format(".reinterpret_cast::<{}>()",
                             ConvertPointeeType(expr->getType())));
        }
        MarkFresh();
        return false;
      }
      EmitFresh(std::format("{}.reinterpret_cast::<{}>()", ToString(sub_expr),
                            ConvertPointeeType(expr->getType())));
      return false;
    }
    return Converter::VisitExplicitCastExpr(expr);
  case clang::Stmt::CXXFunctionalCastExprClass:
    return Converter::VisitExplicitCastExpr(expr);
  default:
    return Convert(expr->getSubExpr());
  }
}

bool ConverterRefCount::VisitUnaryExprOrTypeTraitExpr(
    clang::UnaryExprOrTypeTraitExpr *expr) {
  // TODO: Once Values are dropped from fields, precomputation should be gone
  auto arg_type = expr->getTypeOfArgument();
  auto kind = expr->getKind();
  bool is_size = kind == clang::UnaryExprOrTypeTrait::UETT_SizeOf;
  bool is_align = kind == clang::UnaryExprOrTypeTrait::UETT_AlignOf ||
                  kind == clang::UnaryExprOrTypeTrait::UETT_PreferredAlignOf;
  if ((is_size || is_align) && RustSizeDivergesFromC(arg_type)) {
    EmitFresh(std::format("{}usize", (is_size ? ctx_.getTypeSize(arg_type)
                                              : ctx_.getTypeAlign(arg_type)) /
                                         8));
    return false;
  }
  return Converter::VisitUnaryExprOrTypeTraitExpr(expr);
}

bool ConverterRefCount::VisitReturnStmt(clang::ReturnStmt *stmt) {
  auto *value = stmt->getRetValue();
  ScopedValue moved_reads(moved_reads_, value ? FindMovableReads(ctx_, value)
                                              : decltype(moved_reads_){});
  Converter::VisitReturnStmt(stmt);
  return false;
}

bool ConverterRefCount::VisitStmtExpr(clang::StmtExpr *expr) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return Converter::VisitStmtExpr(expr);
}

void ConverterRefCount::ConvertBinaryOperator(clang::BinaryOperator *expr) {
  auto *lhs = expr->getLHS();
  auto *rhs = expr->getRHS();
  auto lhs_type = lhs->getType();
  auto rhs_type = rhs->getType();
  std::string_view opcode_as_string = expr->getOpcodeStr();

  if (auto *assign = llvm::dyn_cast<clang::CompoundAssignOperator>(expr);
      assign && GetSafeTypeAsString(lhs_type) !=
                    GetSafeTypeAsString(assign->getComputationResultType())) {
    EmitCompoundSetOrAssign(lhs, rhs, ConvertCompoundAssignValue(assign));
    return;
  }

  if (IsUnsignedArithOp(expr)) {
    std::optional<Buffer> buf;
    if (expr->isCompoundAssignmentOp()) {
      buf.emplace(*this);
    }
    {
      PushParen paren(*this);
      if (expr->isCompoundAssignmentOp() && lhs->isLValue()) {
        StrCat(ConvertRValue(lhs));
      } else {
        ConvertUnsignedArithOperand(lhs, expr->getType());
      }
    }
    ConvertUnsignedArithBinaryOperator(expr, rhs);
    if (expr->isCompoundAssignmentOp()) {
      auto value = std::move(*buf).str();
      buf.reset();
      EmitCompoundSetOrAssign(lhs, rhs, value);
    } else {
      MarkFresh();
    }
    return;
  }

  // pointer subtraction. The Sub trait gets elements by Value, so we need
  // fresh pointers
  if (expr->isAdditiveOp() && lhs_type->isPointerType() &&
      rhs_type->isPointerType()) {
    {
      PushParen paren(*this);
      StrCat(ConvertFreshPointer(lhs), expr->getOpcodeStr(),
             ConvertFreshPointer(rhs));
    }
    ConvertCast(expr->getType());
    MarkFresh();
    return;
  }

  if (expr->isAssignmentOp()) {
    ConvertAssignment(lhs, rhs, opcode_as_string);
    return;
  }

  Converter::ConvertBinaryOperator(expr);
}

void ConverterRefCount::ConvertScalarInitList(clang::InitListExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  Converter::ConvertScalarInitList(expr);
}

void ConverterRefCount::ConvertRecordInitList(clang::InitListExpr *expr,
                                              bool is_empty) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  Converter::ConvertRecordInitList(expr, is_empty);
}

void ConverterRefCount::ConvertUnionInitList(clang::InitListExpr *expr,
                                             const clang::FieldDecl *field) {
  auto qual_type = expr->getType();
  auto name = GetUnsafeTypeAsString(qual_type);
  StrCat(std::format("{{ let __u: Value<{}> = Rc::new(RefCell::new({}));", name,
                     GetDefaultAsString(qual_type)));
  StrCat(std::format("{}::{}(__u.as_pointer()).write(", name,
                     GetNamedDeclAsString(field)));
  ConvertVarInit(field->getType(), expr->getInit(0));
  StrCat("); Rc::try_unwrap(__u).ok().unwrap().into_inner() }");
}

void ConverterRefCount::ConvertArrayInitList(clang::InitListExpr *expr) {
  auto conv = getConversionKind();
  bool rows =
      expr->getNumInits() > 0 && expr->getInit(0)->getType()->isArrayType();
  PushConversionKind push(*this,
                          rows && conv == ConversionKind::FullRefCount
                              ? ConversionKind::Pointee
                              : ConversionKind::Unboxed,
                          !rows || conv == ConversionKind::FullRefCount);

  switch (conv) {
  case ConversionKind::Unboxed:
  case ConversionKind::Ptr:
    Converter::ConvertArrayInitList(expr);
    break;
  case ConversionKind::Pointee:
  case ConversionKind::FullRefCount:
    StrCat("Box::new(");
    Converter::ConvertArrayInitList(expr);
    StrCat(')');
    break;
  }
}

bool ConverterRefCount::VisitCXXStdInitializerListExpr(
    clang::CXXStdInitializerListExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::VisitCXXStdInitializerListExpr(expr);
}

void ConverterRefCount::ConvertUnionMemberAccessor(clang::MemberExpr *expr) {
  auto member = expr->getMemberDecl();
  auto *record = clang::cast<clang::RecordDecl>(member->getDeclContext());
  auto name = GetNamedDeclAsString(member);
  auto ptr = ConvertRecordPtr(expr);
  auto str = std::format("{}::{}({}{})", GetRecordName(record), name, ptr,
                         isFresh() ? "" : ".clone()");

  if (isAddrOf()) {
    if (member->getType()->isArrayType()) {
      PushConversionKind push(*this, ConversionKind::Unboxed);
      EmitFresh(std::format(
          "{}.reinterpret_cast::<{}>()", str,
          ToString(
              member->getType()->getAsArrayTypeUnsafe()->getElementType())));
    } else {
      StrCat(str);
      MarkNotFresh();
    }
    return;
  }

  EmitPointee(str, member->getType(), /*fresh=*/true);
}

bool ConverterRefCount::VisitMemberExpr(clang::MemberExpr *expr) {
  auto *member = expr->getMemberDecl();
  if (!member->isCXXInstanceMember()) {
    ConvertDeclRefValue(expr, member);
    return false;
  }
  if (auto field = clang::dyn_cast<clang::FieldDecl>(member);
      field && AsLambdaCapture(field)) {
    ConvertDeclRefValue(expr, member);
    return false;
  }
  bool known = Mapper::Contains(ctx_, expr);

  if (known && RuleRegistry::ReturnsPointer(ctx_, expr)) {
    auto str = GetMappedAsString(expr);
    if (isAddrOf()) {
      EmitFresh(str);
      return false;
    }
    EmitPointee(str, expr->getType(), /*fresh=*/true);
    return false;
  }

  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && !known) {
    if (IsMethodOnPtr(method)) {
      SetUFCSReceiver(expr->getBase(), expr->isArrow(), method);
      StrCat(TraitName(method->getParent()), token::kDoubleColon,
             GetMethodName(method));
      MarkFresh();
      return false;
    }
    // Non-user-defined types (STL) need a mutable borrow for non-const
    // methods, as do virtual methods.
    auto base_type = expr->getBase()->getType().getNonReferenceType();
    if (base_type->isPointerType()) {
      base_type = base_type->getPointeeType();
    }
    bool needs_mut = NeedsMutAccess(ctx_, method, base_type);
    PushExprKind push(*this, needs_mut ? ExprKind::LValue : ExprKind::RValue);
    Converter::ConvertMemberExpr(expr);
    MarkFresh();
    return false;
  }

  if (auto *parent =
          clang::dyn_cast<clang::RecordDecl>(member->getDeclContext());
      parent && parent->isUnion() && clang::isa<clang::FieldDecl>(member)) {
    ConvertUnionMemberAccessor(expr);
    return false;
  }

  if (auto *field = clang::dyn_cast<clang::FieldDecl>(member);
      field && !known && !field->getType()->isReferenceType()) {
    ConvertInlineField(expr);
    return false;
  }

  std::string str;
  if (known) {
    str = GetMappedAsString(expr);
  } else if (clang::isa<clang::FieldDecl>(member)) {
    // The pointer that the reference field holds, which is used in place in
    // an unboxed struct.
    str = ReadField(expr, IsFieldOfUnboxedPlace(expr) ? "" : ".clone()");
  } else {
    Buffer buf(*this);
    PushExprKind push(*this, ExprKind::RValue);
    Converter::ConvertMemberExpr(expr);
    str = std::move(buf).str();
  }

  if (isAddrOf()) {
    StrCat(str);
    if (member->getType()->isReferenceType()) {
      MarkNotFresh();
    } else {
      EmitFresh(AsPointerMethod(member->getType()));
    }
    return false;
  }

  if (member->getType()->isReferenceType()) {
    EmitPointee(str, member->getType().getNonReferenceType(),
                /*fresh=*/false);
    return false;
  }
  if (isRValue()) {
    StrCat(std::format("(*{}.borrow())", std::move(str)));
  } else {
    StrCat(std::format("(*{}.borrow_mut())", std::move(str)));
  }
  SetValueFreshness(expr->getType());
  return false;
}

void ConverterRefCount::ConvertInlineField(clang::MemberExpr *expr) {
  auto name = GetNamedDeclAsString(expr->getMemberDecl());
  auto type = expr->getType();
  auto *base = expr->getBase();
  if (isAddrOf()) {
    // A pointer to the struct, offset to the field. An array decays to a
    // pointer to its first element, and a vector can be used as one.
    if (type->isArrayType()) {
      StrCat(std::format("array_field_ptr!({}, {})", ConvertRecordPtr(expr),
                         name));
    } else if (IsBoxedType(ctx_, type)) {
      StrCat(std::format("field_ref!({}, {}){}", ConvertRecordPtr(expr), name,
                         AsPointerMethod(type)));
    } else {
      StrCat(std::format("field_ptr!({}, {})", ConvertRecordPtr(expr), name));
    }
    MarkFresh();
    return;
  }

  // Pointers, and unique_ptr, which holds a Value, are copied out too, as is
  // any field that is copied.
  if (isRValue() && !record_ptr_ &&
      (TypeIsCopyable(type) || type->isPointerType() || IsUniquePtr(type) ||
       expr == copied_expr_)) {
    // A pointer in an unboxed struct is used in place, like a pointer
    // variable, and only copied when needed.
    if (type->isPointerType() && IsUnboxedPlace(expr)) {
      StrCat(ReadField(expr, ""));
      MarkNotFresh();
      return;
    }
    EmitFresh(ReadField(expr));
    return;
  }

  // The struct, or a struct that holds it, in the closure of a read.
  std::string str;
  {
    Buffer buf(*this);
    PushRecordPtr push(*this, record_ptr_, expr->isArrow() ? nullptr : base);
    Converter::ConvertMemberExpr(expr);
    str = std::move(buf).str();
  }
  // A field of a struct reached through a pointer is written through the
  // pointer, projected to the field.
  if (!pending_deref_.empty()) {
    pending_deref_.set(
        std::format("field!({}, {})", pending_deref_.take(), name),
        /*fresh=*/true);
    return;
  }
  StrCat(str);
  SetValueFreshness(type);
}

std::string ConverterRefCount::ConvertRecordPtr(clang::MemberExpr *expr) {
  Buffer buf(*this);
  PushExprKind push(*this, ExprKind::AddrOf);
  if (expr->isArrow()) {
    ConvertArrow(expr->getBase());
  } else {
    Convert(expr->getBase());
  }
  return std::move(buf).str();
}

std::string ConverterRefCount::ReadField(clang::MemberExpr *expr,
                                         std::string_view copy) {
  // Unless the struct is reached through a pointer, in which case the field
  // is read in a closure, the borrow of the struct ends at the end of the
  // block.
  std::string ptr;
  std::string str;
  // The read of a field of the result of a call is in the block of the
  // temporaries of the arguments of the call.
  FieldReadCall read{GetFieldReadCall(expr), ""};
  {
    ScopedValue push_read(field_read_call_, read.call ? &read : nullptr);
    Buffer buf(*this);
    PushRecordPtr push(*this, &ptr,
                       expr->isArrow() ? nullptr : expr->getBase());
    PushExprKind push_kind(*this, ExprKind::RValue);
    Converter::ConvertMemberExpr(expr);
    str = std::move(buf).str();
  }
  if (!TypeIsCopyable(expr->getMemberDecl()->getType())) {
    str += copy;
  }
  if (IsFieldOfUnboxedPlace(expr)) {
    return str;
  }
  if (!ptr.empty()) {
    str = std::format("{}.with(|__s| {})", ptr, str);
  } else if (read.temporary_bindings.empty()) {
    str = std::format("{{ {} }}", str);
  }
  if (read.temporary_bindings.empty()) {
    return str;
  }
  return std::format("{{ {} {} }}", read.temporary_bindings, str);
}

bool ConverterRefCount::VisitCXXNewExpr(clang::CXXNewExpr *expr) {
  if (expr->isArray()) {
    if (auto *init = llvm::dyn_cast_or_null<clang::InitListExpr>(
            expr->getInitializer())) {
      StrCat("Ptr::alloc_array(");
      PushConversionKind push(*this, ConversionKind::FullRefCount);
      Convert(init);
      StrCat(')');
    } else {
      auto array_size_as_string = ToString(*expr->getArraySize());
      auto alloc_type = expr->getAllocatedType();
      PushConversionKind push(*this, alloc_type->isArrayType()
                                         ? ConversionKind::Pointee
                                         : ConversionKind::Unboxed);
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
    StrCat(')');
  }
  MarkFresh();
  return false;
}

bool ConverterRefCount::VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) {
  if (!TypeNeedsDestruction(expr->getDestroyedType())) {
    Convert(expr->getArgument());
    StrCat(".delete()");
    return false;
  }

  PushBrace brace(*this);
  StrCat(keyword::kLet, "__p", token::kAssign, ToString(expr->getArgument()),
         token::kSemiColon);
  if (expr->isArrayForm()) {
    StrCat(std::format("for __i in 0..__p.len() {{ __p.offset(__i as "
                       "isize).{}(); }}",
                       kDestructorName));
  } else {
    StrCat(std::format("__p.{}();", kDestructorName));
  }
  StrCat("__p.delete();");
  return false;
}

bool ConverterRefCount::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr *expr) {
  // Arrays are boxed slices, also when they are elements of arrays.
  StrCat(array_init_depth_ == 0 ? "Box::new" : "Box::<[_]>::from");
  PushParen outer(*this);
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::VisitArrayInitLoopExpr(expr);
}

void ConverterRefCount::ConvertArrayCXXConstructExpr(
    clang::CXXConstructExpr *expr) {
  bool boxed = getConversionKind() != ConversionKind::Unboxed;
  if (boxed) {
    StrCat("Box::new");
  }
  PushParen outer(*this, boxed);
  StrCat(std::format("std::array::from_fn::<_, {}, _>",
                     GetArraySize(expr->getType())));
  PushParen inner(*this);
  StrCat("|_|");
  PushConversionKind push(*this, ConversionKind::Unboxed);
  ConvertCXXConstructExprArgs(expr);
}

std::string ConverterRefCount::ConvertStream(clang::Expr *expr) {
  return ConvertPointer(expr);
}

bool ConverterRefCount::VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
  if (expr->isElidable()) {
    Convert(expr->getArg(0));
    return false;
  }

  // An array is boxed unless it initializes an unboxed variable.
  bool boxed_array = expr->getType()->isArrayType() &&
                     getConversionKind() != ConversionKind::Unboxed;
  PushConversionKind push(*this, boxed_array ? ConversionKind::Pointee
                                             : ConversionKind::Unboxed);

  if (auto str = GetMappedAsString(expr, expr->getArgs(), expr->getNumArgs());
      !str.empty()) {
    if (isAddrOf()) {
      EmitFresh(std::format("Rc::new(RefCell::new({})){}", std::move(str),
                            AsPointerMethod(expr->getType())));
    } else {
      StrCat(str);
      if (!IsPassThroughRule(expr)) {
        MarkFresh();
      }
    }
    return false;
  }

  auto *ctor = expr->getConstructor();
  if (IsLambdaCopyOrMoveConstructor(ctor)) {
    StrCat(ConvertRValue(expr->getArg(0)),
           std::format(".{}()", GetCopyOrMoveName(ctor)));
    MarkFresh();
    return false;
  }

  if (IsRValueConvertingConstructor(ctor) ||
      (ctor->isMoveConstructor() && !IsUserDefinedDecl(ctor->getParent()))) {
    StrCat(ConvertLValue(expr->getArg(0)));
    return false;
  }

  if (ctor->isCopyOrMoveConstructor() &&
      !IsConvertibleCopyOrMoveConstructor(ctor)) {
    StrCat(ConvertFreshRValue(expr->getArg(0)));
    return false;
  }

  if (ctor->isDefaultConstructor() &&
      !(ctor->isUserProvided() && IsUserDefinedDecl(ctor))) {
    auto ty = expr->getType();
    EmitFresh(GetDefaultAsString(ty));
    return false;
  }

  if (expr->getType()->isArrayType()) {
    ConvertArrayCXXConstructExpr(expr);
  } else {
    ConvertCXXConstructExprArgs(expr);
  }
  MarkFresh();

  return false;
}

bool ConverterRefCount::VisitImplicitValueInitExpr(
    clang::ImplicitValueInitExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  if (ctx_.getAsConstantArrayType(expr->getType())) {
    StrCat("Box::new(");
    Converter::VisitImplicitValueInitExpr(expr);
    StrCat(')');
    MarkFresh();
    return false;
  }

  return Converter::VisitImplicitValueInitExpr(expr);
}

bool ConverterRefCount::VisitCXXScalarValueInitExpr(
    clang::CXXScalarValueInitExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::VisitCXXScalarValueInitExpr(expr);
}

void ConverterRefCount::ConvertVariadicArg(clang::Expr *arg) {
  if (arg->getType()->isRecordType()) {
    StrCat(ConvertFreshRValue(arg));
    return;
  }
  if (arg->getType()->isPointerType()) {
    StrCat(ConvertFreshPointer(arg));
    return;
  }
  Convert(arg);
}

bool ConverterRefCount::VisitVAArgExpr(clang::VAArgExpr *expr) {
  auto va_list_expr = expr->getSubExpr();
  if (auto *cast = clang::dyn_cast<clang::ImplicitCastExpr>(va_list_expr)) {
    va_list_expr = cast->getSubExpr();
  }
  auto str = ConvertLValue(va_list_expr);
  std::string arg_type;
  {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    arg_type = ToString(expr->getType());
  }
  if (!pending_deref_.empty()) {
    StrCat(pending_deref_.take(), ".with_mut(|__v| __v.arg::<", arg_type,
           ">())");
  } else {
    StrCat(str, ".arg::<", arg_type, ">()");
  }
  MarkFresh();
  return false;
}

std::string
ConverterRefCount::GetArrayDefaultAsString(clang::QualType qual_type) {
  if (auto *array_type =
          clang::dyn_cast<clang::IncompleteArrayType>(qual_type)) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    return std::format("Box::<[{}]>::default()",
                       ToString(array_type->getElementType()));
  }
  if (auto *array_type = clang::dyn_cast<clang::ConstantArrayType>(qual_type)) {
    const auto &size = array_type->getSize();
    auto size_as_string = GetNumAsString(size);
    auto element_type = array_type->getElementType();
    // An array stored directly, like an unboxed variable.
    if (getConversionKind() == ConversionKind::Unboxed &&
        !element_type->isArrayType()) {
      auto default_as_string = GetDefaultAsString(element_type);
      if (TypeIsCopyable(element_type)) {
        return std::format("[{}; {}]", default_as_string,
                           size_as_string.c_str());
      }
      return std::format("std::array::from_fn::<_, {}, _>(|_| {})",
                         size_as_string.c_str(), default_as_string);
    }
    PushConversionKind push(*this, element_type->isArrayType()
                                       ? ConversionKind::Pointee
                                       : ConversionKind::Unboxed);
    auto element_type_as_string = ToString(element_type);
    auto default_as_string = GetDefaultAsString(element_type);
    return std::format("(0..{}).map(|_| {}).collect::<Box<[{}]>>()",
                       size_as_string.c_str(), default_as_string,
                       element_type_as_string);
  }
  return Converter::GetArrayDefaultAsString(qual_type);
}

std::string ConverterRefCount::GetDefaultAsString(clang::QualType qual_type) {
  if (auto lambda = AsLambdaClass(qual_type);
      lambda && lambda->captures().empty()) {
    return BoxValue(Converter::GetDefaultAsString(qual_type));
  }

  if (IsVaListType(qual_type)) {
    return BoxValue("Ptr::<VaArg>::default()");
  }

  if (auto arr = GetArrayDefaultAsString(qual_type); !arr.empty()) {
    return BoxValue(std::move(arr));
  }

  if (auto init = Mapper::MapInitializer(*this, qual_type); !init.empty()) {
    return BoxValue(std::move(init));
  }

  if (!qual_type->isPointerType()) {
    return Converter::GetDefaultAsString(qual_type);
  }
  std::string ret;
  auto pointee_type = qual_type->getPointeeType();
  if (pointee_type->isFunctionType()) {
    auto *proto = pointee_type->getAs<clang::FunctionProtoType>();
    assert(proto && "Function pointer default without a prototype");
    ret = std::format("FnPtr::<{}>::null()", ConvertFunctionPointerType(proto));
  } else {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    ret = std::format("Ptr::<{}>::null()", ConvertPointeeType(qual_type));
  }
  return BoxValue(std::move(ret));
}

std::string
ConverterRefCount::GetDefaultAsStringFallback(clang::QualType qual_type) {
  return std::format("<{}>::default()", ToString(qual_type));
}

std::string
ConverterRefCount::ConvertVarDefaultInit(const clang::VarDecl *decl) {
  PushConversionKind push(*this, VarConversionKind(decl));
  return GetDefaultAsString(decl->getType());
}

std::vector<const char *>
ConverterRefCount::GetStructAttributes(const clang::RecordDecl *decl) {
  std::vector<const char *> attrs;

  if (decl->isUnion()) {
    return {"VaArg", "FnPtrArg"};
  }

  if (RecordDerivesClone(decl)) {
    attrs.emplace_back("Clone");
  }

  // Gives access to the fields through pointers to them.
  attrs.emplace_back("Record");
  // Gives access to the bytes of the struct, at the offsets of its fields.
  attrs.emplace_back("ByteRepr");

  if (RecordImplementsClone(decl)) {
    attrs.emplace_back("VaArg");
  }
  attrs.emplace_back("FnPtrArg");

  if (HasMoveFromConstructor(decl)) {
    attrs.emplace_back("MoveCtor");
  }

  if (HasImplicitConversions(decl)) {
    attrs.emplace_back("CppFrom");
  }

  if (HasCopyAssignOperator(decl)) {
    attrs.emplace_back("CopyAssign");
  }

  if (HasMoveAssignOperator(decl)) {
    attrs.emplace_back("MoveAssign");
  }

  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && RecordNeedsDestruction(cxx)) {
    attrs.emplace_back("Destructor");
  }

  if (RecordDerivesDefault(decl)) {
    attrs.emplace_back("Default");
  }
  return attrs;
}

const char *ConverterRefCount::GetConversionTraitName() const {
  return "CppFrom";
}

bool ConverterRefCount::TypeDerivesDefault(clang::QualType qual_type) {
  // Arrays are translated to Value<Box<[T]>>, whose Default is empty
  return !qual_type->isArrayType() && Converter::TypeDerivesDefault(qual_type);
}

std::string ConverterRefCount::ConvertVarInitValue(clang::QualType qual_type,
                                                   clang::Expr *expr) {
  if (auto lambda = clang::dyn_cast<clang::LambdaExpr>(
          expr->IgnoreUnlessSpelledInSource())) {
    Buffer buf(*this);
    VisitLambdaExpr(lambda);
    return std::move(buf).str();
  }

  ScopedPush init_type(curr_init_type_, qual_type);
  if (qual_type->isReferenceType() || qual_type->isFunctionPointerType()) {
    if (llvm::isa<clang::MaterializeTemporaryExpr>(expr->IgnoreImpCasts())) {
      return EmitMaterializedTempBinding(qual_type, expr);
    }
    if (qual_type.getNonReferenceType()->isArrayType() &&
        IsStringLiteralExpr(expr)) {
      return ConvertStringLiteralPtr(expr);
    }
    return ConvertFreshPointer(expr, qual_type);
  }
  return ConvertFreshRValue(expr, qual_type);
}

void ConverterRefCount::ConvertFieldInit(const clang::FieldDecl *field,
                                         clang::Expr *init) {
  PushConversionKind push(*this, ConversionKind::Pointee);
  Converter::ConvertFieldInit(field, init);
}

void ConverterRefCount::ConvertVarInit(clang::QualType qual_type,
                                       clang::Expr *expr) {
  bool is_ref = qual_type->isReferenceType();
  PushConversionKind push(*this, ConversionKind::Unboxed, is_ref);
  StrCat(BoxValue(ConvertVarInitValue(qual_type, expr)));
}

bool ConverterRefCount::EmitGlobalValueAssign(clang::Expr *lhs,
                                              std::string_view assign_operator,
                                              std::string_view rhs) {
  auto *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(lhs->IgnoreImplicit());
  if (!decl_ref) {
    return false;
  }
  auto *var = clang::dyn_cast<clang::VarDecl>(decl_ref->getDecl());
  if (!var || !IsGlobalVar(var) || var->getType()->isReferenceType()) {
    return false;
  }
  StrCat(std::format("{}.with(|rc| *rc.borrow_mut() {} {})",
                     GetNamedDeclAsString(var), assign_operator, rhs));
  return true;
}

void ConverterRefCount::EmitSetOrAssign(clang::Expr *lhs,
                                        std::string_view rhs) {
  if (EmitGlobalValueAssign(lhs, "=", rhs)) {
    MarkFresh();
    return;
  }
  auto lhs_str = ConvertLValue(lhs);
  if (!pending_deref_.empty()) {
    auto ptr = pending_deref_.take();
    StrCat(ptr, ".write(", rhs, ')');
  } else {
    StrCat(lhs_str, token::kAssign, rhs);
  }
  MarkFresh();
}

bool ConverterRefCount::CanBraceAssignedValue(
    clang::Expr *lhs, clang::Expr *rhs, std::string_view assign_operator) {
  return (assign_operator == "=" || lhs->getType()->isArithmeticType()) &&
         !lhs->HasSideEffects(ctx_) && !rhs->HasSideEffects(ctx_);
}

void ConverterRefCount::EmitCompoundSetOrAssign(clang::Expr *lhs,
                                                clang::Expr *rhs,
                                                std::string_view value) {
  if (CanBraceAssignedValue(lhs, rhs, "=")) {
    PushBrace brace(*this, isRValue());
    EmitSetOrAssign(lhs, std::format("{{ {} }}", value));
    return;
  }
  PushBrace brace(*this);
  StrCat(keyword::kLet, "rhs_0", token::kAssign, value, token::kSemiColon);
  EmitSetOrAssign(lhs, "rhs_0");
}

void ConverterRefCount::ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                          std::string_view assign_operator) {
  auto rhs_as_string = ConvertFreshRValue(rhs, lhs->getType());

  PushBrace brace(*this, isRValue());

  // C++ evaluates the assigned value first, which may modify what the place
  // is computed from, like a pointer it is reached through.
  if (MayCauseBorrowMutError(lhs, rhs) ||
      (rhs->HasSideEffects(ctx_) && ReadsMemory(lhs))) {
    if (CanBraceAssignedValue(lhs, rhs, assign_operator)) {
      rhs_as_string = std::format("{{ {} }}", rhs_as_string);
    } else {
      StrCat(keyword::kLet, "__rhs", token::kAssign, rhs_as_string,
             token::kSemiColon);
      rhs_as_string = "__rhs";
    }
  }

  if (assign_operator == "=") {
    EmitSetOrAssign(lhs, rhs_as_string);
  } else if (EmitGlobalValueAssign(lhs, assign_operator, rhs_as_string)) {
    MarkFresh();
  } else {
    auto lhs_str = ConvertLValue(lhs);
    if (!pending_deref_.empty()) {
      bool fresh = pending_deref_.is_fresh();
      auto ptr = pending_deref_.take();
      auto op = assign_operator;
      op.remove_suffix(1); // remove '='
      PushBrace brace(*this);
      if (lhs->getType()->isPointerType()) {
        StrCat(std::format("let _ptr = {}{};", ptr, fresh ? "" : ".clone()"));
        StrCat(std::format("_ptr.write(_ptr.read() {} {})", op, rhs_as_string));
      } else {
        // The place is accessed once, after the assigned value is computed,
        // which may access it too.
        if (rhs_as_string != "__rhs" && !rhs->isEvaluatable(ctx_)) {
          StrCat(keyword::kLet, "__rhs", token::kAssign, rhs_as_string,
                 token::kSemiColon);
          rhs_as_string = "__rhs";
        }
        StrCat(std::format("{}.with_mut(|__v| *__v = *__v {} {})", ptr, op,
                           rhs_as_string));
      }
    } else {
      StrCat(lhs_str, assign_operator, rhs_as_string);
    }
    MarkFresh();
  }

  if (isRValue()) {
    StrCat(token::kSemiColon, ConvertFreshRValue(lhs));
  }
}

void ConverterRefCount::ConvertGenericBinaryOperator(
    clang::BinaryOperator *expr) {
  auto lhs = expr->getLHS();
  auto rhs = expr->getRHS();
  std::string_view opcode = expr->getOpcodeStr();

  auto lhs_vars = GetAllVars(lhs);
  auto rhs_vars = GetAllVars(rhs);

  auto sides_contains_literal = rhs_vars.empty() || lhs_vars.empty();
  auto same_var_on_both_sides = lhs_vars == rhs_vars;
  auto sides_contain_ptr_or_deref =
      RefersToPointer(lhs) || RefersToPointer(rhs);

  auto both_sides_have_va_arg = same_var_on_both_sides &&
                                ContainsVAArgExpr(lhs) &&
                                ContainsVAArgExpr(rhs);

  auto may_cause_borrow_mut_err =
      both_sides_have_va_arg ||
      (!sides_contains_literal && !same_var_on_both_sides &&
       sides_contain_ptr_or_deref);

  if (may_cause_borrow_mut_err) {
    EmitFresh(std::format(
        "({{ {} }} {} {{ {} }})",
        ConvertFreshRValue(
            lhs, GetOperandImplicitConversionTarget(ctx_, expr, lhs, rhs)),
        opcode,
        ConvertFreshRValue(
            rhs, GetOperandImplicitConversionTarget(ctx_, expr, rhs, lhs))));
    return;
  }

  PushParen outer(*this);
  Convert(lhs, GetOperandImplicitConversionTarget(ctx_, expr, lhs, rhs));
  StrCat(opcode);
  Convert(rhs, GetOperandImplicitConversionTarget(ctx_, expr, rhs, lhs));
  MarkFresh();
}

bool ConverterRefCount::ConvertCXXOperatorCallExpr(
    clang::CXXOperatorCallExpr *expr) {
  switch (expr->getOperator()) {
  case clang::OverloadedOperatorKind::OO_Equal:
    ConvertAssignment(expr->getArg(0), expr->getArg(1), "=");
    break;

  case clang::OverloadedOperatorKind::OO_Arrow:
  case clang::OverloadedOperatorKind::OO_Star:
    if (isLValue()) {
      auto ptr = ToString(expr->getArg(0));
      pending_deref_.set(std::move(ptr), isFresh());
      break;
    }

    {
      PushDeref deref(*this, expr->getType(), !isAddrOf());
      Convert(expr->getArg(0));
    }
    break;

  case clang::OverloadedOperatorKind::OO_Subscript: {
    bool is_inner_boxed =
        IsBoxedType(ctx_, expr->getType().getNonReferenceType()) &&
        IsBoxedType(ctx_, expr->getArg(0)->getType().getNonReferenceType());

    // An element of an unboxed container is accessed directly, as is one of
    // a container stored in a Value that can be borrowed.
    auto *idx = expr->getArg(1);
    bool is_unboxed = IsUnboxedPlace(expr->getArg(0));
    if ((isLValue() || isRValue()) && !is_inner_boxed &&
        (is_unboxed || IsElemOfBorrowedValue(expr))) {
      // The index is a size_t, i.e., a usize.
      auto str =
          std::format("{}[{}]", ToString(expr->getArg(0)), ConvertRValue(idx));
      // A copied element is read in a block, which ends the borrow.
      if (!is_unboxed && isRValue() && TypeIsCopyable(expr->getType())) {
        EmitFresh(std::format("{{ {} }}", str));
      } else {
        StrCat(str);
        SetValueFreshness(expr->getType());
      }
      break;
    }

    if (isLValue()) {
      PushConversionKind push_ck(*this, ConversionKind::Unboxed);
      pending_deref_.set(ConvertElemPlace(expr->getArg(0), expr->getArg(1)),
                         /*fresh=*/true);
      break;
    }

    // A field of an element is read in a closure that borrows the element,
    // and a copied element is read as a whole.
    if (expr->getType()->isRecordType() && isRValue() && !is_inner_boxed &&
        (record_ptr_ || expr == copied_expr_)) {
      PushConversionKind push(*this, ConversionKind::Unboxed);
      EmitRecordElemRead(ConvertElemPlace(expr->getArg(0), expr->getArg(1)),
                         expr->getType());
      break;
    }

    {
      bool deref = !isAddrOf();
      PushDeref push_deref(*this, expr->getType(), deref);
      PushConversionKind push(*this, ConversionKind::Unboxed);
      if (deref && !is_inner_boxed) {
        StrCat(ConvertElemPlace(expr->getArg(0), expr->getArg(1)));
      } else {
        StrCat(std::format("{}.offset({})",
                           ConvertObject(expr->getArg(0), ObjectShape::Element),
                           ConvertSubscriptIndex(expr->getArg(1))));
      }
      if (!deref) {
        MarkFresh();
      }
    }
    break;
  }
  case clang::OverloadedOperatorKind::OO_Call: {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    return Converter::ConvertCXXOperatorCallExpr(expr);
  }
  default:
    return Converter::ConvertCXXOperatorCallExpr(expr);
  }
  return false;
}

void ConverterRefCount::EmitRecordElemRead(std::string_view place,
                                           clang::QualType type) {
  if (record_ptr_) {
    StrCat(DerefPtrExpr(place, type));
  } else {
    StrCat(place, ".read()");
    MarkFresh();
  }
}

void ConverterRefCount::CapturePendingDeref(
    llvm::function_ref<void()> convert) {
  pending_deref_.assert_consumed();
  Buffer buf(*this);
  convert();
  pending_deref_.set_unchecked(std::move(buf).str(), isFresh());
}

std::string ConverterRefCount::ConvertPlace(clang::Expr *expr) {
  ConvertLValue(expr);
  assert(!pending_deref_.empty() && "not reached through a pointer");
  return pending_deref_.take();
}

std::string ConverterRefCount::ConvertElemPlace(clang::Expr *container,
                                                clang::Expr *idx) {
  // An element of a vector field is accessed in place, through the struct.
  auto *member =
      clang::dyn_cast<clang::MemberExpr>(container->IgnoreParenNoopCasts(ctx_));
  auto *field = member
                    ? clang::dyn_cast<clang::FieldDecl>(member->getMemberDecl())
                    : nullptr;
  if (field && !field->getType()->isReferenceType() &&
      !field->getParent()->isUnion() && !Mapper::Contains(ctx_, member) &&
      !IsUnboxedPlace(member) &&
      Mapper::Map(ctx_, field->getType().getUnqualifiedType())
          .starts_with("Vec<")) {
    if (IsFieldOfVectorElem(member)) {
      return std::format("index!({}, {})", ConvertPlace(member),
                         ConvertSubscriptIndex(idx));
    }
    auto ptr = ConvertRecordPtr(member);
    return std::format("vec_elem!({}, {}, {})", ptr,
                       GetNamedDeclAsString(field), ConvertSubscriptIndex(idx));
  }
  // An element of a vector reached through a pointer, e.g., a reference, is
  // accessed in place, through the pointer.
  auto *deref = clang::dyn_cast<clang::UnaryOperator>(
      container->IgnoreParenNoopCasts(ctx_));
  if ((IsReferenceType(container) ||
       (deref && deref->getOpcode() == clang::UO_Deref)) &&
      Mapper::Map(ctx_, container->getType().getUnqualifiedType())
          .starts_with("Vec<")) {
    ConvertLValue(container);
    if (!pending_deref_.empty()) {
      auto place = pending_deref_.take();
      return std::format("index!({}, {})", place, ConvertSubscriptIndex(idx));
    }
  }
  return std::format("elem!({}, {})",
                     ConvertObject(container, ObjectShape::Element),
                     ConvertSubscriptIndex(idx));
}

void ConverterRefCount::ConvertFunctionParameters(clang::FunctionDecl *decl) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  if (decl->isMain() && (decl->getNumParams() != 0U)) {
    StrCat(std::format("{}: i32, {}: Ptr<Ptr<i8>>",
                       GetNamedDeclAsString(decl->getParamDecl(0)),
                       GetNamedDeclAsString(decl->getParamDecl(1))));
  } else {
    Converter::ConvertFunctionParameters(decl);
  }
}

std::string ConverterRefCount::ConvertStringLiteralPtr(clang::Expr *expr) {
  auto *literal = expr->IgnoreParens()->IgnoreImplicit();
  auto code_unit =
      ToStringBase(ctx_.getAsArrayType(literal->getType())->getElementType());
  return std::format("Ptr::<{}>::from_string_literal({})", code_unit,
                     ToString(literal));
}

std::string ConverterRefCount::ConvertSubscriptIndex(clang::Expr *idx) {
  auto str = ConvertRValue(idx);
  if (idx->getType()->isEnumeralType()) {
    return std::format("({}) as isize", str);
  }
  return str;
}

clang::DeclRefExpr *ConverterRefCount::GetGlobalArrayRValue(clang::Expr *base) {
  auto *ref = clang::dyn_cast<clang::DeclRefExpr>(base->IgnoreImplicit());
  if (!isRValue() || !ref || !IsGlobalVar(ref) ||
      ref->getDecl()->getType()->isReferenceType() ||
      (ShouldReplaceWithMappedBody(ref->getDecl()) &&
       !GetMappedAsString(ref).empty())) {
    return nullptr;
  }
  auto *arr_ty = ctx_.getAsArrayType(ref->getType());
  if (!arr_ty) {
    return nullptr;
  }
  // Records are accessed in place, as their fields may be written to.
  auto elem_ty = ctx_.getBaseElementType(arr_ty->getElementType());
  if (!TypeIsCopyable(elem_ty) && !elem_ty->isPointerType()) {
    return nullptr;
  }
  return ref;
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

    {
      PushParen paren(*this, is_inner_boxed);
      if (IsStringLiteralExpr(base)) {
        StrCat(std::format("{}.offset({})", ConvertStringLiteralPtr(base),
                           ConvertSubscriptIndex(idx)));
      } else {
        StrCat(std::format("({} as {}).offset({})",
                           ToString(base->IgnoreImplicit()),
                           ConvertPtrType(base->IgnoreImplicit()->getType()),
                           ConvertSubscriptIndex(idx)));
      }

      if (is_inner_boxed) {
        StrCat(".as_pointer()");
      }
    }

    MarkFresh();
  } else if (auto *global = GetGlobalArrayRValue(SubscriptedArray(base))) {
    std::vector<clang::Expr *> idxs = {idx};
    for (auto *row = base->IgnoreImplicit();
         auto *sub = clang::dyn_cast<clang::ArraySubscriptExpr>(row);
         row = sub->getBase()->IgnoreImplicit()) {
      idxs.push_back(sub->getIdx());
    }
    std::string lets;
    std::string path;
    for (unsigned i = idxs.size(); i-- > 0;) {
      auto n = idxs.size() - 1 - i;
      lets += std::format("let __idx{} = ({}) as usize; ", n,
                          ConvertRValue(idxs[i]));
      path += std::format("[__idx{}]", n);
    }
    EmitFresh(std::format("({{ {}{}.with(|rc| rc.borrow(){}{}) }})", lets,
                          ConvertDeclRef(global, global->getDecl()), path,
                          TypeIsCopyable(type) ? "" : ".clone()"));
  } else {
    Convert(base->IgnoreImplicit());
    // An array reached through a pointer is indexed in place.
    if (!pending_deref_.empty()) {
      auto place = pending_deref_.take();
      pending_deref_.set(
          std::format("index!({}, {})", std::move(place), ConvertRValue(idx)),
          /*fresh=*/true);
      return;
    }
    StrCat(std::format("[({}) as usize]", ConvertRValue(idx)));
    SetValueFreshness(type);
  }
}

void ConverterRefCount::ConvertPointerElem(clang::Expr *base,
                                           clang::Expr *idx) {
  // An element of an array field is accessed in place, through the struct.
  if (IsArrayFieldPtr(ctx_, base)) {
    auto *member = clang::cast<clang::MemberExpr>(base->IgnoreParenImpCasts());
    if (IsFieldOfVectorElem(member)) {
      EmitFresh(std::format("index!({}, {})", ConvertPlace(member),
                            ConvertRValue(idx)));
      return;
    }
    auto ptr = ConvertRecordPtr(member);
    StrCat(std::format("array_elem!({}, {}, {})", ptr,
                       GetNamedDeclAsString(member->getMemberDecl()),
                       ConvertRValue(idx)));
  } else {
    StrCat(std::format("elem!({}, {})", ToString(base), ConvertRValue(idx)));
  }
  MarkFresh();
}

void ConverterRefCount::ConvertPointerSubscript(
    clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  auto *idx = expr->getIdx();

  if (isLValue()) {
    CapturePendingDeref([&] { ConvertPointerElem(base, idx); });
    return;
  }

  // A field of an element is read in a closure that borrows the element,
  // and a copied element is read as a whole.
  if (expr->getType()->isRecordType() && isRValue() &&
      (record_ptr_ || expr == copied_expr_)) {
    std::string place;
    {
      Buffer buf(*this);
      ConvertPointerElem(base, idx);
      place = std::move(buf).str();
    }
    EmitRecordElemRead(place, expr->getType());
    return;
  }

  bool deref = !isAddrOf();
  PushDeref push_deref(*this, expr->getType(), deref);
  if (deref) {
    ConvertPointerElem(base, idx);
  } else {
    ConvertPointerOffset(base, idx);
    if (expr->getType()->isArrayType()) {
      StrCat(".as_pointer()");
    }
  }
}

std::string ConverterRefCount::ForceGlobalInit(const clang::VarDecl *decl) {
  return std::format("let _ = {}.with(|_| ());", GetNamedDeclAsString(decl));
}

void ConverterRefCount::ConvertFunctionMain(
    const clang::FunctionDecl *decl,
    const std::string_view main_function_name) {
  if (decl->getNumParams() != 0U) {
    StrCat(std::format(R"(
pub fn main() {{
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {{
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            }})
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit({}(::std::env::args().len() as i32,
                                argv.as_pointer()));
}})",
                       main_function_name));
  } else {
    StrCat(std::format("pub fn main() {{ __cpp2rust_init_globals(); "
                       "std::process::exit({}()); }}",
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
    auto ptr = ToString(expr);
    pending_deref_.set(std::move(ptr), isFresh());
    return;
  }

  if (record_ptr_ && pointee_type->isRecordType() && isRValue()) {
    StrCat(DerefPtrExpr(ToString(expr), pointee_type));
    return;
  }

  std::string str;
  {
    Buffer buf(*this);
    {
      PushDeref deref(*this, pointee_type, !isAddrOf());
      Convert(expr);
    }
    str = std::move(buf).str();
  }

  if (isAddrOf() && pointee_type->isArrayType()) {
    str = std::format("({}).as_pointer()", std::move(str));
  }
  StrCat(std::move(str));
}

void ConverterRefCount::ConvertArrow(clang::Expr *expr) {
  auto *op = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
  bool is_overloaded_arrow =
      op && op->getOperator() == clang::OverloadedOperatorKind::OO_Arrow;

  if (!is_overloaded_arrow || IsUserOperatorCall(op) ||
      RuleRegistry::ReturnsPointer(ctx_, expr)) {
    auto pointee_type = expr->getType()->getPointeeType();
    // Virtual methods, which take &mut self, are called through PtrDyn.
    if (auto *record = pointee_type->getAsRecordDecl();
        isLValue() && record && IsAbstractUserClass(record)) {
      StrCat(std::format("(*{}.upgrade().deref_mut())", ToString(expr)));
      SetValueFreshness(pointee_type);
      return;
    }
    if (isLValue() || isAddrOf()) {
      ConvertDeref(expr);
      return;
    }
    EmitPointee(ToString(expr), pointee_type, /*fresh=*/false);
    return;
  }

  Convert(expr);
}

std::string ConverterRefCount::AccessLValueObject(clang::MemberExpr *member) {
  auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member->getMemberDecl());
  auto *object = member->getBase();

  bool is_mut = method && !method->isConst();
  auto str = is_mut ? ConvertLValue(object) : ConvertRValue(object);
  if (!member->isArrow()) {
    return str;
  }
  return DerefPtrExpr(str, object->getType()->getPointeeType());
}

void ConverterRefCount::ConvertConstructedValue(clang::QualType type,
                                                clang::CXXConstructExpr *ctor) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  ConvertVarInit(type, ctor);
}

bool ConverterRefCount::DerefReadsValue(clang::QualType pointee_type) {
  return pointee_type.isPODType(ctx_) && !pointee_type->isRecordType();
}

const char *
ConverterRefCount::GetPointerDerefSuffix(clang::QualType pointee_type) {
  return DerefReadsValue(pointee_type) ? ".read()" : ".upgrade().deref()";
}

void ConverterRefCount::EmitPointee(std::string ptr,
                                    clang::QualType pointee_type, bool fresh) {
  if (isLValue()) {
    pending_deref_.set(std::move(ptr), fresh);
    return;
  }
  StrCat(DerefPtrExpr(ptr, pointee_type));
  SetDerefFreshness(pointee_type);
}

void ConverterRefCount::SetDerefFreshness(clang::QualType pointee_type) {
  if (DerefReadsValue(pointee_type)) {
    MarkFresh();
  } else {
    SetValueFreshness(pointee_type);
  }
}

const char *
ConverterRefCount::GetPointerDerefPrefix(clang::QualType pointee_type) {
  return DerefReadsValue(pointee_type) ? "" : token::kStar;
}

std::string ConverterRefCount::DerefPtrExpr(std::string_view ptr_expr,
                                            clang::QualType pointee_type) {
  if (record_ptr_ && pointee_type->isRecordType()) {
    *std::exchange(record_ptr_, nullptr) = ptr_expr;
    return "__s";
  }
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

std::string ConverterRefCount::ConvertBorrowMut(clang::Expr *arg) {
  auto lvalue = ConvertLValue(arg);
  // An object reached through a pointer is borrowed through it, unless it is
  // the receiver of a method call, which is borrowed by the call.
  if (!pending_deref_.empty() && !in_receiver_) {
    auto name = std::format("__b{}", mut_borrows_.size());
    mut_borrows_.emplace_back(pending_deref_.take(), name);
    MarkNotFresh();
    return std::format("(*{})", name);
  }
  return lvalue;
}

std::string ConverterRefCount::ConvertTake(clang::Expr *arg) {
  auto lvalue = ConvertLValue(arg);
  MarkFresh();
  // An object reached through a pointer is taken through it.
  if (!pending_deref_.empty()) {
    return std::format("{}.with_mut(std::mem::take)", pending_deref_.take());
  }
  return std::format("std::mem::take(&mut {})", std::move(lvalue));
}

std::string ConverterRefCount::ConvertMappedMethodCall(
    clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
    llvm::ArrayRef<clang::TemplateArgument> bindings, clang::Expr **args,
    unsigned num_args, TempMaterializationCtx *ctx) {
  auto receiver_ph = mc.getReceiverPlaceholder();
  if (!receiver_ph || receiver_ph->access == TranslationRule::Access::kBorrow ||
      receiver_ph->access == TranslationRule::Access::kMove) {
    return Converter::ConvertMappedMethodCall(expr, mc, bindings, args,
                                              num_args, ctx);
  }

  auto arg_idx = receiver_ph->n;
  auto *arg = BuildUnifiedArgs(expr, args, num_args)[arg_idx];
  if (auto *call = clang::dyn_cast<clang::CallExpr>(arg->IgnoreCasts());
      call && IsTransparentStdCall(call)) {
    arg = call->getArg(0);
  }

  auto param_type = Mapper::GetParamType(*this, expr, arg_idx);

  if (arg->getType()->isPointerType()) {
    return std::format(
        "{}.with_mut(|__v: {}| __v{})", ConvertPointer(arg), param_type,
        ConvertIRFragment(mc.body, bindings, expr, args, num_args, ctx));
  }

  // A receiver reached through a pointer is modified through it, only for
  // the call, after the other arguments are computed.
  std::string receiver;
  {
    ScopedValue in_receiver(in_receiver_, true);
    receiver =
        ConvertIRFragment(mc.receiver, bindings, expr, args, num_args, ctx);
  }
  if (pending_deref_.empty()) {
    return receiver +
           ConvertIRFragment(mc.body, bindings, expr, args, num_args, ctx);
  }

  auto ptr = pending_deref_.take();
  auto body = ConvertIRFragment(mc.body, bindings, expr, args, num_args, ctx);
  MarkFresh();

  return std::format("{}.with_mut(|__v: {}| __v{})", ptr, param_type, body);
}

std::string ConverterRefCount::ConvertPointeeType(clang::QualType ptr_type) {
  assert(!ptr_type.isNull() && ptr_type->isPointerType());
  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto pointee = ptr_type->getPointeeType();
  if (pointee->isArrayType()) {
    PushConversionKind array(*this, ConversionKind::Pointee);
    return std::string(Trim(ToString(pointee)));
  }
  if (!pointee->isRecordType()) {
    return std::string(Trim(ToString(pointee)));
  }

  // Pointee of a pointer to incomplete type is an incomplete type that does
  // not have a translation rule. Hence ToString(ptr_type->getPointeeType()) is
  // not enough
  auto str = ToString(ptr_type);
  Unwrap(str, "PtrDyn<", ">");
  Unwrap(str, "Ptr<", ">");
  return std::string(Trim(str));
}

bool ConverterRefCount::ShouldConvertMethod(const clang::CXXMethodDecl *decl) {
  if (clang::isa<clang::CXXDestructorDecl>(decl)) {
    return IsMethodOnPtr(decl);
  }
  return Converter::ShouldConvertMethod(decl);
}

bool ConverterRefCount::ThisIsRustPtr() const {
  auto *method = clang::dyn_cast_or_null<clang::CXXMethodDecl>(curr_function_);
  return method && (IsMethodOnPtr(method) ||
                    clang::isa<clang::CXXConstructorDecl>(method));
}

void ConverterRefCount::SetUFCSReceiver(clang::Expr *base, bool is_arrow,
                                        const clang::CXXMethodDecl *method) {
  if (!IsMethodOnPtr(method)) {
    Converter::SetUFCSReceiver(base, is_arrow, method);
    return;
  }
  bool base_is_pointer = is_arrow && !clang::isa<clang::CXXOperatorCallExpr>(
                                         base->IgnoreParenImpCasts());
  if (clang::isa<clang::CXXThisExpr>(base->IgnoreParenImpCasts()) &&
      !AsLambdaOperatorCall(curr_function_)) {
    if (InConstructor()) {
      ctor_uses_this_ = true;
      ufcs_receiver_ = "&this";
    } else if (ThisIsRustPtr()) {
      ufcs_receiver_ = keyword::kSelfValue;
    } else {
      ufcs_receiver_ = token::kRef + ConvertPointer(base);
    }
    return;
  }
  if (IsTemporaryObject(base) && base->getType()->isRecordType() &&
      !IsReferenceType(base->IgnoreImplicit())) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    ufcs_receiver_ =
        token::kRef + BoxValue(ConvertRValue(base)) + ".as_pointer()";
    return;
  }
  ufcs_receiver_ = token::kRef + (base_is_pointer ? ConvertRValue(base)
                                                  : ConvertPointer(base));
}

std::string
ConverterRefCount::GetUFCSName(const clang::CXXMethodDecl *method) const {
  return IsMethodOnPtr(method) ? TraitName(method->getParent())
                               : GetRecordName(method->getParent());
}

std::string
ConverterRefCount::TraitName(const clang::CXXRecordDecl *decl) const {
  return GetRecordName(decl) + "Impl";
}

ConverterRefCount::MethodsOnPtr &
ConverterRefCount::MethodsOnPtrFor(const clang::CXXRecordDecl *decl) {
  auto name = GetRecordName(decl);
  auto [it, inserted] = methods_on_ptr_.try_emplace(name);
  if (inserted) {
    it->second.trait.header = std::format("pub trait {}", TraitName(decl));
    it->second.impl.header =
        std::format("impl {} for Ptr<{}>", TraitName(decl), name);
  }
  return it->second;
}

bool ConverterRefCount::ConvertOutOfLineMethod(clang::CXXMethodDecl *decl) {
  if (!IsMethodOnPtr(decl)) {
    return Converter::ConvertOutOfLineMethod(decl);
  }
  Buffer buf(*this);
  {
    ScopedValue push(method_target_, MethodTarget::PtrImpl);
    ConvertCXXMethodDecl(decl);
  }
  MethodsOnPtrFor(decl->getParent()).impl.body += std::move(buf).str();
  return false;
}

void ConverterRefCount::ConvertMethodOnPtrTraitDecl(
    clang::CXXMethodDecl *method) {
  Buffer buf(*this);
  {
    ScopedValue push_fn(curr_function_, method);
    ScopedValue push(method_target_, method->getDefinition()
                                         ? MethodTarget::TraitDecl
                                         : MethodTarget::TraitDefault);
    ConvertCXXMethodDecl(method);
  }
  MethodsOnPtrFor(method->getParent()).trait.body += std::move(buf).str();
}

void ConverterRefCount::ConvertMethodOnPtr(clang::CXXMethodDecl *method) {
  if (!method->isThisDeclarationADefinition()) {
    return;
  }
  Buffer buf(*this);
  {
    ScopedValue push(method_target_, MethodTarget::PtrImpl);
    VisitCXXMethodDecl(method);
  }
  MethodsOnPtrFor(method->getParent()).impl.body += std::move(buf).str();
}

void ConverterRefCount::ConvertLateInstantiatedMethods(
    clang::CXXRecordDecl *decl) {
  Converter::ConvertCXXMethodDecls(
      decl, std::format("{} {}", keyword::kImpl, GetRecordName(decl)),
      [](auto *method) {
        return IsEmittableMethod(method) && method->hasBody() &&
               !IsMethodOnPtr(method) &&
               !decl_ids_.contains(GetMethodID(method));
      });
  ForEachMethod(decl, [&](clang::CXXMethodDecl *method) {
    if (IsEmittableMethod(method) && method->hasBody() &&
        IsMethodOnPtr(method) && !decl_ids_.contains(GetMethodID(method))) {
      ConvertMethodOnPtr(method);
    }
  });
}

void ConverterRefCount::ConvertCXXRecordMethods(clang::CXXRecordDecl *decl) {
  auto struct_name = GetRecordName(decl);

  ConvertCXXMethodDecls(decl, std::format("{} {}", keyword::kImpl, struct_name),
                        [](auto *method) {
                          return IsEmittableMethod(method) &&
                                 !IsMethodOnPtr(method);
                        });

  ForEachMethod(decl, [&](clang::CXXMethodDecl *method) {
    if (IsMethodOnPtr(method) && !method->getReturnType()->isUndeducedType()) {
      ConvertMethodOnPtrTraitDecl(method);
      ConvertMethodOnPtr(method);
    }
  });

  if (!GetUserDefinedDestructor(decl) && HasFieldsNeedingDestruction(decl)) {
    MethodsOnPtrFor(decl).trait.body +=
        std::format("fn {}(&self);\n", kDestructorName);
    MethodsOnPtrFor(decl).impl.body += std::format(
        "fn {}(&self) {{ {} }}\n", kDestructorName, DestroyMembers(decl));
  }
}

std::string
ConverterRefCount::DestroyMembers(const clang::CXXRecordDecl *decl) {
  std::string out;
  for (auto *field : FieldsToDestroy(decl)) {
    auto name = GetNamedDeclAsString(field);
    if (field->getType()->isArrayType()) {
      auto *elem =
          field->getType()->getBaseElementTypeUnsafe()->getAsCXXRecordDecl();
      assert(elem);
      out += std::format(
          "{{ let __p = array_field_ptr!(self, {0}); for __i in "
          "0..__p.len() {{ {2}::{1}(&__p.offset(__i as isize)); }} }}\n",
          name, kDestructorName, TraitName(elem));
    } else {
      out +=
          std::format("field_ptr!(self, {0}).{1}();\n", name, kDestructorName);
    }
  }
  return out;
}

void ConverterRefCount::ConvertCXXConstructorBody(
    clang::CXXConstructorDecl *decl) {
  EmitFunctionPreamble(decl);
  auto record_name = GetRecordName(decl->getParent());
  auto saved_uses_this = std::exchange(ctor_uses_this_, false);
  std::string init;
  {
    Buffer buf(*this);
    EmitConstructorInit(decl);
    init = std::move(buf).str();
  }
  std::string body;
  {
    Buffer buf(*this);
    ConvertBodyStmts(decl->getBody());
    body = std::move(buf).str();
  }
  bool uses_this = std::exchange(ctor_uses_this_, saved_uses_this);

  if (!uses_this) {
    if (Trim(body).empty()) {
      StrCat(init);
    } else {
      StrCat(keyword::kLet, "__this", token::kColon, record_name,
             token::kAssign, init, token::kSemiColon, body, "__this");
    }
    return;
  }
  StrCat(keyword::kLet, "__this", token::kColon,
         std::format("Value<{}>", record_name), token::kAssign,
         std::format("Rc::new(RefCell::new({}))", init), token::kSemiColon);
  StrCat(keyword::kLet, "this", token::kColon,
         std::format("Ptr<{}>", record_name), token::kAssign,
         "__this.as_pointer()", token::kSemiColon);
  StrCat(body);
  StrCat("Rc::try_unwrap(__this).ok().unwrap().into_inner()");
}

bool ConverterRefCount::VisitCXXThisExpr(
    [[maybe_unused]] clang::CXXThisExpr *expr) {
  if (auto field = AsLambdaCaptureThis(curr_function_)) {
    auto name = GetNamedDeclAsString(field);
    EmitFresh(field->getType()->isPointerType()
                  ? std::format("(*{}.borrow()).clone()", name)
                  : std::format("{}.as_pointer()", name));
    return false;
  }
  if (InConstructor()) {
    ctor_uses_this_ = true;
    StrCat("this");
  } else {
    StrCat("(*", keyword::kSelfValue, ')');
  }
  MarkNotFresh();
  return false;
}
} // namespace cpp2rust
