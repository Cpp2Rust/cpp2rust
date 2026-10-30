// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter.h"

#include <clang/AST/APValue.h>
#include <clang/AST/IgnoreExpr.h>
#include <clang/Basic/LangOptions.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Basic/Version.h>
#include <clang/Sema/Template.h>
#include <llvm/ADT/DenseMap.h>
#include <llvm/ADT/StringExtras.h>
#include <llvm/Support/ConvertUTF.h>
#include <llvm/Support/ErrorHandling.h>

#include <algorithm>
#include <format>
#include <ranges>
#include <utility>

#include "compiler.h"
#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/mapper.h"
#include "converter/printer.h"
#include "converter/rules/registry.h"

namespace cpp2rust {
std::unordered_map<std::string, std::string> Converter::inner_structs_;
std::unordered_set<std::string> Converter::decl_ids_;
std::unordered_set<std::string> Converter::globals_;
std::vector<std::string> Converter::global_inits_;
Converter::RecordIndex Converter::record_decls_;
std::map<std::string, Converter::DeferredBlock> Converter::virtual_methods_;

void Converter::EmitFilePreamble() {
  StrCat(R"(
extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Write, Seek};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
)");
}

void Converter::EmitDeferredBlock(const DeferredBlock &block,
                                  std::string &out) {
  out += block.header;
  out += " {\n";
  out += block.body;
  out += "}\n";
}

void Converter::EmitVirtualMethods(std::string &out) {
  for (const auto &[name, impl] : virtual_methods_) {
    EmitDeferredBlock(impl, out);
  }
}

std::string Converter::ForceGlobalInit(const clang::VarDecl *decl) {
  return std::format("std::cell::LazyCell::force(&*&raw const {});",
                     GetNamedDeclAsString(decl));
}

void Converter::EmitGlobalInits(Model model, std::string &out) {
  out += model == Model::kUnsafe ? "pub unsafe fn __cpp2rust_init_globals() {\n"
                                 : "pub fn __cpp2rust_init_globals() {\n";
  for (const auto &line : global_inits_) {
    out += line;
    out += '\n';
  }
  out += "}\n";
}

void Converter::EmitOpaqueRecords(Model model, std::string &out) {
  record_decls_.ForEachUndefined([&](const std::string &name) {
    if (model == Model::kRefCount) {
      out += "#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]";
      out += "#[byte_size(1)]";
    } else {
      out += "#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]";
    }
    out += "pub struct ";
    out += name;
    out += ";\n";
  });
}

bool Converter::VisitRecoveryExpr(clang::RecoveryExpr *expr) {
  llvm::errs() << "RecoveryExpr: ";
  expr->dump();
  exit(1);
  return false;
}

bool Converter::Convert(clang::QualType qual_type) {
  // Catch va_list before desugaring
  if (IsVaListType(qual_type)) {
    StrCat("*const VaArg");
    return false;
  }

  if (auto decl = qual_type->getAsRecordDecl();
      decl && IsUserDefinedDecl(decl)) {
    record_decls_.MarkReferenced(GetRecordName(decl));
  }

  auto mapped = Mapper::Map(*this, qual_type);
  if (!mapped.empty() && mapped != token::kIgnoreRule) {
    StrCat(mapped);
    return false;
  }

  qual_type = qual_type.getUnqualifiedType().getDesugaredType(ctx_);
  return TraverseType(qual_type);
}

std::string Converter::GetUnboxedTypeAsString(clang::QualType qual_type) {
  return ToString(qual_type);
}

std::string Converter::GetUnboxedDefaultAsString(clang::QualType qual_type) {
  return GetDefaultAsString(qual_type);
}

std::string
Converter::GetTemplateArgumentAsString(const clang::TemplateArgument &arg) {
  switch (arg.getKind()) {
  case clang::TemplateArgument::Type:
    return GetUnboxedTypeAsString(arg.getAsType());
  case clang::TemplateArgument::Integral:
  case clang::TemplateArgument::NullPtr:
  case clang::TemplateArgument::Declaration:
  case clang::TemplateArgument::StructuralValue: {
    auto expr = GetSema().BuildExpressionFromNonTypeTemplateArgument(
        arg, clang::SourceLocation());
    assert(expr.isUsable() && "cannot build non-type template argument");
    Buffer buf(*this);
    Convert(expr.get());
    return std::move(buf).str();
  }
  default:
    assert(0 && "template parameter is not bound to a type or a value");
    return {};
  }
}

std::string Converter::ConvertPointeeType(clang::QualType ptr_type) {
  assert(!ptr_type.isNull() && ptr_type->isPointerType());
  auto pointee = ptr_type->getPointeeType();
  if (!pointee->isRecordType()) {
    return std::string(Trim(ToString(pointee)));
  }

  auto str = ToString(ptr_type);
  Unwrap(str, "*mut ", "");
  Unwrap(str, "*const ", "");
  return std::string(Trim(str));
}

bool Converter::VisitBuiltinType(clang::BuiltinType *type) {
  auto mapped = Mapper::Map(*this, clang::QualType(type, 0));
  if (mapped.empty()) {
    llvm::report_fatal_error(llvm::Twine("no type rule for builtin type: ") +
                             type->getName(ctx_.getPrintingPolicy()));
  }
  StrCat(mapped);
  return false;
}

bool Converter::VisitRecordType(clang::RecordType *type) {
  auto *decl = type->getDecl();
  if (auto lambda = AsLambdaClass(clang::QualType(type, 0))) {
    if (lambda->isGenericLambda()) {
      StrCat("FnPtr<Generic>");
      return false;
    }
    auto proto = lambda->getLambdaCallOperator()
                     ->getType()
                     ->castAs<clang::FunctionProtoType>();
    StrCat(std::format("FnPtr<{}>", ConvertFunctionPointerType(proto)));
    return false;
  }

  StrCat(GetRecordName(decl));
  return false;
}

bool Converter::VisitEnumType(clang::EnumType *type) {
  StrCat(GetRecordName(type->getDecl()));
  return false;
}

std::string
Converter::ConvertPointer(clang::Expr *expr,
                          std::optional<clang::QualType> implicit_convert_to,
                          int line) {
  log() << "ConvertPointer called from line " << line << '\n';
  PushExprKind push(*this, ExprKind::AddrOf);
  return ToString(expr, implicit_convert_to);
}

std::string Converter::ConvertFreshPointer(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  auto str = ConvertPointer(expr, implicit_convert_to);
  MarkFresh();
  return str;
}

std::string Converter::MakeFresh(std::string str) {
  if (isFresh()) {
    return str;
  }
  MarkFresh();
  return std::format("({}).clone()", std::move(str));
}

std::string Converter::ConvertFreshObject(clang::Expr *expr) {
  return ConvertFreshPointer(expr);
}

std::string Converter::ConvertPointerReceiver(clang::Expr *arg,
                                              std::string_view param_type) {
  return std::format("({} as {})", ConvertFreshObject(arg), param_type);
}

std::string Converter::ConvertTemporaryPointer(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  return std::format("&mut {}", ConvertFreshRValue(expr, implicit_convert_to));
}

std::string Converter::ConvertLValue(clang::Expr *expr) {
  PushExprKind push(*this, ExprKind::LValue);
  return ToString(expr);
}

std::string
Converter::ConvertRValue(clang::Expr *expr,
                         std::optional<clang::QualType> implicit_convert_to,
                         int line) {
  log() << "ConvertRValue called from line " << line << '\n';
  PushExprKind push(*this, ExprKind::RValue);
  return ToString(expr, implicit_convert_to);
}

std::string Converter::ConvertFreshRValue(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  auto str = ConvertRValue(expr, implicit_convert_to);
  if (expr->getType()->isVoidType() || expr->getType()->isPointerType()) {
    MarkFresh();
    return str;
  }
  return MakeFresh(std::move(str));
}

std::pair<std::string, std::string>
Converter::MaterializeTemp(const std::string &binding_name,
                           clang::QualType param_type, clang::Expr *expr) {
  auto pointee = param_type.getNonReferenceType();
  std::string value;
  {
    ScopedPush init_type(curr_init_type_, pointee);
    value = ConvertRValue(expr, pointee);
  }
  auto type_str = ToStringBase(pointee);
  const auto *decl = in_const_initializer_ ? keyword::kStatic : keyword::kLet;

  auto binding =
      std::format("{} mut {} : {} = {};", decl, binding_name, type_str, value);
  auto ref = in_const_initializer_
                 ? std::format("& mut *& raw mut {}", binding_name)
                 : std::format("& mut {}", binding_name);
  return {binding, ref};
}

std::string Converter::EmitMaterializedTempBinding(clang::QualType param_type,
                                                   clang::Expr *expr) {
  assert(materialized_temp_bindings_ && "materialized temp emitted outside a "
                                        "HoistMaterializedTempBindings scope");
  auto [binding, ref] = MaterializeTemp(NewTempName(), param_type, expr);
  *materialized_temp_bindings_ += std::move(binding);
  return ref;
}

bool Converter::VisitConstantArrayType(clang::ConstantArrayType *type) {
  StrCat('[');
  Convert(type->getElementType());
  auto size = GetNumAsString(type->getSize());
  StrCat(std::format("; {}]", size.c_str()));
  return false;
}

bool Converter::VisitIncompleteArrayType(clang::IncompleteArrayType *type) {
  StrCat('[');
  Convert(type->getElementType());
  StrCat("; 0]");
  return false;
}

bool Converter::VisitReferenceType(clang::ReferenceType *type) {
  auto pointee_type = type->getPointeeType();
  if (pointee_type->isFunctionType()) {
    return Convert(ctx_.getPointerType(pointee_type));
  }
  StrCat(pointee_type.isConstQualified() ? "*const" : "*mut");
  return Convert(pointee_type);
}

std::string
Converter::ConvertFunctionPointerType(const clang::FunctionProtoType *proto) {
  std::string result = "fn(";
  for (auto p_ty : proto->param_types()) {
    result += ToString(p_ty);
    result += ',';
  }
  if (proto->isVariadic()) {
    result += "Box<[VaArg]>,";
  }
  result += ')';
  if (!proto->getReturnType()->isVoidType()) {
    result += std::format(" -> {}", ToString(proto->getReturnType()));
  }
  return result;
}

bool Converter::VisitPointerType(clang::PointerType *type) {
  if (auto proto = type->getPointeeType()->getAs<clang::FunctionProtoType>()) {
    StrCat(std::format("Option<{} {}>", keyword_unsafe_,
                       ConvertFunctionPointerType(proto)));
    return false;
  }

  auto pointee_type = type->getPointeeType();
  StrCat(pointee_type.isConstQualified() ? "*const" : "*mut");
  if (pointee_type->isRecordType() &&
      IsAbstractUserClass(pointee_type->getAsRecordDecl())) {
    StrCat(keyword::kDyn);
  }
  return Convert(pointee_type);
}

bool Converter::VisitDecayedType(clang::DecayedType *type) {
  return Convert(type->getDecayedType());
}

bool Converter::VisitTypedefType(clang::TypedefType *type) {
  return Convert(type->desugar());
}

bool Converter::VisitUsingType(clang::UsingType *type) {
  return Convert(type->desugar());
}

bool Converter::Convert(clang::Decl *decl) { return TraverseDecl(decl); }

bool Converter::VisitTranslationUnitDecl(clang::TranslationUnitDecl *decl) {
  for (auto *child : decl->decls()) {
    if (IsUserDefinedDecl(child) &&
        (IsInMainFile(child) || !decl_ids_.contains(GetID(child)))) {
      Convert(child);
      if (!hoisted_records_.empty()) {
        StrCat(hoisted_records_);
        hoisted_records_.clear();
      }
    }
  }
  return false;
}

bool Converter::VisitFunctionDecl(clang::FunctionDecl *decl) {
  if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(decl)) {
    return VisitCXXMethodDecl(method);
  }
  if (!IsConvertibleFunctionDecl(decl)) {
    return false;
  }
  if (!IsInMainFile(decl) && !decl_ids_.insert(GetID(decl)).second) {
    return false;
  }
  decl->dump(log());
  ScopedValue push_fn(curr_function_, decl);
  std::string function_name;
  if (decl->isMain()) {
    function_name = "main_0";
    ConvertFunctionMain(decl, function_name);
  } else {
    function_name = GetNamedDeclAsString(decl->getCanonicalDecl());
  }
  // main_0 should be static
  if (!decl->isMain()) {
    StrCat(AccessSpecifierAsString(decl->getAccess()));
  }
  StrCat(keyword_unsafe_, keyword::kFn, std::move(function_name));
  {
    PushParen paren(*this);
    ConvertFunctionParameters(decl);
  }
  ConvertFunctionReturnType(decl);
  {
    PushBrace brace(*this);
    EmitFunctionPreamble(decl);
    ConvertFunctionBody(decl);
  }
  return false;
}

void Converter::EmitHoistedDecls(clang::CompoundStmt *body) {
  for (auto *child : body->body()) {
    if (auto *decl_stmt = clang::dyn_cast<clang::DeclStmt>(child)) {
      for (auto *decl : decl_stmt->decls()) {
        if (auto *var = clang::dyn_cast<clang::VarDecl>(decl);
            var && var->isLocalVarDecl() && !IsGlobalVar(var)) {
          EmitHoistedDecl(var);
        }
      }
    }
  }
}

void Converter::EmitHoistedDecl(clang::VarDecl *decl) {
  hoisted_decls_.insert(decl);
  if (ConvertVarDeclSkipInit(decl)) {
    StrCat(token::kAssign, ConvertVarDefaultInit(decl), token::kSemiColon);
  }
}

void Converter::ConvertGotoBlock(clang::CompoundStmt *body) {
  HoistMaterializedTempBindings hoist_temps(*this);
  ScopedValue push(hoisted_decls_, {});
  EmitHoistedDecls(body);

  StrCat("goto_block!");
  {
    PushParen paren(*this);
    PushBrace outer(*this);
    StrCat("'__entry: ");
    std::optional<PushBrace> arm;
    arm.emplace(*this);
    for (auto *child : body->body()) {
      if (auto *label = clang::dyn_cast<clang::LabelStmt>(child)) {
        arm.reset();
        StrCat(std::format("'{}: ", label->getDecl()->getName().str()));
        arm.emplace(*this);
        Convert(label->getSubStmt());
      } else {
        Convert(child);
      }
    }
  }
  StrCat(token::kSemiColon);
}

void Converter::ConvertFunctionBody(clang::FunctionDecl *decl) {
  ConvertBodyStmts(decl->getBody());
  if (auto *dtor = clang::dyn_cast<clang::CXXDestructorDecl>(decl)) {
    StrCat(DestroyMembers(dtor->getParent()));
  }
  if (decl->getReturnType()->isVoidType()) {
    return;
  }
  auto compound = clang::dyn_cast<clang::CompoundStmt>(decl->getBody());
  if (decl->isMain() && compound &&
      (compound->body_empty() ||
       !clang::isa<clang::ReturnStmt>(compound->body_back()))) {
    StrCat('0');
    return;
  }
  if (!compound || compound->body_empty()) {
    return;
  }
  if (CompoundHasTopLevelLabel(compound) ||
      !clang::isa<clang::ReturnStmt>(compound->body_back())) {
    StrCat(R"(panic!("ub: non-void function does not return a value"))");
  }
}

bool Converter::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl *decl) {
  for (auto *function_decl : decl->specializations()) {
    if (function_decl->getTemplateSpecializationKind() ==
        clang::TSK_ExplicitSpecialization) {
      continue;
    }
    VisitFunctionDecl(function_decl);
  }
  return false;
}

bool Converter::VisitVarTemplateDecl(clang::VarTemplateDecl *decl) {
  for (auto *var_decl : decl->specializations()) {
    VisitVarDecl(var_decl);
  }
  return false;
}

bool Converter::NeedsMut(const clang::VarDecl *decl, clang::QualType type,
                         llvm::StringRef name) const {
  auto *method_or_null =
      curr_function_ ? clang::dyn_cast<clang::CXXMethodDecl>(curr_function_)
                     : nullptr;
  return ((hoisted_decls_.contains(decl) || NeedsScopedDestructor(decl) ||
           (!type.isConstQualified() && !type->isReferenceType())) &&
          ((method_or_null == nullptr) || !method_or_null->isVirtual()) &&
          !IsGlobalVar(decl));
}

bool Converter::ConvertVarDeclSkipInit(clang::VarDecl *decl) {
  auto qual_type = decl->getType();
  if (IsVaListType(qual_type) && decl->isLocalVarDecl()) {
    StrCat(keyword::kLet, keyword_mut_, GetNamedDeclAsString(decl),
           token::kColon);
    Convert(qual_type);
    return true;
  }

  auto name = GetNamedDeclAsString(decl);
  if (decl->isFileVarDecl()) {
    if ((decl->isThisDeclarationADefinition() ==
             clang::VarDecl::DeclarationOnly &&
         !decl->hasInit()) ||
        !globals_.insert(name).second) {
      return false;
    }
    StrCat(AccessSpecifierAsString(decl->getAccess()), keyword::kStatic,
           keyword_mut_);
    ENSURE(decl_ids_.insert(GetID(decl)).second);
    global_inits_.push_back(ForceGlobalInit(decl));
  } else if (decl->isStaticLocal()) {
    if (!globals_.insert(name).second) {
      return false;
    }
    StrCat(keyword::kStatic, keyword_mut_);
  } else if (decl->isLocalVarDecl()) {
    StrCat(keyword::kLet);
  }

  if (NeedsMut(decl, qual_type, name)) {
    // If this decl requires 'mut', print it irregardless of the model
    StrCat(keyword::kMut);
  }
  StrCat(name, token::kColon);

  bool is_parm_with_default_value = false;
  if (auto parm = clang::dyn_cast<clang::ParmVarDecl>(decl)) {
    is_parm_with_default_value = HasUsableDefaultArg(parm);
  }

  if (is_parm_with_default_value) {
    StrCat("Option<");
  }
  {
    PushLazyType lazy(*this, IsGlobalVar(decl) && LazyStaticInit());
    Convert(qual_type);
  }
  if (is_parm_with_default_value) {
    StrCat('>');
  }
  return true;
}

void Converter::ConvertVarDeclInitializer(clang::VarDecl *decl) {
  if (decl->hasInit()) {
    ConvertVarInit(decl->getType(), decl->getInit());
  } else if (!clang::isa<clang::ParmVarDecl>(decl)) {
    StrCat(ConvertVarDefaultInit(decl));
  }
}

void Converter::EmitHoistedInArmAssignment(clang::VarDecl *decl) {
  if (!decl->hasInit()) {
    return;
  }
  StrCat(GetNamedDeclAsString(decl), token::kAssign);
  ConvertVarInit(decl->getType(), decl->getInit());
  StrCat(token::kSemiColon);
}

void Converter::ConvertVarDecl(clang::VarDecl *decl) {
  if (hoisted_decls_.contains(decl)) {
    EmitHoistedInArmAssignment(decl);
    return;
  }

  HoistMaterializedTempBindings hoist_temps(*this);
  if (decl->isLocalVarDecl() && !IsGlobalVar(decl) && decl->hasInit() &&
      ReferencesVar(decl->getInit(), decl)) {
    if (decl->getType()->isReferenceType()) {
      llvm::report_fatal_error(
          "self-referential initializer of a reference is not supported");
    }
    if (DefaultInitHasSideEffects(ctx_, decl->getType())) {
      llvm::report_fatal_error("self-referential initializer of a variable "
                               "whose default initialization has side effects "
                               "is not supported");
    }
    EmitHoistedDecl(decl);
    EmitHoistedInArmAssignment(decl);
    return;
  }

  if (!ConvertVarDeclSkipInit(decl)) {
    // Skip global variables declared extern
    return;
  }
  ScopedValue static_init(in_const_initializer_,
                          decl->isFileVarDecl() || decl->isStaticLocal());
  StrCat(token::kAssign);
  ConvertVarDeclInitializer(decl);
  StrCat(token::kSemiColon);
}

void Converter::ConvertGlobalVarDecl(clang::VarDecl *decl) {
  HoistMaterializedTempBindings hoist_temps(*this);
  if (!ConvertVarDeclSkipInit(decl)) {
    // Skip global variables declared extern
    return;
  }
  ScopedValue static_init(in_const_initializer_,
                          decl->isFileVarDecl() || decl->isStaticLocal());
  StrCat(token::kAssign);
  {
    PushLazyInit lazy(*this, LazyStaticInit());
    StrCat(keyword_unsafe_);
    PushBrace push(*this);
    ConvertVarDeclInitializer(decl);
  }
  StrCat(token::kSemiColon);
}

bool Converter::VisitVarDecl(clang::VarDecl *decl) {
  if (clang::isa<clang::VarTemplatePartialSpecializationDecl>(decl) ||
      decl->getDeclContext()->isDependentContext()) {
    return false;
  }

  if (IsGlobalVar(decl)) {
    if (decl->hasInit() && ReferencesVar(decl->getInit(), decl)) {
      llvm::report_fatal_error(
          "self-referential initializer of a static variable is not supported");
    }
    ConvertGlobalVarDecl(decl);
  } else {
    ConvertVarDecl(decl);
  }
  EmitScopedDestructor(decl);

  if (auto *decomp = clang::dyn_cast<clang::DecompositionDecl>(decl)) {
    for (auto *binding : decomp->bindings()) {
      if (auto *holding = binding->getHoldingVar()) {
        StrCat(token::kSemiColon);
        VisitVarDecl(holding);
      }
    }
  }

  return false;
}

bool Converter::NeedsScopedDestructor(const clang::VarDecl *decl) const {
  if (in_function_formals_ || !decl->isLocalVarDecl() || IsGlobalVar(decl)) {
    return false;
  }
  auto type = decl->getType();
  if (auto lambda = AsLambdaClass(type);
      lambda && LambdaNeedsDestruction(lambda)) {
    return true;
  }
  return !type->isReferenceType() && !type->isArrayType() &&
         TypeNeedsDestruction(type);
}

void Converter::EmitScopedDestructor(const clang::VarDecl *decl) {
  if (!NeedsScopedDestructor(decl)) {
    return;
  }
  auto type = decl->getType();
  auto destroy =
      AsLambdaClass(type)
          ? std::string("|__f| __f.destroy()")
          : std::format("{}::{}", GetRecordName(type->getAsCXXRecordDecl()),
                        kDestructorName);
  StrCat(token::kSemiColon,
         std::format("let _dtor_{0} = ScopedDestructorUnsafe::new(&raw mut "
                     "{0}, {1})",
                     GetNamedDeclAsString(decl), destroy));
}

bool Converter::RecordDerivesDefault(const clang::RecordDecl *decl) {
  if (auto cxx_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (GetUserDefinedDefaultConstructor(cxx_decl)) {
      return false;
    }
  }

  for (auto f : decl->fields()) {
    if (f->hasInClassInitializer() || !TypeDerivesDefault(f->getType())) {
      return false;
    }
  }

  return true;
}

bool Converter::TypeDerivesDefault(clang::QualType qual_type) {
  // Rust only implements Default for arrays of up to 32 elements
  if (auto *array_type = ctx_.getAsConstantArrayType(qual_type)) {
    return array_type->getSize().ule(32) &&
           TypeDerivesDefault(array_type->getElementType());
  }
  if (qual_type->isArrayType()) {
    return false;
  }

  auto *cxx_record = qual_type->getAsCXXRecordDecl();
  if (IsStdDecl(cxx_record, "array") || IsStdDecl(cxx_record, "basic_string")) {
    return false;
  }

  // libc types do not implement Default
  if (auto record = qual_type->getAsRecordDecl()) {
    if (ctx_.getSourceManager().isInSystemHeader(record->getLocation()) &&
        qual_type.isPODType(ctx_)) {
      return false;
    }
  }

  if (auto lambda = AsLambdaClass(qual_type);
      lambda && lambda->captures().empty()) {
    return false;
  }

  return true;
}

bool Converter::IsPassThroughRule(clang::Expr *expr) {
  const auto *rule = RuleRegistry::GetExprRule(ctx_, expr);
  if (!rule || rule->body.size() != 1) {
    return false;
  }
  auto *ph = std::get_if<TranslationRule::PlaceholderFragment>(&rule->body[0]);
  if (!ph) {
    return false;
  }
  // A pointer to the elements of a container argument is a new pointer.
  auto *call = clang::dyn_cast<clang::CallExpr>(expr);
  return !call ||
         !ParamIsElementPtr(
             expr,
             BuildUnifiedArgs(call, call->getArgs(), call->getNumArgs())[ph->n],
             ph->n);
}

bool Converter::RecordDerivesCopy(clang::QualType type) {
  auto decl = type->getAsRecordDecl();
  if (auto cxx = llvm::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && cxx->isLambda()) {
    return false;
  }
  if (auto rule = RuleRegistry::Search(ctx_, type).first) {
    auto &derives = rule->type_info.derives;
    return std::find(derives.begin(), derives.end(), "Copy") != derives.end();
  }
  decl = decl->getDefinition();
  if (!decl) {
    return false;
  }
  auto attrs = GetStructAttributes(decl);
  return std::find(attrs.begin(), attrs.end(), std::string_view("Copy")) !=
         attrs.end();
}

bool Converter::RecordHasCopyableFields(const clang::RecordDecl *decl) {
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && RecordNeedsDestruction(cxx)) {
    return false;
  }
  for (auto f : decl->fields()) {
    if (auto ptr_ty = f->getType()->getAs<clang::PointerType>()) {
      if (ptr_ty->getPointeeType()->isFunctionType()) {
        if (!FunctionPointerImplementsCopy()) {
          return false;
        }
      }
    }

    // Look recursively into fields that are RecordDecl, or arrays of them
    if (auto field_type = ctx_.getBaseElementType(f->getType());
        field_type->getAsRecordDecl()) {
      if (!RecordDerivesCopy(field_type)) {
        return false;
      }
    }
  }

  return true;
}

bool Converter::VisitRecordDecl(clang::RecordDecl *decl) {
  decl->dump(log());

  // VisitCXXRecordDecl already visited the record
  if (clang::isa<clang::CXXRecordDecl>(decl)) {
    return true;
  }

  if (!decl->isCompleteDefinition()) {
    return false;
  }

  if (!record_decls_.MarkDefined(GetRecordName(decl))) {
    return false;
  }

  EmitRustStructOrUnion(decl);

  return false;
}

void Converter::EmitRustStructOrUnion(clang::RecordDecl *decl) {
  // Enums and static variables. In rust they live outside the record
  for (auto *d : decl->decls()) {
    if (auto *enum_decl = llvm::dyn_cast<clang::EnumDecl>(d)) {
      VisitEnumDecl(enum_decl);
    }
    if (auto *var_decl = clang::dyn_cast<clang::VarDecl>(d)) {
      if (auto *def = var_decl->getDefinition();
          def && var_decl->getInstantiatedFromStaticDataMember() &&
          !var_decl->hasInit()) {
        var_decl = def;
      }
      VisitVarDecl(var_decl);
    }
    if (auto *friend_decl = clang::dyn_cast<clang::FriendDecl>(d)) {
      if (auto *fn = clang::dyn_cast_or_null<clang::FunctionDecl>(
              friend_decl->getFriendDecl());
          fn && fn->isThisDeclarationADefinition()) {
        VisitFunctionDecl(fn);
      }
      if (auto *tmpl = clang::dyn_cast_or_null<clang::FunctionTemplateDecl>(
              friend_decl->getFriendDecl())) {
        for (auto *spec : tmpl->specializations()) {
          if (spec->isThisDeclarationADefinition()) {
            VisitFunctionDecl(spec);
          }
        }
      }
    }
  }

  // Inner records. In rust they live outside the record
  for (auto *d : decl->decls()) {
    if (auto *nested = clang::dyn_cast<clang::RecordDecl>(d)) {
      if (!nested->isImplicit()) {
        inner_structs_[GetID(nested)] = GetRecordName(nested);
        if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(nested)) {
          VisitCXXRecordDecl(cxx);
        } else {
          VisitRecordDecl(nested);
        }
      }
    }
    if (auto *nested_tmpl = clang::dyn_cast<clang::ClassTemplateDecl>(d)) {
      for (auto *spec : nested_tmpl->specializations()) {
        inner_structs_[GetID(spec)] = GetRecordName(spec);
        VisitCXXRecordDecl(spec);
      }
    }
  }

  if (decl->isUnion()) {
    EmitRustUnion(decl);
    return;
  }

  // Derived traits
  if (EmitsReprCForRecords()) {
    EmitReprC(decl);
  }
  EmitDerives(decl);
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    EmitConversionAttrs(cxx);
  }
  EmitByteSizeAttr(decl);

  // Fields
  auto access = clang::dyn_cast<clang::CXXRecordDecl>(decl)
                    ? AccessSpecifierAsString(decl->getAccess())
                    : keyword::kPub;
  StrCat(access, keyword::kStruct, GetRecordName(decl));
  {
    PushBrace brace(*this);
    for (auto *field : decl->fields()) {
      VisitFieldDecl(field);
    }
  }

  // C++ method decls and traits
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    ConvertCXXRecordMethods(cxx);
    ConvertVirtualMethods(cxx);
    AddOrdTrait(cxx);
  }
  AddCloneTrait(decl);
  AddDefaultTrait(decl);
}

void Converter::ConvertLateInstantiatedMethods(clang::CXXRecordDecl *decl) {
  ConvertCXXMethodDecls(
      decl, std::format("{} {}", keyword::kImpl, GetRecordName(decl)),
      [](auto *method) {
        return IsEmittableMethod(method) && method->hasBody() &&
               !decl_ids_.contains(GetMethodID(method));
      });
}

void Converter::ConvertCXXRecordMethods(clang::CXXRecordDecl *decl) {
  ConvertCXXMethodDecls(
      decl, std::format("{} {}", keyword::kImpl, GetRecordName(decl)),
      IsEmittableMethod);

  if (GetUserDefinedDestructor(decl) || !HasFieldsNeedingDestruction(decl)) {
    return;
  }
  StrCat(keyword::kImpl, GetRecordName(decl));
  PushBrace impl_brace(*this);
  StrCat(keyword::kPub, keyword_unsafe_, keyword::kFn, kDestructorName,
         "(&mut self)");
  PushBrace fn_brace(*this);
  StrCat(DestroyMembers(decl));
}

std::string Converter::DestroyMembers(const clang::CXXRecordDecl *decl) {
  std::string out;
  for (auto *field : FieldsToDestroy(decl)) {
    auto name = GetNamedDeclAsString(field);
    auto type = field->getType();
    if (type->isArrayType()) {
      auto *elem = type->getBaseElementTypeUnsafe()->getAsCXXRecordDecl();
      assert(elem);
      out +=
          std::format("for __e in self.{0}.iter_mut() {{ {1}::{2}(__e); }}\n",
                      name, GetRecordName(elem), kDestructorName);
    } else {
      out += std::format("{1}::{2}(&mut self.{0});\n", name,
                         GetRecordName(type->getAsCXXRecordDecl()),
                         kDestructorName);
    }
  }
  return out;
}

void Converter::EmitDerives(const clang::RecordDecl *decl) {
  StrCat("#[derive(");
  for (auto *attr : GetStructAttributes(decl)) {
    StrCat(attr, ',');
  }
  StrCat(")]");
}

void Converter::EmitConversionAttrs(clang::CXXRecordDecl *decl) {
  std::unordered_set<std::string> from_types;
  ForEachConvertingConstructor(decl, [&](clang::CXXConstructorDecl *ctor) {
    auto *param = ctor->getParamDecl(0);
    auto type = param->getType();
    auto source =
        GetUnboxedTypeAsString(type.getNonReferenceType().getUnqualifiedType());
    bool is_move = type->isRValueReferenceType();
    if (!from_types.insert(source + (is_move ? "&&" : "")).second) {
      return;
    }
    const char *kind =
        !type->isReferenceType()                                   ? "value"
        : is_move || type.getNonReferenceType().isConstQualified() ? "ptr"
                                                                   : "mut_ptr";
    StrCat(std::format(
        "#[{}({}, {}, {}{})]", is_move ? "cpp_from_move" : "cpp_from", source,
        GetCtorName(ctor), param->hasDefaultArg() ? "some_" : "", kind));
  });
  std::unordered_set<std::string> into_types;
  ForEachMethod(decl, [&](clang::CXXMethodDecl *method) {
    if (!IsImplicitConversionOperator(method)) {
      return;
    }
    auto *conversion = clang::cast<clang::CXXConversionDecl>(method);
    auto target = GetUnboxedTypeAsString(
        conversion->getConversionType().getUnqualifiedType());
    if (!into_types.insert(target).second) {
      return;
    }
    StrCat(
        std::format("#[cpp_into({}, {})]", target, GetMethodName(conversion)));
  });
}

void Converter::EmitReprC(clang::RecordDecl *decl) {
  if (decl->hasAttr<clang::AlignedAttr>()) {
    StrCat(std::format("#[repr(C, align({}))]",
                       ctx_.getTypeAlign(ctx_.getCanonicalTagType(decl)) / 8));
    return;
  }
  StrCat("#[repr(C)]");
}

void Converter::EmitRustUnion(clang::RecordDecl *decl) {
  EmitReprC(decl);
  EmitDerives(decl);

  StrCat(keyword::kPub, keyword::kUnion, GetRecordName(decl));
  {
    PushBrace brace(*this);
    if (decl->field_empty()) {
      StrCat("__empty: u8,");
    }
    for (auto *field : decl->fields()) {
      VisitFieldDecl(field);
    }
  }

  AddDefaultTrait(decl);
}

bool Converter::VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
  decl->dump(log());

  if (!IsConvertibleCXXRecordDecl(decl)) {
    return false;
  }

  if (!decl->isStruct() && !decl->isClass() && !decl->isUnion()) {
    // FIXME: improve error handling
    assert(0 && "unsupported record kind");
    return false;
  }

  if (!decl->isUnion()) {
    for (auto c : GetTemplateInstantiatedCtors(decl)) {
      if (!decl_ids_.contains(GetID(c))) {
        StrCat(keyword::kImpl, GetRecordName(decl));
        PushBrace brace(*this);
        VisitCXXMethodDecl(c);
      }
    }
  }

  if (!record_decls_.MarkDefined(GetRecordName(decl))) {
    // Other translation units may instantiate members this one did not.
    if (!decl->isUnion() && !decl->isAbstract()) {
      ConvertLateInstantiatedMethods(decl);
    }
    return false;
  }

  if (decl->isAbstract()) {
    ConvertAbstractClass(decl);
    return false;
  }

  if (!decl->isUnion()) {
    DefineImplicitMembers(*sema_, decl);
  }
  EmitRustStructOrUnion(decl);
  return false;
}

bool Converter::VisitCXXMethodDecl(clang::CXXMethodDecl *decl) {
  decl->dump(log());
  if (!ShouldConvertMethod(decl)) {
    return false;
  }
  if (decl->getDescribedFunctionTemplate() || decl->isDependentContext() ||
      (!decl->isPureVirtual() && !decl->hasBody())) {
    return false;
  }
  if (!decl_ids_.insert(GetMethodID(decl)).second) {
    return false;
  }
  ScopedValue push_fn(curr_function_, decl);

  if (decl->isOutOfLine() && !decl->overridden_methods().empty()) {
    return ConvertOutOfLineVirtualMethod(decl);
  }
  if (decl->isOutOfLine() && !decl->isTemplateInstantiation()) {
    return ConvertOutOfLineMethod(decl);
  }
  return ConvertCXXMethodDecl(decl);
}

bool Converter::ShouldConvertMethod(const clang::CXXMethodDecl *decl) {
  return IsConvertibleCXXMethodDecl(decl);
}

bool Converter::ConvertOutOfLineMethod(clang::CXXMethodDecl *decl) {
  StrCat(keyword::kImpl, GetRecordName(decl->getParent()));
  PushBrace impl_brace(*this);
  return ConvertCXXMethodDecl(decl);
}

std::string Converter::GetMethodName(const clang::CXXMethodDecl *decl) {
  if (clang::isa<clang::CXXDestructorDecl>(decl)) {
    return kDestructorName;
  }
  if (const char *name = GetCopyOrMoveName(decl);
      name && CanUseCopyOrMoveName(decl, name)) {
    return name;
  }
  if (IsOverloadedMethod(decl)) {
    return GetOverloadedFunctionName(decl);
  }
  if (auto *conversion = clang::dyn_cast<clang::CXXConversionDecl>(decl)) {
    auto name = GetConversionName(conversion);
    bool shared =
        llvm::any_of(conversion->getParent()->methods(),
                     [&](const clang::CXXMethodDecl *method) {
                       auto *other =
                           clang::dyn_cast<clang::CXXConversionDecl>(method);
                       return other && other != conversion &&
                              GetConversionName(other) == name;
                     });
    return shared ? name + std::format("_{}", GetMethodIndex(conversion))
                  : name;
  }
  return GetNamedDeclAsString(decl);
}

std::string
Converter::GetConversionName(const clang::CXXConversionDecl *decl) const {
  auto name = "to_" + GetUnsafeTypeAsString(decl->getConversionType());
  ToIdentifier(name);
  return HasMethodNamed(decl->getParent(), name) ? GetFunctionBaseName(decl)
                                                 : name;
}

bool Converter::ConvertCXXMethodDecl(clang::CXXMethodDecl *decl) {
  if (auto *ctor = clang::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return VisitCXXConstructorDecl(ctor);
  }

  if (method_target_ == MethodTarget::ValueImpl &&
      (decl->isStatic() ||
       (!decl->isVirtual() && !decl->getParent()->isAbstract()))) {
    StrCat(AccessSpecifierAsString(decl->getAccess()));
  }
  StrCat(keyword_unsafe_, keyword::kFn, GetMethodName(decl));

  {
    PushParen paren(*this);
    if (!decl->isStatic()) {
      StrCat(GetSelfMaybeWithMut(decl), token::kComma);
    }
    ConvertFunctionParameters(decl);
  }
  ConvertFunctionReturnType(decl);
  if (decl->isPureVirtual() || method_target_ == MethodTarget::TraitDecl) {
    StrCat(token::kSemiColon);
  } else if (method_target_ == MethodTarget::TraitDefault) {
    PushBrace body(*this);
    StrCat("unimplemented!()");
  } else {
    PushBrace body(*this);
    EmitFunctionPreamble(decl);
    ConvertFunctionBody(decl);
  }
  return false;
}

std::string Converter::GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) {
  return MethodNeedsMutableReceiver(decl) ? "&mut self" : "&self";
}

std::string Converter::GetCtorName(clang::CXXConstructorDecl *decl) {
  if (decl->isCopyOrMoveConstructor()) {
    if (const char *name = GetCopyOrMoveName(decl);
        CanUseCopyOrMoveName(decl, name)) {
      return name;
    }
    return std::format("new_{}", GetCtorIndex(decl));
  }
  return GetNumberOfConvertingCtors(decl->getParent()) != 1
             ? std::format("new_{}", GetCtorIndex(decl))
             : "new";
}

bool Converter::VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) {
  if (decl->isOutOfLine() ||
      (decl->isImplicit() && !IsConvertibleImplicitMember(decl))) {
    return false;
  }
  ScopedValue push_fn(curr_function_, decl);

  if (decl->isCopyOrMoveConstructor() && !decl->hasBody()) {
    return false;
  }

  StrCat(AccessSpecifierAsString(decl->getAccess()), keyword_unsafe_,
         keyword::kFn, GetCtorName(decl));
  {
    PushParen paren(*this);
    ConvertFunctionParameters(decl);
  }
  StrCat(token::kArrow, "Self");
  {
    PushBrace brace(*this);
    ConvertCXXConstructorBody(decl);
  }

  return false;
}

void Converter::ConvertCXXConstructorBody(clang::CXXConstructorDecl *decl) {
  EmitFunctionPreamble(decl);
  StrCat(keyword::kLet, "mut", "this", token::kAssign);
  EmitConstructorInit(decl);
  StrCat(token::kSemiColon);
  ConvertBodyStmts(decl->getBody());
  StrCat("this");
}

void Converter::EmitConstructorInit(clang::CXXConstructorDecl *decl) {
  if (decl->isDelegatingConstructor()) {
    Convert((*decl->init_begin())->getInit());
    return;
  }
  StrCat("Self");
  PushBrace this_init(*this);
  const auto *record_decl = decl->getParent();
  auto *definition_or_null = decl->getDefinition();
  assert(definition_or_null);
  auto *definition = clang::cast<clang::CXXConstructorDecl>(definition_or_null);

  for (const auto *field : record_decl->fields()) {
    auto field_name = GetNamedDeclAsString(field);
    const clang::CXXCtorInitializer *ctor_initializer = nullptr;
    for (const auto *init : definition->inits()) {
      if (init->isMemberInitializer() && init->getMember() == field) {
        ctor_initializer = init;
        break;
      }
    }

    StrCat(field_name, token::kColon);
    ConvertFieldInit(field, ctor_initializer ? ctor_initializer->getInit()
                                             : field->getInClassInitializer());
    StrCat(token::kComma);
  }
}

bool Converter::VisitFieldDecl(clang::FieldDecl *decl) {
  auto access_spec = AccessSpecifierAsString(decl->getAccess());
  auto field_name = GetNamedDeclAsString(decl);
  StrCat(access_spec, std::move(field_name), token::kColon);
  Convert(decl->getType());
  StrCat(token::kComma);
  return false;
}

std::string
Converter::EmitDefaultArgStorage(const clang::ParmVarDecl *param,
                                 const clang::MaterializeTemporaryExpr *temp) {
  auto storage = std::format("__{}_default", GetNamedDeclAsString(param));
  StrCat(std::format("let mut {} : Option<{}> = None", storage,
                     ToString(temp->getType().getUnqualifiedType())),
         token::kSemiColon);
  return storage;
}

void Converter::EmitFunctionPreamble(clang::FunctionDecl *decl) {
  for (auto *param : DefinitionParameters(decl)) {
    if (!HasUsableDefaultArg(param)) {
      continue;
    }
    auto name = GetNamedDeclAsString(param);
    auto type = ToString(param->getType());
    std::string value;
    if (auto *temp = GetDefaultArgTemporary(param)) {
      auto storage = EmitDefaultArgStorage(param, temp);
      value = std::format("{}.insert({}) as {}", storage,
                          ToString(temp->getSubExpr()), type);
    } else if (param->getType()->isReferenceType()) {
      Buffer buf(*this);
      ConvertVarInit(param->getType(), param->getDefaultArg());
      value = std::move(buf).str();
    } else {
      value = ToString(param->getDefaultArg());
    }
    StrCat(std::format("let mut {} : {} = {}.unwrap_or_else(|| {} {{ {} }})",
                       name, type, name, keyword_unsafe_, value),
           token::kSemiColon);
  }
}

void Converter::ConvertDeclContext(clang::DeclContext *decl) {
  for (auto *child : decl->decls()) {
    if (IsInMainFile(child) || !decl_ids_.contains(GetID(child))) {
      Convert(child);
    }
  }
}

bool Converter::VisitNamespaceDecl(clang::NamespaceDecl *decl) {
  ConvertDeclContext(decl);
  return false;
}

bool Converter::VisitLinkageSpecDecl(clang::LinkageSpecDecl *decl) {
  ConvertDeclContext(decl);
  return false;
}

bool Converter::VisitTypedefDecl([[maybe_unused]] clang::TypedefDecl *decl) {
  return false;
}

bool Converter::VisitTypeAliasDecl(clang::TypeAliasDecl *) { return false; }

bool Converter::VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl *) {
  return false;
}

bool Converter::VisitStaticAssertDecl(clang::StaticAssertDecl *decl) {
  auto *assert_expr = decl->getAssertExpr();
  if (assert_expr->isValueDependent()) {
    return false;
  }
  std::string condition;
  if (assert_expr->getType()->isBooleanType() &&
      IsRustConstEvaluableExpr(assert_expr)) {
    condition = ToString(assert_expr);
  } else {
    bool value = false;
    ENSURE(assert_expr->EvaluateAsBooleanCondition(value, ctx_,
                                                   /*InConstantContext=*/true));
    condition = value ? keyword::kTrue : keyword::kFalse;
  }
  StrCat(std::format("const _: () = assert!({}{});", condition,
                     GetAssertMessageAsString(assert_expr, ctx_)));
  return false;
}

bool Converter::VisitConceptDecl(clang::ConceptDecl *) { return false; }

static bool IsaSemiColonStmt(const clang::Stmt *stmt) {
  switch (stmt->getStmtClass()) {
  case clang::Stmt::IfStmtClass:
  case clang::Stmt::WhileStmtClass:
  case clang::Stmt::DoStmtClass:
  case clang::Stmt::ForStmtClass:
  case clang::Stmt::CompoundStmtClass:
  case clang::Stmt::CXXForRangeStmtClass:
  case clang::Stmt::CaseStmtClass:
  case clang::Stmt::DefaultStmtClass:
    return false;
  default:
    return true;
  }
}

bool Converter::Convert(clang::Stmt *stmt) {
  PushExprKind push(*this, ExprKind::Void);
  auto exited_visit = TraverseStmt(stmt);
  if (stmt && IsaSemiColonStmt(stmt)) {
    StrCat(token::kSemiColon);
  }
  return exited_visit;
}

void Converter::ConvertBody(clang::Stmt *body) {
  PushBrace brace(*this);
  ConvertBodyStmts(body);
}

void Converter::ConvertBodyStmts(clang::Stmt *body) {
  auto *compound = clang::dyn_cast_or_null<clang::CompoundStmt>(body);
  if (!compound) {
    Convert(body);
    return;
  }
  if (CompoundHasTopLevelLabel(compound)) {
    ConvertGotoBlock(compound);
    return;
  }
  for (auto *child : compound->body()) {
    Convert(child);
  }
}

bool Converter::VisitCompoundStmt(clang::CompoundStmt *stmt) {
  ConvertBody(stmt);
  return false;
}

bool Converter::VisitDeclStmt(clang::DeclStmt *stmt) {
  for (auto *decl : stmt->decls()) {
    auto *var = clang::dyn_cast<clang::VarDecl>(decl);
    if (clang::isa<clang::TagDecl>(decl) || (var && var->isStaticLocal())) {
      Buffer buf(*this);
      Convert(decl);
      hoisted_records_ += std::move(buf).str();
      continue;
    }
    if (var && IsAliasOfInit(ctx_, var)) {
      continue;
    }
    if (var && var->isImplicit()) {
      VisitVarDecl(var);
    } else {
      Convert(decl);
    }
    StrCat(token::kSemiColon);
  }
  return false;
}

bool Converter::VisitReturnStmt(clang::ReturnStmt *stmt) {
  auto return_type = curr_function_->getReturnType();
  if (!return_type->isVoidType()) {
    HoistMaterializedTempBindings hoist_temps(*this);
    StrCat(keyword::kReturn);
    ConvertVarInit(return_type, stmt->getRetValue());
  } else {
    Convert(stmt->getRetValue());
    StrCat(token::kSemiColon, keyword::kReturn, token::kSemiColon);
  }
  return false;
}

bool Converter::VisitGotoStmt(clang::GotoStmt *stmt) {
  StrCat(std::format("goto!('{})", stmt->getLabel()->getName().str()));
  return false;
}

void Converter::ConvertCondition(clang::Expr *cond) {
  PushExprKind push(*this, ExprKind::RValue);
  Convert(NormalizeToBool(cond, ctx_));
}

bool Converter::VisitIfStmt(clang::IfStmt *stmt) {
  // The variables of the init statement are only visible in the if.
  PushBrace scope(*this, stmt->getInit());
  if (auto *init = stmt->getInit()) {
    Convert(init);
  }
  if (bool taken = false;
      !stmt->getConditionVariable() && !stmt->isConsteval() &&
      !stmt->getCond()->isValueDependent() &&
      stmt->getCond()->EvaluateAsBooleanCondition(taken, ctx_)) {
    ConvertBody(taken ? stmt->getThen() : stmt->getElse());
    return false;
  }
  StrCat(keyword::kIf);
  ConvertCondition(stmt->getCond());
  ConvertBody(stmt->getThen());
  if (stmt->hasElseStorage()) {
    StrCat(keyword::kElse);
    if (clang::isa<clang::IfStmt>(stmt->getElse())) {
      Convert(stmt->getElse());
    } else {
      ConvertBody(stmt->getElse());
    }
  }
  return false;
}

bool Converter::VisitWhileStmt(clang::WhileStmt *stmt) {
  ScopedPush push(break_target_, BreakTarget::Loop);
  StrCat("'loop_:");
  StrCat(keyword::kWhile);
  ConvertCondition(stmt->getCond());
  ScopedPush no_inc(curr_for_inc_, nullptr);
  ConvertBody(stmt->getBody());
  return false;
}

bool Converter::VisitDoStmt(clang::DoStmt *stmt) {
  ScopedPush push(break_target_, BreakTarget::Loop);
  const char *control_var = "__do_while";
  StrCat(keyword::kLet, "mut", control_var, token::kAssign, keyword::kTrue,
         token::kSemiColon);
  StrCat("'loop_:", keyword::kWhile, control_var, "||");
  {
    PushParen paren(*this);
    ConvertCondition(stmt->getCond());
  }
  {
    PushBrace loop_brace(*this);
    StrCat(control_var, token::kAssign, keyword::kFalse, token::kSemiColon);
    ScopedPush no_inc(curr_for_inc_, nullptr);
    ConvertBodyStmts(stmt->getBody());
  }
  return false;
}

bool Converter::VisitForStmt(clang::ForStmt *stmt) {
  ScopedPush push(break_target_, BreakTarget::Loop);
  Convert(stmt->getInit());
  StrCat("'loop_:");
  StrCat(keyword::kWhile);
  if (stmt->getCond() == nullptr) {
    StrCat("true");
  } else {
    ConvertCondition(stmt->getCond());
  }
  {
    PushBrace brace(*this);
    {
      ScopedPush inc(curr_for_inc_, stmt->getInc());
      ConvertBodyStmts(stmt->getBody());
    }
    Convert(stmt->getInc());
    StrCat(token::kSemiColon);
  }
  return false;
}

bool Converter::VisitCXXForRangeStmt(clang::CXXForRangeStmt *stmt) {
  ScopedPush push(break_target_, BreakTarget::Loop);
  Convert(stmt->getInit());
  Convert(stmt->getRangeStmt());
  auto *begin =
      clang::cast<clang::VarDecl>(stmt->getBeginStmt()->getSingleDecl());
  auto *end = clang::cast<clang::VarDecl>(stmt->getEndStmt()->getSingleDecl());
  // The iterator is named after the loop variable, which shadows it.
  auto name = GetNamedDeclAsString(stmt->getLoopVariable());
  range_iterator_names_[begin] = name;
  StrCat("'loop_:");
  StrCat(keyword::kFor, name, keyword::kIn, "IterRange::new(",
         ConvertFreshRValue(begin->getInit()), token::kComma,
         ConvertFreshRValue(end->getInit()), ')');
  {
    PushBrace brace(*this);
    ScopedPush no_inc(curr_for_inc_, nullptr);
    Convert(stmt->getLoopVarStmt());
    ConvertBodyStmts(stmt->getBody());
  }
  return false;
}

bool Converter::VisitBreakStmt([[maybe_unused]] clang::BreakStmt *stmt) {
  StrCat(keyword::kBreak);
  if (isSwitchBreak()) {
    StrCat("'switch");
  }
  return false;
}

bool Converter::VisitContinueStmt([[maybe_unused]] clang::ContinueStmt *stmt) {
  if (!curr_for_inc_.empty()) {
    Convert(curr_for_inc_.back());
    StrCat(token::kSemiColon);
  }
  StrCat(keyword::kContinue);
  StrCat("'loop_");
  return false;
}

bool Converter::Convert(clang::Expr *expr,
                        std::optional<clang::QualType> implicit_convert_to) {
  if (expr && implicit_convert_to) {
    auto to = *implicit_convert_to;
    if (NeedsImplicitPointeeCast(ctx_, expr, to)) {
      StrCat(ConvertPointeeCast(ToString(expr), expr, to));
      return false;
    }
    if (NeedsImplicitScalarCast(ctx_, expr->IgnoreImplicit()->getType(), to)) {
      StrCat(ConvertScalarCast(ToString(expr), to));
      return false;
    }
  }
  computed_expr_type_ = ComputedExprType::Unknown;
  bool result = TraverseStmt(expr);
  if (expr && computed_expr_type_ == ComputedExprType::Unknown) {
    expr->dump();
    assert(false && "computed_expr_type_ not set");
  }
  return result;
}

std::string Converter::ConvertScalarCast(std::string str, clang::QualType to) {
  Buffer buf(*this);
  {
    PushParen paren(*this);
    StrCat(str);
    ConvertCast(to);
  }
  MarkFresh();
  return std::move(buf).str();
}

std::string Converter::ConvertPointeeCast(std::string str,
                                          const clang::Expr *from,
                                          clang::QualType to) {
  auto pointee = GetExprPointee(ctx_, from, to);
  if (to->isFunctionPointerType()) {
    return std::format("std::mem::transmute::<{}, {}>({})",
                       ToString(ctx_.getPointerType(pointee)), ToString(to),
                       str);
  }
  if (to->getPointeeType().isConstQualified()) {
    pointee = pointee.withConst();
  }
  auto target = GetUnsafeTypeAsString(to->getPointeeType());
  if (to->isReferenceType() && !isAddrOf()) {
    return std::format("*(&raw {} {}).cast::<{}>()",
                       pointee.isConstQualified() ? "const" : "mut", str,
                       target);
  }
  MarkFresh();
  return std::format("({} as {}).cast::<{}>()", str,
                     GetUnsafeTypeAsString(ctx_.getPointerType(pointee)),
                     target);
}

bool Converter::GetFmtArg(clang::Expr *arg, std::string &fmt,
                          std::string &fmt_args, const char *&fmt_trait,
                          std::string &fmt_width) {
  if (auto *str_lit =
          clang::dyn_cast<clang::StringLiteral>(arg->IgnoreImplicit())) {
    if (!IsAsciiStringLiteral(str_lit)) {
      return false;
    }
    auto str = GetEscapedStringLiteral(arg);
    std::string_view trim(str);
    // Delete " from string
    trim.remove_prefix(1);
    trim.remove_suffix(1);
    fmt += trim;
  } else if (auto ch = GetEscapedUTF8CharLiteral(arg); !ch.empty()) {
    fmt += std::move(ch);
  } else if (IsStdFunction(arg, "endl")) {
    fmt += "\\n";
  } else if (IsStdFunction(arg, "hex")) {
    fmt_trait = "x";
  } else if (IsStdFunction(arg, "dec")) {
    fmt_trait = "";
  } else if (IsStdFunction(arg, "setw")) {
    fmt_width = Trim(ToString(arg));
  } else if (!arg->getType()->isCharType() &&
             Mapper::Map(*this, arg->getType()) !=
                 std::format("Vec<{}>", CharRustType())) {
    fmt += ("{:" + fmt_width + fmt_trait + "}");
    fmt_width.clear(); // Reset setw after first usage
    auto arg_str = ToString(arg);
    if (arg->getType()->isBooleanType()) {
      arg_str = std::format("({} as u8)", std::move(arg_str));
    }
    fmt_args += std::move(arg_str) + ", ";
  } else {
    return false;
  }
  return true;
}

bool Converter::GetRawArg(clang::Expr *arg, std::string &raw_args) {
  if (arg->getType()->isCharType()) {
    raw_args += "(&[" + ToString(arg) + " as u8]";
  } else if (Mapper::Map(*this, arg->getType()) ==
             std::format("Vec<{}>", CharRustType())) {
    PushExprKind push(*this, ExprKind::RValue);
    std::string str = ToString(arg);
    raw_args += "(&(" + str + ").iter().take((" + str +
                ").len() - 1).map(|&c| c as u8).collect::<Vec<u8>>()[..]";
  } else if (IsStdFunction(arg, "endl")) {
    raw_args += "(&[b'\\n']";
  } else if (clang::isa<clang::StringLiteral>(arg->IgnoreImplicit())) {
    raw_args += "(b" + GetEscapedStringLiteral(arg);
  } else {
    return false;
  }
  raw_args += " as &[u8]), ";
  return true;
}

std::string Converter::ConvertStream(clang::Expr *expr) {
  return ToString(expr);
}

void Converter::ConvertCallToOstream(clang::CallExpr *expr) {
  clang::Expr *stream = nullptr;
  auto collect_args = [expr, &stream]() -> std::vector<clang::Expr *> {
    std::vector<clang::Expr *> result;
    auto *current = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
    if (!current) {
      return {};
    }

    while (current) {
      result.push_back(current->getArg(1));
      if (auto *next =
              clang::dyn_cast<clang::CXXOperatorCallExpr>(current->getArg(0));
          next && IsCallToOstream(next)) {
        current = next;
      } else {
        stream = current->getArg(0);
        break;
      }
    }

    std::reverse(result.begin(), result.end());
    return result;
  };

  std::vector<clang::Expr *> args = collect_args();
  if (args.empty()) {
    return;
  }

  std::string fmt;
  const char *fmt_trait = "";
  std::string fmt_width;
  std::string fmt_args;
  std::string raw_args;
  std::string stream_str = ConvertStream(stream);
  size_t arg_count = args.size();

  auto write_raw_args = [&]() {
    if (!raw_args.empty()) {
      StrCat(stream_str, ".write_all(&([", std::move(raw_args),
             "].concat()));");
      raw_args.clear();
    }
  };

  auto write_fmt_args = [&]() {
    if (!fmt_args.empty() || !fmt.empty()) {
      StrCat("write!(", stream_str, ',',
             std::format(R"("{}",)", std::move(fmt)), std::move(fmt_args),
             ");");
      fmt_args.clear();
      fmt.clear();
    }
  };

  size_t i = 0;
  while (i < arg_count) {
    while (i < arg_count &&
           GetFmtArg(args[i], fmt, fmt_args, fmt_trait, fmt_width))
      ++i;
    write_fmt_args();
    while (i < arg_count && GetRawArg(args[i], raw_args))
      ++i;
    write_raw_args();
  }

  assert(*fmt_trait == '\0' && "Stream state was not restored after call");
}

void Converter::ConvertVariadicArg(clang::Expr *arg) {
  PushExprKind push(*this, ExprKind::RValue);
  Convert(arg);
  if (arg->getType()->isFunctionPointerType()) {
    StrCat(".map_or(::std::ptr::null_mut(), |f| f as *mut ::libc::c_void)");
  }
}

void Converter::ConvertVAArgCall(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr)) {
    StrCat(ToString(expr->getArg(0)->IgnoreImpCasts()), "= __args.as_ptr()");
    return;
  }
  if (IsBuiltinVaEnd(expr)) {
    // va_end is a no-op
    return;
  }
  if (IsBuiltinVaCopy(expr)) {
    StrCat(ToString(expr->getArg(0)->IgnoreImpCasts()), '=',
           ToString(expr->getArg(1)->IgnoreImpCasts()), ".clone()");
    return;
  }
}

bool Converter::VisitCallExpr(clang::CallExpr *expr) {
  if (ConvertSpecialCall(expr)) {
    return false;
  }
  std::optional<TempMaterializationCtx> temps;
  std::string str;
  {
    Buffer buf(*this);
    temps = ConvertCallExpr(expr);
    str = std::move(buf).str();
  }
  EmitCallResult(expr, std::move(str), temps ? &*temps : nullptr);
  return false;
}

bool Converter::ConvertSpecialCall(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr) || IsBuiltinVaEnd(expr) || IsBuiltinVaCopy(expr)) {
    ConvertVAArgCall(expr);
    MarkFresh();
    return true;
  }

  // p->~T() on a scalar is a no-op
  if (clang::isa<clang::CXXPseudoDestructorExpr>(
          expr->getCallee()->IgnoreParenImpCasts())) {
    MarkFresh();
    return true;
  }

  // p->~T() is a no-op when T has nothing to destruct
  if (auto *dtor = clang::dyn_cast_or_null<clang::CXXDestructorDecl>(
          expr->getCalleeDecl());
      dtor && !RecordNeedsDestruction(dtor->getParent())) {
    MarkFresh();
    return true;
  }

  if (auto conversion = clang::dyn_cast_or_null<clang::CXXConversionDecl>(
          expr->getCalleeDecl());
      conversion && conversion->getParent()->isLambda()) {
    ConvertLambdaToFunctionPointer(clang::cast<clang::CXXMemberCallExpr>(expr)
                                       ->getImplicitObjectArgument(),
                                   conversion);
    return true;
  }

  // The remaining calls are converted as rules say, if there is one.
  if (Mapper::Contains(ctx_, expr)) {
    return false;
  }

  if (IsImplicitAssignmentCall(expr)) {
    auto *call = clang::cast<clang::CXXMemberCallExpr>(expr);
    ConvertAssignment(call->getImplicitObjectArgument(), call->getArg(0), "=");
    return true;
  }

  if (IsTransparentStdCall(expr)) {
    Convert(expr->getArg(0));
    return true;
  }

  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
      opcall && !IsUserOperatorCall(opcall)) {
    ConvertCXXOperatorCallExpr(opcall);
    return true;
  }
  return false;
}

void Converter::EmitCallResult(clang::CallExpr *expr, std::string str,
                               const TempMaterializationCtx *temps) {
  bool deref = (GetReturnTypeOfFunction(expr)->isReferenceType() ||
                (temps && IsReferenceType(expr))) &&
               !isAddrOf() && !isVoid();
  // The dereference is in the block of the temporaries, so they are still
  // alive, as is the read of a field of the result.
  if (deref && !(temps && Mapper::IsRefExpr(ctx_, expr))) {
    Buffer buf(*this);
    EmitDeref(std::move(str), expr->getType());
    str = std::move(buf).str();
  }
  if (deref && DeferTemporaries(expr, temps)) {
    StrCat(str);
  } else {
    StrCat(temps ? temps->Wrap(std::move(str)) : str);
  }
  if (deref) {
    SetValueFreshness(expr->getType());
  } else if (!temps || !IsPassThroughRule(expr)) {
    MarkFresh();
  }
}

void Converter::ConvertLambdaToFunctionPointer(
    clang::Expr *lambda, const clang::CXXConversionDecl *conversion) {
  EmitLambdaFunctionPointer(AsLambdaClass(lambda->getType()),
                            AsLambdaCallOperator(conversion));
}

void Converter::EmitLambdaFunctionPointer(const clang::CXXRecordDecl *lambda,
                                          clang::CXXMethodDecl *call_operator) {
  StrCat("Some");
  PushParen paren(*this);
  ConvertLambdaClosure(lambda, call_operator);
  MarkFresh();
}

void Converter::ConvertLambdaCall(clang::CallExpr *expr,
                                  const clang::CXXRecordDecl *lambda) {
  StrCat(ConvertRValue(GetCallee(expr)));
  if (lambda->isGenericLambda()) {
    auto call_operator = expr->getDirectCallee();
    auto proto = call_operator->getType()->castAs<clang::FunctionProtoType>();
    StrCat(std::format(".spec::<{}>({})", ConvertFunctionPointerType(proto),
                       GetLambdaSpecializationIndex(call_operator)));
  }
  StrCat(".call");
}

void Converter::EmitFnPtrCall(clang::CallExpr *expr) {
  auto callee = GetCallee(expr);
  if (auto lambda = AsLambdaClass(callee->getType())) {
    ConvertLambdaCall(expr, lambda);
    return;
  }
  {
    PushParen paren(*this);
    Convert(callee);
  }
  StrCat(".unwrap()");
}

std::string Converter::GetFunctionRefName(const clang::FunctionDecl *fn_decl) {
  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(fn_decl);
      method && method->isStatic()) {
    return std::format("{}::{}", GetRecordName(method->getParent()),
                       GetMethodName(method));
  }
  return Mapper::MapFunctionName(ctx_, fn_decl);
}

void Converter::ConvertFunctionToFunctionPointer(
    const clang::FunctionDecl *fn_decl) {
  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(fn_decl);
      method && method->isLambdaStaticInvoker()) {
    assert(!method->getParent()->isGenericLambda());
    EmitLambdaFunctionPointer(method->getParent(),
                              method->getParent()->getLambdaCallOperator());
    return;
  }
  EmitFresh(std::format("Some({})", GetFunctionRefName(fn_decl)));
}

std::string Converter::ConvertFnPtrCallee(clang::Expr *arg) {
  PushExprKind push(*this, ExprKind::Callee);
  Buffer buf(*this);
  Convert(arg);
  return std::move(buf).str();
}

std::string Converter::ConvertFnPtrPlaceholder(clang::Expr *arg) {
  auto proto =
      arg->getType()->getPointeeType()->getAs<clang::FunctionProtoType>();
  return std::format("({} as {} {})", ConvertFnPtrCallee(arg), keyword_unsafe_,
                     ConvertFunctionPointerType(proto));
}

Converter::CallInfo Converter::CollectCallInfo(clang::CallExpr *expr) {
  using Kind = CallArg::Kind;

  CallInfo info;
  info.expr = expr;
  auto callee = GetCallee(expr);
  unsigned arg_begin = 0;
  if (auto op_call = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (auto method = clang::dyn_cast_or_null<clang::CXXMethodDecl>(
            op_call->getDirectCallee());
        method && !method->isExplicitObjectMemberFunction()) {
      arg_begin = 1;
    }
  }

  auto decl = expr->getCalleeDecl();
  const auto *function = decl ? decl->getAsFunction() : nullptr;
  const clang::FunctionProtoType *proto = nullptr;
  if (!function) {
    auto callee_ty = callee->getType().getDesugaredType(ctx_);
    if (auto ptr_ty = callee_ty->getAs<clang::PointerType>()) {
      proto = ptr_ty->getPointeeType()->getAs<clang::FunctionProtoType>();
    }
  }
  if (!function && !proto) {
    llvm::report_fatal_error(
        "Either function decl or function prototype should be known");
  }

  unsigned num_args = expr->getNumArgs() - arg_begin;
  unsigned num_named_params =
      function ? function->getNumParams() : proto->getNumParams();
  info.is_variadic = function ? function->isVariadic() : proto->isVariadic();
  info.is_fn_ptr_call =
      !function || AsLambdaClass(callee->getType()) ||
      clang::isa<clang::SubstNonTypeTemplateParmExpr>(
          clang::IgnoreExprNodes(callee, clang::IgnoreParensSingleStep,
                                 clang::IgnoreImplicitCastsSingleStep));
  info.is_libc_passthrough = RuleRegistry::IsLibcPassthrough(ctx_, expr);

  for (unsigned i = 0; i < num_named_params && i < num_args; ++i) {
    auto *arg = expr->getArg(i + arg_begin);
    CallArg ca{
        .param_name =
            function && !function->getParamDecl(i)->getName().empty()
                ? ("_" + GetNamedDeclAsString(function->getParamDecl(i)))
                : ("_arg" + std::to_string(i)),
        .param_type = function ? function->getParamDecl(i)->getType()
                               : proto->getParamType(i),
        .expr = arg,
        .has_default =
            function && HasUsableDefaultArg(function->getParamDecl(i)),
        .kind = (IsLiteral(arg) || info.is_libc_passthrough) ? Kind::Inline
                                                             : Kind::Hoisted,
    };
    bool is_materialize = clang::isa<clang::MaterializeTemporaryExpr>(arg);
    if (is_materialize && ca.param_type->isReferenceType()) {
      ca.kind = Kind::Materialized;
    } else if (is_materialize) {
      ca.kind = Kind::Inline;
    }
    info.args.push_back(std::move(ca));
  }

  if (info.is_variadic) {
    for (unsigned i = num_named_params; i < num_args; ++i) {
      info.variadic_args.push_back(expr->getArg(i + arg_begin));
    }
  }

  // Inline arguments that don't alias
  clang::Expr *receiver = GetCallObject(expr);
  for (auto &ca : info.args) {
    if (ca.kind != Kind::Hoisted) {
      continue;
    }
    bool aliases = receiver && ArgsMayAlias(ca.expr, receiver);
    for (const auto &other : info.args) {
      if (&other != &ca && ArgsMayAlias(ca.expr, other.expr)) {
        aliases = true;
        break;
      }
    }
    if (!aliases) {
      ca.kind = Kind::Inline;
    }
  }

  return info;
}

void Converter::ConvertParamTy(clang::QualType param_type, clang::Expr *expr) {
  {
    PushExprKind push(*this, ExprKind::AddrOf, param_type->isReferenceType());
    ConvertVarInit(param_type, expr);
  }
  ConvertParamTyConstCast(param_type, expr);
}

void Converter::ConvertFunctionPointerTransmute(clang::Expr *expr,
                                                clang::QualType type) {
  StrCat("std::mem::transmute::<");
  Convert(expr->getType());
  StrCat(',');
  Convert(type);
  StrCat(">(");
  Convert(expr);
  StrCat(')');
}

void Converter::ConvertParamTyConstCast(clang::QualType param_type,
                                        clang::Expr *expr) {
  if (!param_type->isPointerType() || !expr->getType()->isPointerType() ||
      IsVaListType(param_type) || IsVaListType(expr->getType())) {
    return;
  }
  if (auto *suffix = GetConstCastSuffix(param_type->getPointeeType(),
                                        expr->getType()->getPointeeType())) {
    StrCat(suffix);
  }
}

void Converter::EmitHoistedArgs(CallInfo &info) {
  using Kind = CallArg::Kind;
  for (auto &ca : info.args) {
    switch (ca.kind) {
    case Kind::Hoisted:
      StrCat(
          std::format("let {}: {} =", ca.param_name, ToString(ca.param_type)));
      ConvertParamTy(ca.param_type, ca.expr);
      StrCat(";");
      break;
    case Kind::Materialized: {
      auto [binding, ref] =
          MaterializeTemp(ca.param_name, ca.param_type, ca.expr);
      StrCat(binding);
      ca.ref_temp_name = std::move(ref);
      break;
    }
    case Kind::Inline:
      break;
    }
  }
}

void Converter::EmitArgList(const CallInfo &info) {
  using Kind = CallArg::Kind;
  PushParen call_args(*this);

  if (!ufcs_receiver_.empty()) {
    StrCat(std::exchange(ufcs_receiver_, std::string()), token::kComma);
  }

  for (unsigned i = 0; i < info.args.size(); i++) {
    const auto &ca = info.args[i];

    if (ca.has_default && clang::isa<clang::CXXDefaultArgExpr>(ca.expr)) {
      StrCat("None", token::kComma);
      continue;
    }

    if (ca.has_default) {
      StrCat("Some");
    }

    {
      PushParen push(*this, ca.has_default);
      switch (ca.kind) {
      case Kind::Hoisted:
        StrCat(ca.param_name);
        break;
      case Kind::Materialized:
        StrCat(ca.ref_temp_name);
        break;
      case Kind::Inline:
        ConvertParamTy(ca.param_type, ca.expr);
        if (info.is_libc_passthrough) {
          StrCat(
              std::format("as {}", Mapper::GetParamType(*this, info.expr, i)));
        }
        break;
      }
    }

    StrCat(token::kComma);
  }

  if (info.is_variadic) {
    if (!info.is_libc_passthrough) {
      StrCat("Box::new");
    }
    PushParen box(*this, !info.is_libc_passthrough);
    PushBracket push(*this, !info.is_libc_passthrough);
    EmitVariadicArgs(info.variadic_args, !info.is_libc_passthrough);
  }
}

void Converter::EmitVariadicArgs(llvm::ArrayRef<clang::Expr *> args,
                                 bool into_va_arg) {
  for (auto *arg : args) {
    {
      PushParen p(*this);
      ConvertVariadicArg(arg);
    }
    if (into_va_arg) {
      StrCat(".into()");
    }
    StrCat(token::kComma);
  }
}

void Converter::EmitCall(CallInfo &&info) {
  EmitHoistedArgs(info);

  if (info.is_fn_ptr_call) {
    EmitFnPtrCall(info.expr);
  } else if (info.is_libc_passthrough) {
    auto *direct_callee = info.expr->getDirectCallee();
    assert(direct_callee);
    StrCat("libc::", direct_callee->getName());
  } else {
    PushExprKind push(*this, ExprKind::Callee);
    Convert(GetCallee(info.expr));
  }

  EmitArgList(info);
}

void Converter::ConvertGenericCallExpr(clang::CallExpr *expr) {
  PushParen outer(*this);
  StrCat(keyword_unsafe_);
  PushBrace unsafe_brace(*this);
  EmitCall(CollectCallInfo(expr));
}

std::string Converter::GetUFCSName(const clang::CXXMethodDecl *method) const {
  return GetRecordName(method->getParent());
}

void Converter::ConvertUserOperatorCall(clang::CXXOperatorCallExpr *expr) {
  auto *callee = expr->getDirectCallee();
  PushParen outer(*this);
  StrCat(keyword_unsafe_);
  PushBrace unsafe_brace(*this);
  auto info = CollectCallInfo(expr);
  EmitHoistedArgs(info);
  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(callee)) {
    if (method->isInstance()) {
      SetUFCSReceiver(expr->getArg(0), false, method);
    }
    StrCat(GetUFCSName(method), token::kDoubleColon, GetMethodName(method));
  } else {
    StrCat(GetNamedDeclAsString(callee->getCanonicalDecl()));
  }
  EmitArgList(info);
}

std::optional<Converter::TempMaterializationCtx>
Converter::ConvertCallExpr(clang::CallExpr *expr) {
  auto *callee = expr->getCallee();

  if (IsBuiltinConstantP(callee)) {
    clang::APValue V;
    StrCat(expr->getArg(0)->isCXX11ConstantExpr(ctx_
#if CLANG_VERSION_MAJOR >= 24
                                                ,
                                                V
#endif
                                                )
               ? token::kOne
               : token::kZero);
  } else if (Mapper::Contains(ctx_, expr) &&
             !RuleRegistry::IsLibcPassthrough(ctx_, expr)) {
    auto **args = expr->getArgs();
    auto num_args = expr->getNumArgs();
    auto ctx = CollectRefBindingTempArgs(expr);
    StrCat(GetMappedAsString(expr, args, num_args, &ctx));
    return ctx;
  } else if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    // Calls to other operators are converted by VisitCallExpr.
    assert(IsUserOperatorCall(opcall));
    ConvertUserOperatorCall(opcall);
  } else {
    ConvertGenericCallExpr(expr);
  }
  return std::nullopt;
}

static std::string getTypedLiteral(const char *num, std::string_view type) {
  if (type.contains("::")) {
    // Not a builtin type
    return std::format("({} as {})", num, type);
  }
  return std::format("{}_{}", num, type);
}

std::string Converter::getIntegerLiteral(clang::IntegerLiteral *expr,
                                         bool incl_type,
                                         const clang::QualType *type) {
  auto value = GetIntegerLiteralValue(ctx_, expr, type, CharRustTypeIsSigned());
  llvm::SmallString<16> num_as_string;
  value.toString(num_as_string, 10);
  if (num_as_string[0] != '-' && !incl_type) {
    if (type && (*type)->isFloatingType() &&
        num_as_string.find('.') == llvm::StringRef::npos) {
      num_as_string += ".0";
    }
    return std::string(num_as_string);
  }

  auto ty = type ? *type : expr->getType();
  auto type_as_string = GetUnsafeTypeAsString(ty);

  if (ty->isFloatingType() || incl_type) {
    return getTypedLiteral(num_as_string.c_str(), type_as_string);
  }

  return static_cast<std::string>(num_as_string);
}

bool Converter::VisitIntegerLiteral(clang::IntegerLiteral *expr) {
  if (auto str = GetMappedAsString(expr); !str.empty()) {
    EmitFresh(str);
    return false;
  }
  EmitFresh(
      getIntegerLiteral(expr, Mapper::Map(*this, expr->getType()) != "i32"));
  return false;
}

bool Converter::VisitFloatingLiteral(clang::FloatingLiteral *expr) {
  EmitFresh(GetNumAsString(expr->getValue()));
  return false;
}

bool Converter::VisitCharacterLiteral(clang::CharacterLiteral *expr) {
  std::string ch;
  if (expr->getKind() != clang::CharacterLiteralKind::Ascii) {
    ch = std::to_string(expr->getValue());
  } else {
    auto uc = static_cast<unsigned char>(expr->getValue());
    ch = (uc > 0x7F ? "b'" : "'") + GetEscapedCharLiteral(expr->getValue()) +
         '\'';
  }
  {
    PushParen paren(*this);
    StrCat(ch, keyword::kAs, ToStringBase(expr->getType()));
  }
  MarkFresh();
  return false;
}

std::string Converter::GetEscapedUTF8CharLiteral(clang::Expr *expr) const {
  auto char_expr =
      clang::dyn_cast<clang::CharacterLiteral>(expr->IgnoreCasts());
  if (!char_expr) {
    return {};
  }
  std::string ch = GetEscapedCharLiteral(char_expr->getValue());
  auto start = reinterpret_cast<const llvm::UTF8 *>(ch.data());
  auto end = reinterpret_cast<const llvm::UTF8 *>(start + ch.size());
  return llvm::isLegalUTF8String(&start, end) ? std::move(ch) : "";
}

std::string Converter::GetEscapedStringLiteral(clang::Expr *expr,
                                               uint64_t pad_nulls) const {
  auto str_expr = clang::dyn_cast<clang::StringLiteral>(expr->IgnoreCasts());
  assert(str_expr);
  auto raw = str_expr->getString();
  std::string out;
  out.push_back('"');
  for (unsigned char c : raw) {
    out += GetEscapedCharLiteral(static_cast<char>(c));
  }
  for (uint64_t i = 0; i < pad_nulls; ++i) {
    out += "\\0";
  }
  out.push_back('"');
  return out;
}

bool Converter::IsArrayInitContext() const {
  return !curr_init_type_.empty() && curr_init_type_.back()->isArrayType();
}

std::string
Converter::GetCodeUnitArrayLiteral(const clang::StringLiteral *expr) {
  auto elem_type =
      ToStringBase(ctx_.getAsArrayType(expr->getType())->getElementType());
  uint64_t len = expr->getLength();
  uint64_t total = len + 1;
  if (IsArrayInitContext()) {
    if (auto *arr_ty = ctx_.getAsConstantArrayType(curr_init_type_.back())) {
      total = std::max(arr_ty->getSize().getZExtValue(), len);
    }
  }
  std::string out = "[";
  for (uint64_t i = 0; i < total; ++i) {
    out += std::format("{} as {}, ", i < len ? expr->getCodeUnit(i) : 0,
                       elem_type);
  }
  out += ']';
  return out;
}

std::string Converter::GetSourceFileAsString(clang::QualType type) {
  return std::format("concat!(file!(), \"\\0\").as_ptr() as {}",
                     ToString(type));
}

bool Converter::VisitSourceLocExpr(clang::SourceLocExpr *expr) {
  switch (expr->getIdentKind()) {
  case clang::SourceLocIdentKind::Line:
    EmitFresh(std::format("line!() as {}", ToString(expr->getType())));
    return false;
  case clang::SourceLocIdentKind::File:
    EmitFresh(GetSourceFileAsString(expr->getType()));
    return false;
  case clang::SourceLocIdentKind::Function: {
    auto value = expr->EvaluateInContext(ctx_, nullptr);
    auto *str = clang::cast<clang::StringLiteral>(
        value.getLValueBase().get<const clang::Expr *>());
    Convert(clang::ImplicitCastExpr::Create(
        ctx_, expr->getType(), clang::CK_ArrayToPointerDecay,
        const_cast<clang::StringLiteral *>(str), nullptr, clang::VK_PRValue,
        clang::FPOptionsOverride()));
    return false;
  }
  default:
    assert(0 && "unsupported SourceLocExpr");
    return false;
  }
}

bool Converter::VisitStringLiteral(clang::StringLiteral *expr) {
  if (IsCodeUnitStringLiteral(expr)) {
    EmitFresh(GetCodeUnitArrayLiteral(expr));
    return false;
  }

  auto init_type = curr_init_type_.empty()
                       ? clang::QualType()
                       : curr_init_type_.back().getNonReferenceType();
  if (!init_type.isNull() && init_type->isArrayType()) {
    if (auto *arr_ty = ctx_.getAsConstantArrayType(init_type)) {
      uint64_t arr_size = arr_ty->getSize().getZExtValue();
      if (expr->getString().empty()) {
        StrCat(std::format("[0 as libc::c_char; {}]", arr_size));
        MarkFresh();
        return false;
      }
      uint64_t pad = arr_size > expr->getString().size()
                         ? arr_size - expr->getString().size()
                         : 0;
      EmitFresh(std::format("std::mem::transmute(*b{})",
                            GetEscapedStringLiteral(expr, pad)));
      return false;
    }
    EmitFresh(std::format("std::mem::transmute(*b{})",
                          GetEscapedStringLiteral(expr, 1)));
    return false;
  }
  if (expr->getString().contains('\0')) {
    std::string out = "(&[";
    for (char c : expr->getString()) {
      auto value = CharRustTypeIsSigned()
                       ? std::to_string(static_cast<signed char>(c))
                       : std::to_string(static_cast<unsigned char>(c));
      out += getTypedLiteral(value.c_str(), CharRustType()) + ", ";
    }
    out += getTypedLiteral("0", CharRustType()) + "])";
    EmitFresh(out);
    return false;
  }
  EmitFresh(std::format("c{}", GetEscapedStringLiteral(expr, 0)));
  return false;
}

bool Converter::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr *expr) {
  EmitBoolLiteral(expr->getValue());
  return false;
}

void Converter::EmitBoolLiteral(bool value) {
  EmitFresh(value ? keyword::kTrue : keyword::kFalse);
}

bool Converter::VisitCXXNoexceptExpr(clang::CXXNoexceptExpr *expr) {
  EmitBoolLiteral(expr->getValue());
  return false;
}

void Converter::ConvertIntegerToEnumeralCast(clang::Expr *to,
                                             clang::Expr *from) {
  // Short circuit `(X as i32) as Enum` to `X`
  if (auto ref =
          clang::dyn_cast<clang::DeclRefExpr>(from->IgnoreParenImpCasts())) {
    if (auto ec = clang::dyn_cast<clang::EnumConstantDecl>(ref->getDecl())) {
      auto src_enum = clang::dyn_cast<clang::EnumDecl>(ec->getDeclContext());
      auto dst_enum = to->getType()->getAs<clang::EnumType>();
      if (src_enum && dst_enum && dst_enum->getDecl() == src_enum) {
        EmitFresh(EnumeratorName(ec));
        return;
      }
    }
  }
  PushParen paren(*this);
  {
    PushParen inner(*this);
    Convert(from);
  }
  StrCat(keyword::kAs, GetUnsafeTypeAsString(to->getType()));
  MarkFresh();
}

void Converter::ConvertIntegralToBooleanCast(clang::ImplicitCastExpr *expr) {
  auto sub_expr = expr->getSubExpr();
  auto *stripped = sub_expr->IgnoreParenImpCasts();

  if (auto *enum_type = sub_expr->getType()->getAs<clang::EnumType>();
      enum_type && enum_type->getDecl()->getIntegerType()->isBooleanType()) {
    Convert(sub_expr);
    return;
  }

  if (auto binop = clang::dyn_cast<clang::BinaryOperator>(stripped)) {
    // Comparisons and logical ops already produces bool, no wrap needed.
    if ((binop->isComparisonOp() || binop->isLogicalOp()) &&
        binop->getType()->isBooleanType()) {
      Convert(sub_expr);
      return;
    }
  }

  PushParen paren(*this);
  Convert(sub_expr);
  StrCat(token::kDiff);
  EmitFresh(sub_expr->getType()->isFloatingType() ? "0.0" : "0");
}

bool Converter::IsCastRedundantInRust(clang::Expr *expr,
                                      clang::QualType target_type) {
  auto target = GetUnsafeTypeAsString(target_type);
  if (const auto *rule = RuleRegistry::GetExprRule(ctx_, expr)) {
    return rule->return_type.type == target;
  }
  return GetUnsafeTypeAsString(expr->getType()) == target;
}

bool Converter::VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
  auto *sub_expr = expr->getSubExpr();
  auto type = expr->getType();
  switch (expr->getCastKind()) {
  case clang::CastKind::CK_LValueToRValue: {
    PushExprKind push(*this, ExprKind::RValue);
    Convert(sub_expr);
    // The value may have been copied out already.
    if (!isFresh()) {
      SetValueFreshness(type);
    }
    break;
  }
  case clang::CastKind::CK_ArrayToPointerDecay: {
    // __va_list_tag [1] decays to __va_list_tag *. Just pass through by value
    if (IsVaListType(sub_expr->getType())) {
      Convert(sub_expr);
      break;
    }
    bool dest_pointee_const =
        expr->getType()->getPointeeType().isConstQualified();
    {
      PushExprKind push(*this, ExprKind::LValue);
      Convert(sub_expr);
    }
    if (IsStringLiteralExpr(sub_expr)) {
      StrCat(".as_ptr()");
      if (!dest_pointee_const) {
        StrCat(".cast_mut()");
      }
    } else {
      StrCat(dest_pointee_const ? ".as_ptr()" : ".as_mut_ptr()");
    }
    MarkFresh();
    break;
  }
  case clang::CastKind::CK_BitCast: {
    PushParen paren(*this);
    Convert(sub_expr);
    ConvertCast(type);
    MarkFresh();
    break;
  }
  case clang::CastKind::CK_NoOp: {
    const char *suffix = nullptr;
    bool type_changed = false;
    if (expr->getType()->isPointerType() &&
        sub_expr->getType()->isPointerType()) {
      suffix = GetConstCastSuffix(expr->getType()->getPointeeType(),
                                  sub_expr->getType()->getPointeeType());
      type_changed = !suffix && !IsCastRedundantInRust(sub_expr, type);
    }
    if (type_changed) {
      PushParen paren(*this);
      Convert(sub_expr);
      ConvertCast(type);
      MarkFresh();
    } else {
      {
        PushParen paren(*this, suffix);
        Convert(sub_expr);
      }
      if (suffix) {
        EmitFresh(suffix);
      }
    }
    break;
  }
  case clang::CastKind::CK_FunctionToPointerDecay:
  case clang::CastKind::CK_BuiltinFnToFnPtr: {
    PushExprKind push(*this, ExprKind::AddrOf, !isCallee());
    Convert(sub_expr);
    break;
  }
  case clang::CastKind::CK_ConstructorConversion:
  case clang::CastKind::CK_DerivedToBase:
    Convert(sub_expr);
    break;
  case clang::CastKind::CK_IntegralToBoolean:
  case clang::CastKind::CK_FloatingToBoolean:
    ConvertIntegralToBooleanCast(expr);
    break;
  case clang::CastKind::CK_PointerToBoolean:
    StrCat(token::kNot);
    ConvertEqualsNullPtr(sub_expr);
    break;
  case clang::CastKind::CK_NullToPointer:
    EmitFresh(GetDefaultAsString(type));
    break;
  default:
    if (auto *literal = clang::dyn_cast<clang::IntegerLiteral>(sub_expr)) {
      auto type = expr->getType();
      EmitFresh(getIntegerLiteral(literal, true, &type));
      break;
    }
    // Skip cast if source and target map to the same Rust type.
    if (IsCastRedundantInRust(sub_expr, type)) {
      Convert(sub_expr);
      break;
    }
    if (type->isEnumeralType() && !sub_expr->getType()->isEnumeralType()) {
      ConvertIntegerToEnumeralCast(expr, sub_expr);
      break;
    }
    {
      PushParen outer(*this);
      {
        // as binds tighter than binary operators.
        PushParen inner(*this, clang::isa<clang::BinaryOperator>(sub_expr));
        Convert(sub_expr);
      }
      ConvertCast(type);
    }
    MarkFresh();
  }
  return false;
}

bool Converter::VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
  auto type = expr->getTypeAsWritten();
  auto *sub_expr = expr->getSubExpr();
  if (type->isVoidType()) {
    StrCat(token::kRef);
    PushParen paren(*this);
    PushExprKind push(*this, ExprKind::Void);
    Convert(expr->getSubExpr());
    return false;
  }
  // A cast to a reference type only rebinds the operand, it converts nothing
  if (type->isReferenceType()) {
    const char *suffix = isAddrOf()
                             ? GetConstCastSuffix(type.getNonReferenceType(),
                                                  sub_expr->getType())
                             : nullptr;
    {
      PushParen paren(*this, suffix);
      Convert(sub_expr);
    }
    if (suffix) {
      EmitFresh(suffix);
    }
    return false;
  }
  if (type->isEnumeralType() && !sub_expr->getType()->isEnumeralType()) {
    ConvertIntegerToEnumeralCast(expr, sub_expr);
    return false;
  }
  switch (expr->getStmtClass()) {
  case clang::Stmt::CXXFunctionalCastExprClass:
    Convert(sub_expr, type);
    return false;
  case clang::Stmt::CXXReinterpretCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass:
  case clang::Stmt::CStyleCastExprClass:
    if (expr->getType() == sub_expr->getType() ||
        (expr->getCastKind() == clang::CK_NoOp && type->isRecordType())) {
      return Convert(sub_expr);
    }
    if (type->isFunctionPointerType() ||
        sub_expr->getType()->isFunctionPointerType()) {
      ConvertFunctionPointerTransmute(sub_expr, type);
      return false;
    }
    if (type->isBooleanType() && sub_expr->getType()->isIntegerType() &&
        !sub_expr->getType()->isBooleanType()) {
      PushParen paren(*this);
      Convert(sub_expr);
      StrCat(token::kDiff, token::kZero);
      return false;
    }
    {
      PushParen paren(*this);
      Convert(sub_expr);
      if (auto *unary_oper = clang::dyn_cast<clang::UnaryOperator>(sub_expr);
          unary_oper && unary_oper->getOpcode() == clang::UO_AddrOf &&
          (clang::isa<clang::ArraySubscriptExpr>(unary_oper->getSubExpr()) ||
           clang::isa<clang::CXXOperatorCallExpr>(unary_oper->getSubExpr()))) {
        ConvertCast(sub_expr->getType());
      }
      ConvertCast(type);
    }
    return false;
  default:
    Convert(sub_expr);
    return false;
  }
}

bool Converter::VisitCXXRewrittenBinaryOperator(
    clang::CXXRewrittenBinaryOperator *expr) {
  Convert(expr->getSemanticForm());
  return false;
}

bool Converter::VisitBinaryOperator(clang::BinaryOperator *expr) {
  if (expr->getOpcode() == clang::BO_Cmp) {
    EmitFresh(std::format("std::cmp::Ord::cmp(&({}), &({}))",
                          ConvertRValue(expr->getLHS()),
                          ConvertRValue(expr->getRHS())));
    return false;
  }
  bool needs_cast = (expr->isComparisonOp() || expr->isLogicalOp()) &&
                    expr->getType()->isIntegerType() &&
                    !expr->getType()->isBooleanType();
  PushParen outer(*this, needs_cast);
  {
    PushParen inner(*this, needs_cast);
    ConvertBinaryOperator(expr);
  }
  if (needs_cast) {
    ConvertCast(expr->getType());
  }
  return false;
}

void Converter::ConvertBinaryOperator(clang::BinaryOperator *expr) {
  auto type = expr->getType();
  auto *lhs = expr->getLHS();
  auto *rhs = expr->getRHS();
  auto lhs_type = lhs->getType();
  auto rhs_type = rhs->getType();
  std::string_view opcode_as_string = expr->getOpcodeStr();

  if (auto *cmpd_assign_op =
          llvm::dyn_cast<clang::CompoundAssignOperator>(expr);
      expr->isCompoundAssignmentOp() &&
      GetUnsafeTypeAsString(lhs_type) !=
          GetUnsafeTypeAsString(cmpd_assign_op->getComputationResultType())) {
    Convert(lhs);
    StrCat(token::kAssign, ConvertCompoundAssignValue(cmpd_assign_op));
  } else if (expr->isCommaOp()) {
    PushBrace brace(*this);
    {
      PushExprKind push(*this, ExprKind::Void);
      Convert(lhs);
    }
    StrCat(token::kSemiColon);
    if (isRValue()) {
      StrCat(ConvertFreshRValue(rhs));
    } else {
      Convert(rhs);
    }
  } else if (IsUnsignedArithOp(expr)) {
    if (expr->isCompoundAssignmentOp()) {
      Convert(lhs);
      StrCat(token::kAssign);
    }
    {
      PushParen paren(*this);
      ConvertUnsignedArithOperand(lhs, type);
    }
    ConvertUnsignedArithBinaryOperator(expr, rhs);
    if (!expr->isCompoundAssignmentOp()) {
      MarkFresh();
    }
  } else if (expr->isAssignmentOp()) {
    if (expr->isCompoundAssignmentOp() &&
        expr->getLHS()->getType()->isPointerType() &&
        expr->getRHS()->getType()->isIntegralOrEnumerationType()) {
      PushBrace brace(*this, !isVoid());
      Convert(lhs);
      StrCat(token::kAssign);
      {
        PushParen paren(*this);
        ConvertUnsignedArithOperand(lhs, type);
      }
      ConvertUnsignedArithBinaryOperator(expr, rhs);
      if (!isVoid()) {
        StrCat(token::kSemiColon, ConvertRValue(lhs));
      }
    } else {
      ConvertAssignment(lhs, rhs, opcode_as_string);
    }
  } else if (IsComparisonWithNullOp(expr)) {
    if (expr->getOpcode() == clang::BO_EQ) {
      ConvertEqualsNullPtr(lhs);
    } else {
      StrCat(token::kNot);
      PushParen paren(*this);
      ConvertEqualsNullPtr(lhs);
    }
  } else if (expr->isAdditiveOp() && expr->getType()->isPointerType()) {
    auto [base, idx] = lhs_type->isPointerType() ? std::make_tuple(lhs, rhs)
                                                 : std::make_tuple(rhs, lhs);
    ConvertPointerOffset(base, idx, expr->getOpcode() == clang::BO_Add);
  } else if (expr->isAdditiveOp() && lhs_type->isPointerType() &&
             rhs_type->isPointerType()) {
    {
      PushParen outer(*this);
      {
        PushParen inner(*this);
        Convert(lhs);
        StrCat(keyword::kAs, "usize", token::kMinus);
        Convert(rhs);
        StrCat(keyword::kAs, "usize");
      }
      StrCat(token::kDiv);
      auto pointee_type_as_string = ConvertPointeeType(lhs_type);
      auto size_of_as_string =
          std::format("::std::mem::size_of::<{}>()", pointee_type_as_string);
      StrCat(size_of_as_string);
    }
    ConvertCast(expr->getType());
    MarkFresh();
  } else if (expr->isLogicalOp()) {
    {
      PushParen paren(*this);
      ConvertCondition(expr->getLHS());
    }
    StrCat(expr->getOpcodeStr());
    {
      PushParen paren(*this);
      ConvertCondition(expr->getRHS());
    }
    MarkFresh();
  } else {
    ConvertGenericBinaryOperator(expr);
  }
}

std::string
Converter::ConvertCompoundAssignValue(clang::CompoundAssignOperator *expr) {
  auto *lhs = expr->getLHS();
  auto computation_result_type = expr->getComputationResultType();
  Buffer buf(*this);
  {
    PushParen outer(*this);
    {
      PushParen inner(*this);
      StrCat(ConvertRValue(lhs));
      ConvertCast(computation_result_type);
    }
    if (IsUnsignedArithOp(expr)) {
      ConvertUnsignedArithBinaryOperator(expr, expr->getRHS());
    } else {
      std::string_view op = expr->getOpcodeStr();
      op.remove_suffix(1); // remove '=' from operator
      StrCat(op);
      Convert(expr->getRHS(), computation_result_type);
    }
  }
  if (lhs->getType()->isBooleanType()) {
    StrCat(token::kDiff, token::kZero);
  } else {
    ConvertCast(lhs->getType());
  }
  return std::move(buf).str();
}

void Converter::ConvertGenericBinaryOperator(clang::BinaryOperator *expr) {
  auto *lhs = expr->getLHS();
  auto *rhs = expr->getRHS();

  PushParen outer(*this);
  {
    PushParen lhs_paren(*this);
    Convert(lhs, GetOperandImplicitConversionTarget(ctx_, expr, lhs, rhs));
  }

  StrCat(expr->getOpcodeStr());

  {
    PushParen rhs_paren(*this);
    Convert(rhs, GetOperandImplicitConversionTarget(ctx_, expr, rhs, lhs));
  }
  MarkFresh();
}

bool Converter::IsReferenceType(const clang::Expr *expr) const {
  const auto *e = IgnoreTransparentStdCall(expr->IgnoreCasts())->IgnoreCasts();
  if (const auto *call = clang::dyn_cast<clang::CallExpr>(e)) {
    return !clang::isa<clang::CXXOperatorCallExpr>(call) &&
           GetReturnTypeOfFunction(call)->isReferenceType();
  }
  if (const auto *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(e)) {
    if (auto *var = clang::dyn_cast<clang::VarDecl>(decl_ref->getDecl());
        var && IsAliasOfInit(ctx_, var)) {
      return IsReferenceType(var->getInit());
    }
    return GetDeclRefType(curr_function_, decl_ref, decl_ref->getDecl())
        ->isReferenceType();
  }
  if (const auto *member = clang::dyn_cast<clang::MemberExpr>(e)) {
    return member->getMemberDecl()->getType()->isReferenceType();
  }
  return false;
}

bool Converter::ConvertIncAndDec(clang::UnaryOperator *expr) {
  const char *method = GetIncDecMethod(expr->getOpcode());
  if (!method) {
    return false;
  }
  {
    PushExprKind push(*this, ExprKind::LValue);
    Convert(expr->getSubExpr());
  }
  StrCat(std::format(".{}()", method));
  MarkFresh();
  return true;
}

bool Converter::VisitUnaryOperator(clang::UnaryOperator *expr) {
  if (auto str = GetMappedAsString(expr); !str.empty()) {
    EmitFresh(str);
    return false;
  }

  auto opcode = expr->getOpcode();
  auto *sub_expr = expr->getSubExpr();
  if (ConvertIncAndDec(expr)) {
    return false;
  }
  switch (opcode) {
  case clang::UO_Extension:
  case clang::UO_Plus:
    Convert(sub_expr);
    break;
  case clang::UO_AddrOf: {
    PushParen paren(*this);
    ConvertAddrOf(sub_expr, expr->getType());
    break;
  }
  case clang::UO_Deref:
    ConvertDeref(sub_expr);
    break;
  case clang::UO_Not:
    StrCat(token::kNot);
    Convert(sub_expr);
    MarkFresh();
    break;
  case clang::UO_LNot: {
    bool needs_int_cast =
        expr->getType()->isIntegerType() && !expr->getType()->isBooleanType();
    PushParen paren_cast(*this, needs_int_cast);
    StrCat(token::kNot);
    {
      PushParen paren_operand(*this);
      ConvertCondition(sub_expr);
    }
    if (needs_int_cast) {
      ConvertCast(expr->getType());
    }
    MarkFresh();
    break;
  }
  case clang::UO_Minus:
    if (auto *literal = clang::dyn_cast<clang::IntegerLiteral>(sub_expr)) {
      if (sub_expr->getType()->isUnsignedIntegerType()) {
        StrCat(std::format("(-{}_i{} as {})", getIntegerLiteral(literal, false),
                           ctx_.getTypeSize(expr->getType()),
                           GetUnsafeTypeAsString(expr->getType())));
      } else {
        StrCat(token::kMinus, getIntegerLiteral(literal, true));
      }
      MarkFresh();
      break;
    }
    if (expr->getType()->isUnsignedIntegerType()) {
      {
        PushParen paren(*this);
        Convert(sub_expr);
      }
      EmitFresh(".wrapping_neg()");
      break;
    }
    [[fallthrough]];
  default:
    StrCat(expr->getOpcodeStr(opcode));
    Convert(sub_expr);
    MarkFresh();
  }
  return false;
}

bool Converter::VisitStmtExpr(clang::StmtExpr *expr) {
  auto *body = expr->getSubStmt();
  PushBrace brace(*this);
  for (auto *s : body->body()) {
    // The last expression is the value of the block, so it has no semicolon.
    auto *tail = clang::dyn_cast<clang::Expr>(s);
    if (tail && s == body->body_back()) {
      Convert(tail);
    } else {
      Convert(s);
    }
  }
  MarkFresh();
  return false;
}

bool Converter::ConvertLValueConditional(clang::ConditionalOperator *expr) {
  if (!expr->isLValue() || expr->getType()->isFunctionType()) {
    return false;
  }
  // `&(c ? a : b)` -> `c ? &a : &b`
  if (isAddrOf()) {
    Convert(MakeConditionalAddrOf(ctx_, expr));
    return true;
  }
  // `(c ? a : b) = v` -> `*(c ? &a : &b) = v`
  if (isLValue()) {
    Convert(clang::UnaryOperator::Create(
        ctx_, MakeConditionalAddrOf(ctx_, expr), clang::UO_Deref,
        expr->getType(), clang::VK_LValue, clang::OK_Ordinary,
        expr->getExprLoc(), false, clang::FPOptionsOverride()));
    return true;
  }
  return false;
}

bool Converter::VisitConditionalOperator(clang::ConditionalOperator *expr) {
  if (ConvertLValueConditional(expr)) {
    return false;
  }
  auto branch = [&](clang::Expr *branch_expr) {
    PushBrace brace(*this);
    ConvertConditionalBranch(branch_expr, expr->getType());
    if (expr->getType()->isVoidType()) {
      StrCat(token::kSemiColon);
    }
  };
  StrCat(keyword::kIf);
  ConvertCondition(expr->getCond());
  branch(expr->getTrueExpr());
  StrCat(keyword::kElse);
  branch(expr->getFalseExpr());
  return false;
}

void Converter::ConvertConditionalBranch(clang::Expr *expr,
                                         clang::QualType type) {
  Convert(expr, type);
}

std::string Converter::ConvertDeclRef(clang::Expr *expr,
                                      clang::ValueDecl *decl) {
  if (isAddrOf()) {
    clang::Expr *addrof_op = ToAddrOf(ctx_, expr);
    if (auto str = GetMappedAsString(addrof_op); !str.empty()) {
      return str;
    }
  }

  if (ShouldReplaceWithMappedBody(decl)) {
    if (auto str = GetMappedAsString(expr); !str.empty()) {
      if (RuleRegistry::ReturnsReference(expr) && !isAddrOf() &&
          !Mapper::IsRefExpr(ctx_, expr)) {
        return std::format("({}{})", GetPointerDerefPrefix(expr->getType()),
                           str);
      }
      return str;
    }
  }

  if (auto *function = decl->getAsFunction()) {
    if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(function)) {
      if (method->isStatic()) {
        return GetFunctionRefName(method);
      }
    }
    return GetNamedDeclAsString(function->getCanonicalDecl());
  }

  if (auto enum_constant = clang::dyn_cast<clang::EnumConstantDecl>(decl)) {
    auto name = EnumeratorName(enum_constant);
    if (!expr->getType()->isEnumeralType()) {
      return std::format("({} as i32)", name);
    }
    return name;
  }

  if (auto *var = clang::dyn_cast<clang::VarDecl>(decl);
      var && IsGlobalVar(var) && LazyStaticInit()) {
    return std::format("(*std::cell::LazyCell::force_mut(&mut *&raw mut {}))",
                       GetNamedDeclAsString(decl));
  }

  if (auto it = range_iterator_names_.find(decl);
      it != range_iterator_names_.end()) {
    return it->second;
  }
  return GetNamedDeclAsString(decl);
}

bool Converter::VisitDeclRefExpr(clang::DeclRefExpr *expr) {
  if (auto param =
          clang::dyn_cast<clang::TemplateParamObjectDecl>(expr->getDecl());
      param && AsLambdaClass(param->getType())) {
    EmitFresh(GetDefaultAsString(param->getType()));
    return false;
  }

  if (auto init = AsLambdaUncapturedConstant(curr_function_, expr)) {
    Convert(init);
    return false;
  }

  if (auto *binding = clang::dyn_cast<clang::BindingDecl>(expr->getDecl())) {
    Convert(binding->getBinding());
    return false;
  }

  if (auto var = clang::dyn_cast<clang::VarDecl>(expr->getDecl());
      var && IsAliasOfInit(ctx_, var)) {
    Convert(var->getInit());
    return false;
  }
  ConvertDeclRefValue(expr, expr->getDecl());
  return false;
}

void Converter::ConvertDeclRefValue(clang::Expr *expr, clang::ValueDecl *decl) {
  auto str = ConvertDeclRef(expr, decl);
  auto type = GetDeclRefType(curr_function_, expr, decl);

  if (type->getAs<clang::ReferenceType>() && !isAddrOf()) {
    EmitDeref(std::move(str), type.getNonReferenceType());
    SetValueFreshness(expr->getType());
    return;
  }

  if (auto *fn_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (isAddrOf()) {
      ConvertFunctionToFunctionPointer(fn_decl);
      return;
    }
    EmitFresh(str);
    return;
  }

  if (!type->getAs<clang::ReferenceType>() && isAddrOf()) {
    StrCat(token::kRef, type.isConstQualified() ? "" : keyword_mut_, str);
    MarkFresh();
    return;
  }

  StrCat(str);
  SetValueFreshness(expr->getType());
}

bool Converter::VisitParenExpr(clang::ParenExpr *expr) {
  if (auto *bin = clang::dyn_cast<clang::BinaryOperator>(expr->getSubExpr());
      bin && (bin->isCommaOp() || (bin->isAssignmentOp() && isVoid()))) {
    Convert(expr->getSubExpr());
    return false;
  }

  {
    PushParen inner(*this);
    Convert(expr->getSubExpr());
  }

  return false;
}

bool Converter::ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) {
  switch (expr->getOperator()) {
  case clang::OverloadedOperatorKind::OO_Equal:
    ConvertAssignment(expr->getArg(0), expr->getArg(1), "=");
    break;
  case clang::OverloadedOperatorKind::OO_Star: {
    PushParen paren(*this);
    StrCat(token::kStar);
    Convert(expr->getArg(0));
    break;
  }
  case clang::OverloadedOperatorKind::OO_Arrow:
    Convert(expr->getArg(0));
    break;
  case clang::OverloadedOperatorKind::OO_Subscript: {
    auto *base = expr->getArg(0);
    auto *idx = expr->getArg(1);
    // Rust borrows the container before computing the index, so an index that
    // reads the container, or that has side effects through anything the
    // container is reached through, is computed beforehand.
    const clang::ValueDecl *container = nullptr;
    if (!idx->HasSideEffects(ctx_)) {
      auto *inner = base->IgnoreParenImpCasts();
      if (auto *ref = clang::dyn_cast<clang::DeclRefExpr>(inner)) {
        container = ref->getDecl();
      } else if (auto *member = clang::dyn_cast<clang::MemberExpr>(inner)) {
        container = member->getMemberDecl();
      }
    }
    if (container ? GetAllVars(idx).contains(container)
                  : ReferToCommonVar(idx, base)) {
      std::string idx_str;
      {
        ScopedValue no_autoref(autoref_mut_, std::nullopt);
        idx_str = ConvertRValue(idx);
      }
      PushParen paren(*this);
      StrCat(token::kStar);
      PushBrace brace(*this);
      StrCat("let __idx =", idx_str);
      if (Mapper::Map(*this, idx->getType()) != "usize") {
        StrCat(keyword::kAs, "usize");
      }
      StrCat(token::kSemiColon, IsMutatingCall(expr) ? "&mut" : "&");
      ScopedValue autoref(autoref_mut_, IsMutatingCall(expr));
      Convert(base->IgnoreImplicit());
      StrCat("[__idx]");
      break;
    }
    ScopedValue autoref(autoref_mut_, IsMutatingCall(expr));
    ConvertArraySubscript(base, idx, expr->getType());
    break;
  }
  case clang::OverloadedOperatorKind::OO_LessLess:
    if (IsCallToOstream(expr)) {
      ConvertCallToOstream(expr);
      return false;
    }
    break;
  case clang::OverloadedOperatorKind::OO_Call:
    ConvertGenericCallExpr(expr);
    break;
  case clang::OverloadedOperatorKind::OO_Less:
    if (auto callee = expr->getDirectCallee()) {
      if (clang::isa<clang::CXXMethodDecl>(callee)) {
        Convert(expr->getArg(0));
        if (callee->isUserProvided()) {
          StrCat(token::kDot, GetOverloadedOperator(callee));
          PushParen paren(*this);
          StrCat(ConvertPointer(expr->getArg(1)));
        } else {
          StrCat(token::kLt);
          Convert(expr->getArg(1));
        }
      } else {
        StrCat(GetOverloadedOperator(callee));
        PushParen paren(*this);
        StrCat(ConvertFreshPointer(expr->getArg(0)), token::kComma,
               ConvertFreshPointer(expr->getArg(1)));
      }
    }
    MarkFresh();
    break;
  default:
    // FIXME: improve error handling
    llvm::errs() << "unsupported CXXOperatorCallExpr: "
                 << clang::getOperatorSpelling(expr->getOperator()) << '\n';
    assert(0);
  }
  return false;
}

bool Converter::VisitMemberExpr(clang::MemberExpr *expr) {
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
  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && IsMethodOnPtr(method) && !Mapper::Contains(ctx_, expr)) {
    SetUFCSReceiver(expr->getBase(), expr->isArrow(), method);
    StrCat(GetRecordName(method->getParent()), token::kDoubleColon,
           GetMethodName(method));
    MarkFresh();
    return false;
  }
  std::string str;
  {
    Buffer buf(*this);
    Converter::ConvertMemberExpr(expr);
    str = std::move(buf).str();
  }

  if (auto *field = clang::dyn_cast<clang::FieldDecl>(member);
      field && field->isMutable()) {
    auto *base = expr->getBase();
    auto object_type = expr->isArrow() ? base->getType()->getPointeeType()
                                       : base->getType().getNonReferenceType();
    if (!isRValue() && object_type.isConstQualified()) {
      if (isAddrOf()) {
        EmitFresh(std::format("(&raw const {}).cast_mut()", str));
        return false;
      }
      str = std::format("(*(&raw const {}).cast_mut())", str);
    }
  }

  if (isAddrOf()) {
    bool is_reference_type = member->getType()->isReferenceType();
    if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member)) {
      is_reference_type |= method->getReturnType()->isReferenceType();
    }

    if (is_reference_type || RuleRegistry::ReturnsPointer(ctx_, expr)) {
      MarkNotFresh();
    } else {
      StrCat(token::kRef,
             expr->getType().isConstQualified() ? "" : keyword_mut_);
      MarkFresh();
    }
    StrCat(str);
    return false;
  }

  if (member->getType()->isReferenceType()) {
    EmitDeref(std::move(str), member->getType().getNonReferenceType());
    return false;
  }

  if (member->getType()->isFunctionPointerType()) {
    PushParen paren(*this);
    StrCat(str);
    SetValueFreshness(expr->getType());
    return false;
  }

  StrCat(str);
  if (clang::isa<clang::CXXMethodDecl>(member)) {
    MarkFresh();
  } else {
    SetValueFreshness(expr->getType());
  }
  return false;
}

void Converter::SetUFCSReceiver(clang::Expr *base, bool is_arrow,
                                const clang::CXXMethodDecl *method) {
  if (clang::isa<clang::CXXThisExpr>(base->IgnoreParenImpCasts()) &&
      !AsLambdaOperatorCall(curr_function_)) {
    ufcs_receiver_ = InConstructor() ? "&mut this" : keyword::kSelfValue;
    return;
  }
  Buffer buf(*this);
  PushExprKind push(*this, ExprKind::LValue);
  StrCat(MethodNeedsMutableReceiver(method) ? "&mut" : "&");
  if (is_arrow) {
    ConvertArrow(base);
  } else {
    Convert(base);
  }
  ufcs_receiver_ = std::move(buf).str();
}

void Converter::ConvertMemberExpr(clang::MemberExpr *expr) {
  if (auto mapped = GetMappedAsString(expr); !mapped.empty()) {
    if (RuleRegistry::ReturnsPointer(ctx_, expr) && !isAddrOf() &&
        !Mapper::IsRefExpr(ctx_, expr)) {
      PushParen paren(*this);
      StrCat(token::kStar, mapped);
    } else {
      StrCat(mapped);
    }
    return;
  }

  // A field of the result of a call is read in the block of the temporaries
  // of the arguments of the call.
  if (auto *call = GetFieldReadCall(expr);
      call && (!field_read_call_ || field_read_call_->call != call)) {
    FieldReadCall read{call, ""};
    std::string str;
    {
      ScopedValue push_read(field_read_call_, &read);
      Buffer buf(*this);
      ConvertMemberExpr(expr);
      str = std::move(buf).str();
    }
    if (read.temporary_bindings.empty()) {
      StrCat(str);
    } else {
      StrCat(std::format("{{ {} {} }}", read.temporary_bindings, str));
    }
    return;
  }

  auto *member = expr->getMemberDecl();
  auto *base = expr->getBase();
  bool base_is_this = clang::isa<clang::CXXThisExpr>(base->IgnoreCasts()) &&
                      !ThisIsRustPtr() && !AsLambdaOperatorCall(curr_function_);
  PushExprKind push(*this, isLValue() ? ExprKind::LValue : ExprKind::RValue);
  if (base_is_this) {
    StrCat(InConstructor() ? "this" : keyword::kSelfValue);
  } else if (expr->isArrow()) {
    ConvertArrow(base);
  } else {
    Convert(base);
  }

  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && IsOverloadedMethod(method)) {
    StrCat(token::kDot);
    StrCat(GetMethodName(method));
  } else if (member->getDeclName().isIdentifier()) {
    StrCat(token::kDot);
    StrCat(GetNamedDeclAsString(member));
  }
}

bool Converter::DeferTemporaries(const clang::Expr *call,
                                 const TempMaterializationCtx *temps) {
  if (!temps || !field_read_call_ || field_read_call_->call != call) {
    return false;
  }
  field_read_call_->temporary_bindings = temps->temporary_bindings;
  return true;
}

// The call whose result expr reads a field of, through fields of fields.
const clang::Expr *Converter::GetFieldReadCall(const clang::MemberExpr *expr) {
  const clang::Expr *base = expr;
  while (auto *member = clang::dyn_cast<clang::MemberExpr>(base)) {
    if (member->isArrow()) {
      return nullptr;
    }
    base = member->getBase()->IgnoreParenImpCasts();
  }
  return clang::isa<clang::CallExpr>(base) ? base : nullptr;
}

bool Converter::VisitCXXThisExpr(clang::CXXThisExpr *expr) {
  if (auto field = AsLambdaCaptureThis(curr_function_)) {
    auto name = GetNamedDeclAsString(field);
    EmitFresh(field->getType()->isPointerType()
                  ? name
                  : std::format("(&raw mut {})", name));
    return false;
  }
  if (InConstructor()) {
    StrCat("&raw mut this");
  } else {
    PushParen paren(*this);
    StrCat(keyword::kSelfValue, keyword::kAs, ToString(expr->getType()));
  }
  MarkFresh();
  return false;
}

bool Converter::VisitOpaqueValueExpr(clang::OpaqueValueExpr *expr) {
  // The common expression of an array init loop uses the indices of the
  // loops outside it.
  auto outer = array_init_outer_depth_.find(expr);
  if (outer == array_init_outer_depth_.end()) {
    Convert(expr->getSourceExpr());
    return false;
  }
  auto depth = std::exchange(array_init_depth_, outer->second);
  Convert(expr->getSourceExpr());
  array_init_depth_ = depth;
  return false;
}

std::string Converter::ArrayInitIndexName() const {
  return array_init_depth_ == 1 ? "__i"
                                : std::format("__i{}", array_init_depth_);
}

bool Converter::VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr *expr) {
  EmitFresh(ArrayInitIndexName());
  return false;
}

bool Converter::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr *expr) {
  StrCat(std::format("std::array::from_fn::<_, {}, _>",
                     GetArraySize(expr->getType())));
  PushParen paren(*this);
  array_init_outer_depth_[expr->getCommonExpr()] = array_init_depth_++;
  StrCat(std::format("|{}: usize|", ArrayInitIndexName()));
  ConvertVarInit(expr->getSubExpr()->getType(), expr->getSubExpr());
  --array_init_depth_;
  MarkFresh();
  return false;
}

bool Converter::VisitInitListExpr(clang::InitListExpr *expr) {
  auto *syntactic = expr->isSyntacticForm() ? expr : expr->getSyntacticForm();
  if (auto form = expr->getSemanticForm())
    expr = form;

  if (expr->isTransparent()) {
    Convert(expr->getInit(0));
    return false;
  }

  auto qual_type = expr->getType();
  if (qual_type->isScalarType()) {
    ConvertScalarInitList(expr);
  } else if (qual_type->isRecordType()) {
    ConvertRecordInitList(expr, syntactic->getNumInits() == 0);
  } else if (IsInitExprOfStringLiteral(expr)) {
    Convert(expr->getInit(0)->IgnoreParenImpCasts());
  } else {
    ConvertArrayInitList(expr);
  }
  MarkFresh();
  return false;
}

void Converter::ConvertScalarInitList(clang::InitListExpr *expr) {
  assert(expr->getNumInits() < 2 && "Excess elements in scalar initializer");
  if (expr->getNumInits() > 0) {
    auto init = expr->getInit(0);
    ConvertVarInit(init->getType(), init);
  } else {
    StrCat(GetDefaultAsString(expr->getType()));
  }
}

void Converter::ConvertRecordInitList(clang::InitListExpr *expr,
                                      bool is_empty) {
  auto qual_type = expr->getType();
  if (is_empty) {
    StrCat(GetDefaultAsString(qual_type));
    return;
  }
  if (auto str = GetMappedAsString(expr, expr->getInits(), expr->getNumInits());
      !str.empty()) {
    StrCat(str);
    return;
  }

  const auto *record = qual_type->getAsRecordDecl();
  assert(IsUserDefinedDecl(record) && "aggregate list without a rule");
  if (record->isUnion()) {
    auto *field = expr->getInitializedFieldInUnion();
    assert(field && expr->getNumInits() == 1);
    ConvertUnionInitList(expr, field);
    return;
  }
  StrCat(GetUnsafeTypeAsString(qual_type));
  PushBrace brace(*this);
  unsigned i = 0;
  for (const auto *field : record->fields()) {
    StrCat(GetNamedDeclAsString(field), token::kColon);
    ConvertFieldInit(field,
                     i < expr->getNumInits() ? expr->getInit(i++) : nullptr);
    StrCat(token::kComma);
  }
}

void Converter::ConvertUnionInitList(clang::InitListExpr *expr,
                                     const clang::FieldDecl *field) {
  StrCat(GetUnsafeTypeAsString(expr->getType()));
  PushBrace brace(*this);
  StrCat(GetNamedDeclAsString(field), token::kColon);
  ConvertFieldInit(field, expr->getInit(0));
}

void Converter::ConvertArrayInitList(clang::InitListExpr *expr) {
  PushBracket bracket(*this);
  auto element_type = ctx_.getAsArrayType(expr->getType())->getElementType();
  for (auto *init : expr->inits()) {
    ConvertVarInit(element_type, init);
    StrCat(token::kComma);
  }
  if (expr->hasArrayFiller()) {
    if (auto arr_ty = ctx_.getAsConstantArrayType(expr->getType())) {
      assert((arr_ty->getSize().getZExtValue() - expr->getNumInits()) &&
             "Number of initializers should be less than total size of array");
      for (unsigned i = 0;
           i < arr_ty->getSize().getZExtValue() - expr->getNumInits(); ++i) {
        ConvertVarInit(expr->getArrayFiller()->getType(),
                       expr->getArrayFiller());
        StrCat(token::kComma);
      }
    }
  }
}

bool Converter::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr *expr) {
  auto record = expr->getType()->getAsRecordDecl();
  if (!record || !record->hasAttr<clang::TransparentUnionAttr>()) {
    Convert(expr->getInitializer());
    return false;
  }
  auto init = clang::cast<clang::InitListExpr>(expr->getInitializer());
  assert(init->getNumInits() == 1);
  PushExprKind push(*this, ExprKind::RValue);
  Convert(init->getInit(0));
  return false;
}

bool Converter::VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  if (base->IgnoreCasts()->getType()->isPointerType() ||
      clang::isa<clang::StringLiteral>(base->IgnoreCasts())) {
    ConvertPointerSubscript(expr);
  } else {
    ConvertArraySubscript(base, expr->getIdx(), expr->getType());
  }
  return false;
}

bool Converter::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr *expr) {
  EmitNullPointer();
  return false;
}

void Converter::EmitNullPointer() { EmitFresh(token::kDefault); }

bool Converter::VisitVAArgExpr(clang::VAArgExpr *expr) {
  auto va_list_expr = expr->getSubExpr();
  if (expr->getType()->isFunctionPointerType()) {
    StrCat("std::mem::transmute::<*mut ::libc::c_void", token::kComma);
    Convert(expr->getType());
    StrCat('>');
    PushParen paren(*this);
    {
      PushExprKind push(*this, ExprKind::RValue);
      Convert(va_list_expr);
    }
    EmitFresh(".arg::<*mut ::libc::c_void>()");
    return false;
  }
  Convert(va_list_expr);
  StrCat(".arg::<");
  Convert(expr->getType());
  EmitFresh(">()");
  return false;
}

bool Converter::VisitGNUNullExpr(clang::GNUNullExpr *expr) {
  EmitNullPointer();
  return false;
}

bool Converter::VisitCXXNewExpr(clang::CXXNewExpr *expr) {
  if (expr->isArray()) {
    if (auto *init = llvm::dyn_cast_or_null<clang::InitListExpr>(
            expr->getInitializer())) {
      StrCat("Box::leak(Box::new(");
      Convert(init);
      StrCat("))");
    } else {
      assert(expr->getArraySize().has_value());
      auto array_size_as_string = ToString(*expr->getArraySize());
      auto alloc_type_as_string = ToString(expr->getAllocatedType());
      auto default_alloc_type_as_string =
          GetDefaultAsString(expr->getAllocatedType());
      auto new_array_as_string =
          std::format("Box::leak((0..{}).map(|_| {}).collect::<Box<[{}]>>())",
                      array_size_as_string, default_alloc_type_as_string,
                      alloc_type_as_string);
      StrCat(new_array_as_string);
    }
    if (!curr_init_type_.empty() && curr_init_type_.back()->isPointerType()) {
      StrCat(".as_mut_ptr()");
    }
  } else {
    auto initializer_as_string =
        expr->getInitializer() ? ToString(expr->getInitializer())
                               : GetDefaultAsString(expr->getAllocatedType());
    auto new_as_string =
        std::format("(Box::leak(Box::new({})) as {})", initializer_as_string,
                    ToString(expr->getType()));
    StrCat(new_as_string);
  }
  MarkFresh();
  return false;
}

bool Converter::VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) {
  auto *argument = expr->getArgument();
  auto destroyed_type = expr->getDestroyedType();
  PushBrace brace(*this);
  StrCat(keyword::kLet, "__p", token::kAssign, ToString(argument),
         token::kSemiColon);
  StrCat("if !__p.is_null()");
  PushBrace not_null(*this);
  if (!TypeNeedsDestruction(destroyed_type)) {
    EmitDeallocation(expr, "__p");
    return false;
  }
  auto record_name = GetRecordName(destroyed_type->getAsCXXRecordDecl());
  if (expr->isArrayForm()) {
    StrCat(std::format("for __i in 0..libcc2rs::malloc_usable_size(__p as *mut "
                       "::libc::c_void) / ::std::mem::size_of::<{0}>() {{ "
                       "{0}::{1}(&mut *__p.add(__i)); }}",
                       record_name, kDestructorName));
  } else {
    StrCat(std::format("{}::{}(&mut *__p)", record_name, kDestructorName),
           token::kSemiColon);
  }
  EmitDeallocation(expr, "__p");
  return false;
}

void Converter::EmitDeallocation(clang::CXXDeleteExpr *expr,
                                 const std::string &argument_as_string) {
  if (expr->isArrayForm()) {
    auto destroyed_type = expr->getDestroyedType();
    auto type = ToString(destroyed_type);
    bool is_const = destroyed_type.isConstQualified();
    StrCat(std::format(
        R"(
        ::std::mem::drop(Box::from_raw(
          ::std::slice::from_raw_parts{0}({1},
            libcc2rs::malloc_usable_size({1} as *mut ::libc::c_void) /
            ::std::mem::size_of::<{2}>()){3})))",
        is_const ? "" : "_mut", argument_as_string, type,
        is_const ? std::format(" as *const [{0}] as *mut [{0}]", type) : ""));
  } else {
    bool cast_mut = !IsMut(expr->getDestroyedType());
    StrCat(std::format("::std::mem::drop(Box::from_raw({}{}))",
                       argument_as_string, cast_mut ? ".cast_mut()" : ""));
  }
}

void Converter::ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) {
  StrCat(std::format("std::array::from_fn::<_, {}, _>",
                     GetArraySize(expr->getType())));
  PushParen paren(*this);
  StrCat("|_|");
  ConvertCXXConstructExprArgs(expr);
}

void Converter::ConvertCXXConstructExprArgs(clang::CXXConstructExpr *expr) {
  HoistMaterializedTempBindings hoist_temps(*this, /*as_block=*/true);
  auto ctor = expr->getConstructor();
  StrCat(GetRecordName(ctor->getParent()), token::kDoubleColon,
         GetCtorName(ctor));
  PushParen paren(*this);

  unsigned arg_idx = 0;
  for (unsigned param_idx = 0; param_idx < ctor->getNumParams(); ++param_idx) {
    auto param = ctor->getParamDecl(param_idx);
    auto param_type = param->getType();
    bool has_default = HasUsableDefaultArg(param);

    if (arg_idx < expr->getNumArgs() &&
        clang::isa<clang::CXXDefaultArgExpr>(expr->getArg(arg_idx))) {
      assert(has_default);
      ++arg_idx;
      StrCat("None", token::kComma);
      continue;
    }

    if (arg_idx < expr->getNumArgs()) {
      clang::Expr *arg = expr->getArg(arg_idx++);
      PushBrace brace(*this);
      if (has_default) {
        StrCat("Some");
      }
      PushParen some(*this, has_default);
      ConvertVarInit(param_type, arg);
    } else {
      assert(has_default);
      StrCat("None");
    }
    StrCat(token::kComma);
  }
}

bool Converter::VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
  if (expr->isElidable()) {
    Convert(expr->getArg(0));
    return false;
  }

  if (auto str = GetMappedAsString(expr, expr->getArgs(), expr->getNumArgs());
      !str.empty()) {
    StrCat(str);
    if (!IsPassThroughRule(expr)) {
      MarkFresh();
    }
    return false;
  }

  auto *ctor = expr->getConstructor();
  if (IsLambdaCopyOrMoveConstructor(ctor)) {
    Convert(expr->getArg(0));
    EmitFresh(std::format(".{}()", GetCopyOrMoveName(ctor)));
    return false;
  }
  if (IsPassThroughConstructor(ctor)) {
    Convert(expr->getArg(0));
    if ((ctor->isCopyConstructor() || IsDefaultedMoveConstructor(ctor)) &&
        !TypeIsCopyable(expr->getType())) {
      EmitFresh(".clone()");
    }
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

bool Converter::VisitUnaryExprOrTypeTraitExpr(
    clang::UnaryExprOrTypeTraitExpr *expr) {
  const char *fn = nullptr;
  switch (expr->getKind()) {
  case clang::UnaryExprOrTypeTrait::UETT_SizeOf:
    fn = "size_of";
    break;
  case clang::UnaryExprOrTypeTrait::UETT_AlignOf:
  case clang::UnaryExprOrTypeTrait::UETT_PreferredAlignOf:
    fn = "align_of";
    break;
  default:
    // FIXME: improve error handling
    log() << "unsupported unary expr or type trait expr\n";
    return false;
  }
  EmitFresh(std::format("::std::mem::{}::<{}>()", fn,
                        GetUnsafeTypeAsString(expr->getTypeOfArgument())));
  return false;
}

bool Converter::VisitConceptSpecializationExpr(
    clang::ConceptSpecializationExpr *expr) {
  assert(!expr->isValueDependent());
  EmitBoolLiteral(expr->isSatisfied());
  return false;
}

bool Converter::VisitRequiresExpr(clang::RequiresExpr *expr) {
  assert(!expr->isValueDependent());
  EmitBoolLiteral(expr->isSatisfied());
  return false;
}

void Converter::EmitEvaluatedInt(clang::Expr *expr) {
  clang::Expr::EvalResult result;
  ENSURE(expr->EvaluateAsInt(result, ctx_));
  EmitFresh(std::to_string(result.Val.getInt().getExtValue()));
}

bool Converter::VisitTypeTraitExpr(clang::TypeTraitExpr *expr) {
  EmitEvaluatedInt(expr);
  return false;
}

bool Converter::VisitSizeOfPackExpr(clang::SizeOfPackExpr *expr) {
  EmitEvaluatedInt(expr);
  return false;
}

bool Converter::VisitOffsetOfExpr(clang::OffsetOfExpr *expr) {
  std::string member_path;
  for (unsigned i = 0; i < expr->getNumComponents(); ++i) {
    const clang::OffsetOfNode &node = expr->getComponent(i);
    ENSURE(node.getKind() == clang::OffsetOfNode::Field);
    if (!member_path.empty()) {
      member_path += '.';
    }
    member_path += GetNamedDeclAsString(node.getField());
  }
  EmitFresh(
      std::format("::std::mem::offset_of!({}, {})",
                  GetUnsafeTypeAsString(expr->getTypeSourceInfo()->getType()),
                  member_path));
  return false;
}

bool Converter::VisitEnumDecl(clang::EnumDecl *decl) {
  ENSURE(decl_ids_.insert(GetID(decl)).second);
  if (Mapper::Contains(ctx_, ctx_.getCanonicalTagType(decl))) {
    return false;
  }
  auto name = GetRecordName(decl);
  StrCat(std::format("pub type {} = {};", name,
                     GetUnsafeTypeAsString(decl->getIntegerType())));
  bool is_bool = decl->getIntegerType()->isBooleanType();
  for (auto e : decl->enumerators()) {
    llvm::SmallVector<char, 32> init;
    e->getInitVal().toString(init, 10);
    std::string value(init.data(), init.size());
    if (is_bool) {
      value = e->getInitVal().getBoolValue() ? "true" : "false";
    }
    StrCat(
        std::format("pub const {}: {} = {};", EnumeratorName(e), name, value));
  }
  return false;
}

std::string
Converter::EnumeratorName(const clang::EnumConstantDecl *decl) const {
  auto *enum_decl = clang::cast<clang::EnumDecl>(decl->getDeclContext());
  return std::format("{}_{}", GetRecordName(enum_decl),
                     std::string_view(decl->getName()));
}

bool Converter::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) {
  Convert(expr->getExpr());
  return false;
}

bool Converter::VisitConstantExpr(clang::ConstantExpr *expr) {
  Convert(expr->getSubExpr());
  MarkFresh();
  return false;
}

bool Converter::VisitLambdaExpr(clang::LambdaExpr *expr) {
  auto decl = expr->getLambdaClass();
  if (decl->captures().empty() && !decl->isGenericLambda()) {
    ConvertCapturelessLambda(decl);
    return false;
  }
  StrCat(LambdaMacro());
  PushParen paren(*this);
  if (decl->isGenericLambda()) {
    StrCat("Generic", token::kComma);
  }
  {
    PushBrace captures(*this);
    auto init = expr->capture_init_begin();
    for (auto field : decl->fields()) {
      ConvertLambdaCapture(field, *init++);
    }
  }
  ForEachLambdaCallOperator(decl, [&](clang::CXXMethodDecl *call_operator) {
    StrCat(token::kComma);
    ConvertLambdaClosure(decl, call_operator);
  });
  MarkFresh();
  return false;
}

void Converter::ConvertCapturelessLambda(const clang::CXXRecordDecl *decl) {
  assert(!decl->isGenericLambda());
  auto call_operator = decl->getLambdaCallOperator();
  auto proto = call_operator->getType()->castAs<clang::FunctionProtoType>();
  StrCat(std::format("FnPtr::<{}>::new", ConvertFunctionPointerType(proto)));
  PushParen paren(*this);
  ConvertLambdaClosure(decl, call_operator);
  MarkFresh();
}

void Converter::ConvertLambdaCapture(const clang::FieldDecl *field,
                                     clang::Expr *init) {
  StrCat(keyword::kLet, GetNamedDeclAsString(field), token::kColon);
  Convert(field->getType());
  StrCat(token::kAssign);
  ConvertVarInit(field->getType(), init);
  StrCat(token::kSemiColon);
}

void Converter::ConvertLambdaClosure(const clang::CXXRecordDecl *decl,
                                     clang::CXXMethodDecl *call_operator) {
  StrCat('|');
  ConvertParamList(call_operator->parameters(), call_operator->isVariadic());
  StrCat('|');
  ConvertFunctionReturnType(call_operator);
  PushBrace body(*this);
  std::optional<PushBrace> unsafe_brace;
  if (decl->captures().empty() && !decl->isGenericLambda()) {
    StrCat(keyword_unsafe_);
    unsafe_brace.emplace(*this);
  }
  EmitFunctionPreamble(call_operator);
  ScopedValue push_fn(curr_function_, call_operator);
  ScopedValue function_body(in_const_initializer_, false);
  ConvertFunctionBody(curr_function_);
}

bool Converter::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) {
  if (auto *arr_ty = ctx_.getAsConstantArrayType(expr->getType())) {
    auto elem_ty = arr_ty->getElementType();
    if (elem_ty->isIntegerType() && !elem_ty->isEnumeralType()) {
      StrCat(std::format("[0; {}]", arr_ty->getSize().getZExtValue()));
    } else {
      StrCat(
          std::format("std::array::from_fn::<_, {}, _>(|_| Default::default())",
                      arr_ty->getSize().getZExtValue()));
    }
    MarkFresh();
    return false;
  }

  EmitFresh(GetDefaultAsString(expr->getType()));
  return false;
}

bool Converter::VisitCXXScalarValueInitExpr(
    clang::CXXScalarValueInitExpr *expr) {
  EmitFresh(GetDefaultAsString(expr->getType()));
  return false;
}

bool Converter::ConvertSwitchCaseCondition(clang::SwitchCase *stmt) {
  clang::Stmt *cur = stmt;
  clang::SwitchCase *last = nullptr;
  bool first = true;

  while (auto *sc = clang::dyn_cast<clang::SwitchCase>(cur)) {
    if (auto *case_stmt = clang::dyn_cast<clang::CaseStmt>(sc)) {
      if (!first) {
        StrCat("|| __v == ");
      }
      Convert(case_stmt->getLHS());
    }
    last = sc;
    first = false;
    cur = sc->getSubStmt();
  }

  if (clang::isa<clang::CaseStmt>(last)) {
    StrCat(" => ");
  } else /* DefaultStmt */ {
    StrCat("_ => ");
  }
  return false;
}

void Converter::EmitSwitchArm(const SwitchArm &arm, bool is_default) {
  if (is_default) {
    StrCat("_ => ");
  } else {
    StrCat("__v if __v == ");
    ConvertSwitchCaseCondition(arm.head);
  }
  if (!arm.label.empty()) {
    StrCat(std::format("'{}: ", arm.label.str()));
  }
  StrCat(token::kOpenCurlyBracket);
  for (auto *t : arm.body) {
    Convert(t);
  }
  StrCat("},");
}

bool Converter::VisitSwitchStmt(clang::SwitchStmt *stmt) {
  auto *body = clang::dyn_cast<clang::CompoundStmt>(stmt->getBody());
  assert(body);
  auto arms = AnalyzeSwitchArms(body);

  bool needs_switch_macro = std::ranges::any_of(arms, [](const SwitchArm &arm) {
    return !arm.label.empty() || arm.has_fallthrough;
  });

  ScopedPush push(break_target_, needs_switch_macro
                                     ? BreakTarget::FallthroughSwitch
                                     : BreakTarget::Switch);

  if (needs_switch_macro) {
    StrCat("switch!");
  } else {
    StrCat("'switch:");
  }

  PushParen switch_macro_paren(*this, needs_switch_macro);
  PushBrace switch_label_brace(*this, !needs_switch_macro);

  if (needs_switch_macro) {
    StrCat("match", ToString(stmt->getCond()));
  } else {
    StrCat(std::format("match {{ {} }}", ConvertRValue(stmt->getCond())));
  }

  PushBrace match_brace(*this);

  const SwitchArm *default_arm = nullptr;
  for (const auto &arm : arms) {
    if (arm.is_default_case) {
      default_arm = &arm;
      continue;
    }
    EmitSwitchArm(arm, /*is_default=*/false);
  }

  if (default_arm) {
    EmitSwitchArm(*default_arm, /*is_default=*/true);
  } else {
    StrCat(R"( _ => {})");
  }

  return false;
}

// TODO: right now defaults go into the constructor, but they should also be
// placed in the Default trait impl.
bool Converter::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr *expr) {
  Convert(expr->getExpr());
  return false;
}

bool Converter::VisitPredefinedExpr(clang::PredefinedExpr *expr) {
  Convert(expr->getFunctionName());
  return false;
}

bool Converter::VisitClassTemplateDecl(clang::ClassTemplateDecl *decl) {
  for (auto decl : decl->specializations()) {
    VisitCXXRecordDecl(decl);
  }
  return false;
}

bool Converter::VisitCXXStdInitializerListExpr(
    clang::CXXStdInitializerListExpr *expr) {
  auto *backing = expr->getSubExpr();
  auto str = GetMappedAsString(expr, &backing, 1);
  ENSURE(!str.empty());
  EmitFresh(str);
  return false;
}

std::string Converter::GetArrayDefaultAsString(clang::QualType qual_type) {
  if (auto *array_type = clang::dyn_cast<clang::ConstantArrayType>(qual_type)) {
    auto size_as_string = GetNumAsString(array_type->getSize());
    auto element_type = array_type->getElementType();
    auto element_type_as_string = GetDefaultAsString(element_type);
    if (element_type->getAsRecordDecl() && !RecordDerivesCopy(element_type)) {
      return std::format("std::array::from_fn::<_, {}, _>(|_| {})",
                         size_as_string.c_str(), element_type_as_string);
    }
    return std::format("[{}; {}]", element_type_as_string,
                       size_as_string.c_str());
  }
  if (clang::isa<clang::IncompleteArrayType>(qual_type)) {
    return "[]";
  }
  return {};
}

std::string Converter::GetDefaultAsString(clang::QualType qual_type) {
  if (auto lambda = AsLambdaClass(qual_type);
      lambda && lambda->captures().empty()) {
    Buffer buf(*this);
    ConvertCapturelessLambda(lambda);
    return std::move(buf).str();
  }

  if (IsVaListType(qual_type)) {
    return "::std::ptr::null()";
  }

  if (auto arr = GetArrayDefaultAsString(qual_type); !arr.empty()) {
    return arr;
  }

  if (auto init = Mapper::MapInitializer(*this, qual_type); !init.empty()) {
    return init;
  }

  if (qual_type->isPointerType()) {
    auto pointee = qual_type->getPointeeType();
    if (pointee->isFunctionType()) {
      return "None";
    }
    return pointee.isConstQualified() ? "std::ptr::null()"
                                      : "std::ptr::null_mut()";
  }

  return GetDefaultAsStringFallback(qual_type);
}

std::string Converter::GetDefaultAsStringFallback(clang::QualType qual_type) {
  qual_type = qual_type.getUnqualifiedType().getCanonicalType();

  if (auto record = qual_type->getAsRecordDecl()) {
    if (ctx_.getSourceManager().isInSystemHeader(record->getLocation()) &&
        qual_type.isPODType(ctx_)) {
      return std::format("unsafe {{ std::mem::zeroed::<{}>() }}",
                         ToString(qual_type));
    }
  }

  if (qual_type->isEnumeralType()) {
    auto enum_decl = qual_type->castAs<clang::EnumType>()->getDecl();
    if (enum_decl->enumerators().empty()) {
      return GetDefaultAsString(enum_decl->getIntegerType());
    }
    return EnumeratorName(*enum_decl->enumerator_begin());
  }

  return std::format("<{}>::default()", ToString(qual_type));
}

std::string Converter::ConvertVarDefaultInit(const clang::VarDecl *decl) {
  return GetDefaultAsString(decl->getType());
}

std::string
Converter::GetOverloadedFunctionName(const clang::CXXMethodDecl *decl) {
  auto name = GetFunctionBaseName(decl);
  if (auto *conversion = clang::dyn_cast<clang::CXXConversionDecl>(decl)) {
    name = GetConversionName(conversion);
  }
  name += std::format("_{}", GetMethodIndex(decl));

  if (const auto *targs = decl->getTemplateSpecializationArgs()) {
    std::vector<clang::TemplateArgument> args;
    for (const auto &arg : targs->asArray()) {
      if (arg.getKind() == clang::TemplateArgument::Pack) {
        args.insert(args.end(), arg.pack_begin(), arg.pack_end());
      } else {
        args.push_back(arg);
      }
    }
    for (const auto &arg : args) {
      name += '_';
      switch (arg.getKind()) {
      case clang::TemplateArgument::Type:
        name += Printer::ToRustName(
            arg.getAsType().getCanonicalType().getAsString());
        break;
      case clang::TemplateArgument::Integral:
        name += Printer::ToRustName(
            std::string(GetNumAsString(arg.getAsIntegral())));
        break;
      default:
        name += "targ";
        break;
      }
    }
  }

  ToIdentifier(name);
  return name;
}

std::string Converter::GetRecordName(const clang::NamedDecl *decl) const {
  auto ID = GetID(decl);
  if (auto it = inner_structs_.find(ID); it != inner_structs_.end()) {
    return it->second;
  }
  return Printer::ToRustName(
      Printer::ToString(ctx_, GetTypeForDecl(ctx_, decl)));
}

std::vector<const char *>
Converter::GetStructAttributes(const clang::RecordDecl *decl) {
  if (decl->isUnion()) {
    return {"Copy", "Clone", "FnPtrArg", "VaArg"};
  }

  std::vector<const char *> struct_attrs;

  if (HasDefaultedCopyConstructor(decl) && RecordHasCopyableFields(decl)) {
    struct_attrs.emplace_back("Copy");
  }

  if (HasDefaultedCopyConstructor(decl)) {
    struct_attrs.emplace_back("Clone");
  }

  if (RecordImplementsClone(decl)) {
    struct_attrs.emplace_back("VaArg");
  }
  struct_attrs.emplace_back("FnPtrArg");

  if (HasMoveFromConstructor(decl)) {
    struct_attrs.emplace_back("MoveCtorUnsafe");
  }

  if (HasImplicitConversions(decl)) {
    struct_attrs.emplace_back("CppFromUnsafe");
  }

  if (HasCopyAssignOperator(decl)) {
    struct_attrs.emplace_back("CopyAssignUnsafe");
  }

  if (HasMoveAssignOperator(decl)) {
    struct_attrs.emplace_back("MoveAssignUnsafe");
  }

  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && RecordNeedsDestruction(cxx)) {
    struct_attrs.emplace_back("DestructorUnsafe");
  }

  if (RecordDerivesDefault(decl)) {
    struct_attrs.emplace_back("Default");
  }

  return struct_attrs;
}

std::string Converter::GetUnsafeTypeAsString(clang::QualType qual_type) const {
  std::string type_as_string;
  Converter converter(type_as_string, ctx_);
  converter.Convert(qual_type);
  return std::string(Trim(type_as_string));
}

void Converter::ConvertVarInit(clang::QualType qual_type, clang::Expr *expr) {
  if (qual_type->isFunctionReferenceType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    ScopedPush init_type(curr_init_type_, qual_type);
    Convert(expr);
    return;
  }
  if (qual_type->isReferenceType() && !IsReferenceType(expr)) {
    if (llvm::isa<clang::MaterializeTemporaryExpr>(expr->IgnoreImpCasts())) {
      StrCat(EmitMaterializedTempBinding(qual_type, expr));
      return;
    }
    if (auto *cond = clang::dyn_cast<clang::ConditionalOperator>(
            expr->IgnoreParenImpCasts());
        cond && cond->isLValue()) {
      PushExprKind push(*this, ExprKind::AddrOf);
      Convert(cond);
      return;
    }
    StrCat(token::kRef);
    if (IsMut(qual_type)) {
      StrCat(keyword_mut_);
    }
  }
  PushExprKind push(*this,
                    IsReferenceType(expr) || qual_type->isFunctionPointerType()
                        ? ExprKind::AddrOf
                        : ExprKind::RValue);
  ScopedPush init_type(curr_init_type_, qual_type);
  Convert(expr, qual_type);
}

void Converter::ConvertUnsignedArithOperand(clang::Expr *expr,
                                            clang::QualType type) {
  bool needs_cast =
      (expr->isIntegerConstantExpr(ctx_) &&
       !clang::isa<clang::ImplicitCastExpr>(expr)) ||
      Mapper::Map(*this, expr->getType()) != Mapper::Map(*this, type);
  PushParen paren(*this, needs_cast);
  Convert(expr);
  if (needs_cast) {
    ConvertCast(type);
  }
}

void Converter::ConvertEqualsNullPtr(clang::Expr *expr) {
  StrCat('(');
  Convert(expr);
  EmitFresh(expr->getType()->isFunctionPointerType() ? ").is_none()"
                                                     : ").is_null()");
}

void Converter::ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) {
  bool deref = !isAddrOf();
  PushParen paren(*this, deref);
  if (deref) {
    StrCat(token::kStar);
  }
  ConvertPointerOffset(expr->getBase(), expr->getIdx());
}

void Converter::ConvertPointerOffset(clang::Expr *base, clang::Expr *idx,
                                     bool is_addition) {
  Convert(base);
  StrCat(token::kDot, "offset");
  PushParen outer(*this);
  if (!is_addition) {
    StrCat(token::kMinus);
  }
  PushParen neg_paren(*this, !is_addition);
  {
    PushParen inner(*this);
    PushExprKind push(*this, ExprKind::RValue);
    Convert(idx);
  }
  StrCat(keyword::kAs, "isize");
  MarkFresh();
}

static bool IsFlexibleArrayMemberAccess(clang::ASTContext &ctx,
                                        clang::Expr *array) {
  return array->isFlexibleArrayMemberLike(
      ctx, clang::LangOptions::StrictFlexArraysLevelKind::OneZeroOrIncomplete,
      /*IgnoreTemplateOrMacroSubstitution=*/true);
}

void Converter::EmitFlexibleArrayElementPtr(clang::Expr *array,
                                            clang::Expr *idx, bool is_mut) {
  {
    ScopedValue no_autoref(autoref_mut_, std::nullopt);
    Convert(array);
  }
  StrCat(is_mut ? ".as_mut_ptr()" : ".as_ptr()", ".add");
  {
    PushParen call(*this);
    {
      PushParen paren(*this);
      Convert(idx);
    }
    StrCat(keyword::kAs, "usize");
  }
}

void Converter::ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                                      clang::QualType type) {
  if (auto inner = base->IgnoreImplicit()) {
    if (inner->getType()->isArrayType() &&
        IsFlexibleArrayMemberAccess(ctx_, inner)) {
      PushParen outer(*this);
      StrCat(token::kStar);
      EmitFlexibleArrayElementPtr(inner, idx,
                                  !inner->getType().isConstQualified());
      return;
    }
  }
  Convert(base->IgnoreImplicit());
  ScopedValue no_autoref(autoref_mut_, std::nullopt);
  PushBracket bracket(*this);
  {
    PushParen paren(*this);
    Convert(idx);
  }

  if (Mapper::Map(*this, idx->getType()) != "usize") {
    StrCat(keyword::kAs, "usize");
  }
}

void Converter::ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                  std::string_view assign_operator) {
  std::string lhs_as_string;
  {
    ScopedPush init_type(curr_init_type_, lhs->getType());
    lhs_as_string = ConvertLValue(lhs);
  }
  auto rhs_as_string = ConvertFreshRValue(rhs, lhs->getType());

  PushBrace brace(*this, !isVoid());

  StrCat(lhs_as_string, assign_operator, rhs_as_string);
  if (!isVoid()) {
    StrCat(token::kSemiColon,
           isAddrOf() ? ConvertRValue(lhs) : ConvertFreshRValue(lhs));
  }
}

void Converter::ConvertFunctionParameters(clang::FunctionDecl *decl) {
  ConvertParamList(DefinitionParameters(decl), decl->isVariadic());
}

void Converter::ConvertParamList(llvm::ArrayRef<clang::ParmVarDecl *> params,
                                 bool is_variadic) {
  ScopedValue in_formals(in_function_formals_, true);
  for (auto *param : params) {
    ConvertVarDeclSkipInit(param);
    StrCat(token::kComma);
  }
  if (is_variadic) {
    StrCat("__args: Box<[VaArg]>", token::kComma);
  }
}

void Converter::ConvertFunctionReturnType(clang::FunctionDecl *decl) {
  auto return_type = decl->getReturnType();
  if (!return_type->isVoidType()) {
    StrCat(token::kArrow);
    Convert(return_type);
  }
}

void Converter::ConvertFunctionMain(const clang::FunctionDecl *decl,
                                    const std::string_view main_function_name) {
  if (decl->getNumParams() != 0U) {
    StrCat(std::format(R"(
pub fn main() {{
    let mut args: Vec<Vec<u8>> = std::env::args().map(|arg| arg.as_bytes().to_vec()).collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args.iter().map(|arg| arg.as_ptr() as *mut libc::c_char).collect();
    argv.push(::std::ptr::null_mut());
    unsafe {{
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }}
}})",
                       main_function_name));
  } else {
    StrCat(std::format("pub fn main() {{ unsafe {{ __cpp2rust_init_globals(); "
                       "std::process::exit({}() as i32); }} }}",
                       main_function_name));
  }
}

void Converter::ConvertAbstractClass(clang::CXXRecordDecl *decl) {
  auto trait_name = GetRecordName(decl);
  auto access_specifier_as_string = AccessSpecifierAsString(decl->getAccess());
  auto signature = std::format("{} {} trait {}", access_specifier_as_string,
                               keyword_unsafe_, trait_name);
  auto predicate = [](auto *method) {
    return !method->isImplicit() &&
           !clang::isa<clang::CXXDestructorDecl>(method);
  };
  ConvertCXXMethodDecls(decl, signature, predicate);
}

void Converter::ConvertCXXMethodDecls(
    const clang::CXXRecordDecl *decl, const std::string_view signature,
    bool (*predicate)(clang::CXXMethodDecl *)) {
  bool first = true;
  auto convert_method = [&](clang::CXXMethodDecl *method) {
    if (predicate(method)) {
      if (first) {
        StrCat(signature, token::kOpenCurlyBracket);
        first = false;
      }
      VisitCXXMethodDecl(method);
    }
  };
  ForEachMethod(decl, convert_method);
  if (!first) {
    StrCat(token::kCloseCurlyBracket);
  }
}

Converter::DeferredBlock &
Converter::VirtualMethodsFor(const clang::CXXRecordDecl *decl) {
  auto name = GetRecordName(decl);
  auto [it, inserted] = virtual_methods_.try_emplace(name);
  if (inserted) {
    it->second.header = std::format(
        "{} impl {} for {}", keyword_unsafe_,
        GetUnsafeTypeAsString(decl->bases_begin()->getType()), name);
  }
  return it->second;
}

void Converter::ConvertVirtualMethods(clang::CXXRecordDecl *decl) {
  if (decl->bases_begin() == decl->bases_end()) {
    return;
  }
  bool any = false;
  Buffer buf(*this);
  for (auto *method : decl->methods()) {
    if (!method->isImplicit() && method->isVirtual()) {
      any = true;
      VisitCXXMethodDecl(method);
    }
  }
  auto body = std::move(buf).str();
  if (!any) {
    return;
  }
  VirtualMethodsFor(decl).body += body;
}

bool Converter::ConvertOutOfLineVirtualMethod(clang::CXXMethodDecl *decl) {
  auto *record = decl->getParent();
  if (record->bases_begin() == record->bases_end()) {
    return false;
  }
  Buffer buf(*this);
  auto emitted = ConvertCXXMethodDecl(decl);
  VirtualMethodsFor(record).body += std::move(buf).str();
  return emitted;
}

void Converter::ConvertOrdAndPartialOrdTraitsBase(
    std::string_view cmp_body, std::string_view eq_body,
    std::string_view record_name) {
  if (!cmp_body.empty()) {
    StrCat(keyword::kImpl, "std::cmp::Ord for ", record_name, '{');
    StrCat("fn cmp(&self, other: &Self) -> std::cmp::Ordering {");
    StrCat(std::format("{} {{", keyword_unsafe_));
    StrCat(cmp_body);
    StrCat("}}}");

    StrCat(keyword::kImpl, "std::cmp::PartialOrd for", record_name, '{');
    StrCat(R"(
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
    }
  })");
  }

  StrCat(keyword::kImpl, "std::cmp::PartialEq for", record_name, '{');
  StrCat("fn eq(&self, other: &Self) -> bool {");
  StrCat(std::format("{} {{", keyword_unsafe_));
  StrCat(eq_body);
  StrCat("}}}");

  StrCat(keyword::kImpl, "std::cmp::Eq for", record_name, "{}");
}

std::string Converter::GetComparisonCall(const clang::FunctionDecl *op,
                                         const clang::CXXRecordDecl *decl,
                                         std::string_view lhs,
                                         std::string_view rhs) {
  if (const auto *method = clang::dyn_cast<clang::CXXMethodDecl>(op)) {
    return std::format("{}::{}({}, {})", GetUFCSName(method),
                       GetMethodName(method),
                       GetComparisonReceiver(method, lhs),
                       GetComparisonArg(op->getParamDecl(0), decl, rhs));
  }
  return std::format("{}({}, {})", GetNamedDeclAsString(op->getCanonicalDecl()),
                     GetComparisonArg(op->getParamDecl(0), decl, lhs),
                     GetComparisonArg(op->getParamDecl(1), decl, rhs));
}

std::string Converter::GetComparisonBool(const clang::FunctionDecl *op,
                                         const clang::CXXRecordDecl *decl,
                                         std::string_view lhs,
                                         std::string_view rhs) {
  auto call = GetComparisonCall(op, decl, lhs, rhs);
  auto ret = op->getReturnType();
  if (ret->isBooleanType()) {
    return call;
  }
  if (ret->isIntegerType()) {
    return std::format("(({}) != 0)", call);
  }
  bool returns_ref = ret->isReferenceType();
  const auto *record = ret.getNonReferenceType()->getAsCXXRecordDecl();
  assert(record && "unsupported comparison operator return type");
  for (const auto *found : record->getVisibleConversionFunctions()) {
    const auto *conversion = clang::dyn_cast<clang::CXXConversionDecl>(found);
    if (conversion && conversion->getConversionType()->isBooleanType()) {
      return std::format(
          "{{ let __cmp = {}; {}::{}({}) }}", call, GetUFCSName(conversion),
          GetMethodName(conversion),
          GetComparisonResultReceiver(conversion, returns_ref, "__cmp"));
    }
  }
  assert(0 && "comparison operator result has no conversion to bool");
  return call;
}

std::string Converter::GetComparisonArg(const clang::ParmVarDecl *param,
                                        const clang::CXXRecordDecl *decl,
                                        std::string_view value) {
  if (param->getType()->isReferenceType()) {
    return std::format("{} as *const {}", value, GetRecordName(decl));
  }
  return std::format("{}.clone()", value);
}

std::string
Converter::GetComparisonResultReceiver(const clang::CXXMethodDecl *method,
                                       bool returns_ref,
                                       std::string_view result) {
  return GetComparisonReceiver(
      method, std::format("{}{}", returns_ref ? "&*" : "&", result));
}

std::string Converter::GetComparisonReceiver(const clang::CXXMethodDecl *method,
                                             std::string_view lhs) {
  if (!MethodNeedsMutableReceiver(method)) {
    return std::string(lhs);
  }
  return std::format("&mut *(&raw const *{}).cast_mut()", lhs);
}

std::string Converter::GetEqBody(const clang::CXXRecordDecl *decl,
                                 const clang::FunctionDecl *eq,
                                 const clang::FunctionDecl *lt,
                                 const clang::FunctionDecl *cmp,
                                 std::string_view lhs, std::string_view rhs) {
  if (eq) {
    return GetComparisonBool(eq, decl, lhs, rhs);
  }
  if (cmp) {
    return std::format("{} == std::cmp::Ordering::Equal",
                       GetComparisonCall(cmp, decl, lhs, rhs));
  }
  return std::format("!({}) && !({})", GetComparisonBool(lt, decl, lhs, rhs),
                     GetComparisonBool(lt, decl, rhs, lhs));
}

void Converter::ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                              const clang::FunctionDecl *eq,
                                              const clang::FunctionDecl *lt,
                                              const clang::FunctionDecl *cmp) {
  std::string cmp_body;
  if (cmp) {
    cmp_body = GetComparisonCall(cmp, decl, "self", "other");
  } else if (lt) {
    cmp_body = std::format("if {} {{ std::cmp::Ordering::Less }} else if {} {{ "
                           "std::cmp::Ordering::Greater }} else {{ "
                           "std::cmp::Ordering::Equal }}",
                           GetComparisonBool(lt, decl, "self", "other"),
                           GetComparisonBool(lt, decl, "other", "self"));
  }
  ConvertOrdAndPartialOrdTraitsBase(
      cmp_body, GetEqBody(decl, eq, lt, cmp, "self", "other"),
      GetRecordName(decl));
}

void Converter::AddOrdTrait(const clang::CXXRecordDecl *decl) {
  const clang::FunctionDecl *eq = nullptr;
  const clang::FunctionDecl *lt = nullptr;
  const clang::FunctionDecl *cmp = nullptr;
  auto consider = [&](const clang::FunctionDecl *fn) {
    if (!fn || fn->isImplicit() || fn->isDeleted() || !fn->hasBody() ||
        fn->getDescribedFunctionTemplate() || !IsSameTypeComparison(fn, decl)) {
      return;
    }
    if (fn->getOverloadedOperator() == clang::OO_Spaceship &&
        !ctx_.CompCategories.lookupInfoForType(fn->getReturnType())) {
      return;
    }
    switch (fn->getOverloadedOperator()) {
    case clang::OO_EqualEqual:
      eq = fn;
      break;
    case clang::OO_Less:
      lt = fn;
      break;
    case clang::OO_Spaceship:
      cmp = fn;
      break;
    default:
      break;
    }
  };
  auto consider_decl = [&](const clang::NamedDecl *found) {
    if (const auto *tmpl =
            clang::dyn_cast<clang::FunctionTemplateDecl>(found)) {
      for (const auto *spec : tmpl->specializations()) {
        consider(spec);
      }
      return;
    }
    consider(clang::dyn_cast<clang::FunctionDecl>(found));
  };
  for (const auto *method : decl->methods()) {
    consider(method);
  }
  for (auto op : {clang::OO_EqualEqual, clang::OO_Less, clang::OO_Spaceship}) {
    auto name = ctx_.DeclarationNames.getCXXOperatorName(op);
    for (const auto *found : decl->getDeclContext()->lookup(name)) {
      consider_decl(found);
    }
  }
  for (const auto *friend_decl : decl->friends()) {
    if (const auto *found = friend_decl->getFriendDecl()) {
      consider_decl(found);
    }
  }

  if (!eq && !lt && !cmp) {
    return;
  }

  ConvertOrdAndPartialOrdTraits(decl, eq, lt, cmp);
}

void Converter::AddCloneTrait(const clang::RecordDecl *decl) {
  auto *ctor = GetUserDefinedCopyConstructor(decl);
  if (!ctor) {
    return;
  }
  auto record_name = GetRecordName(decl);
  StrCat(keyword::kImpl, "Clone for", record_name);
  PushBrace impl_brace(*this);
  StrCat("fn clone(&self) -> Self");
  PushBrace fn_brace(*this);
  auto source = ctor->getParamDecl(0)->getType().getNonReferenceType();
  StrCat(std::format("unsafe {{ {}::{}(self as *const {}{}) }}", record_name,
                     GetCtorName(ctor), record_name,
                     source.isConstQualified()
                         ? ""
                         : std::format(" as *mut {}", record_name)));
}

bool Converter::RecordImplementsClone(const clang::RecordDecl *decl) {
  return HasDefaultedCopyConstructor(decl) ||
         GetUserDefinedCopyConstructor(decl) != nullptr;
}

void Converter::AddDefaultTraitForUnion(const clang::RecordDecl *decl) {
  EmitDefaultImpl(decl, [&] { StrCat("unsafe { std::mem::zeroed() }"); });
}

void Converter::AddDefaultTrait(const clang::RecordDecl *decl) {
  if (decl->isUnion()) {
    AddDefaultTraitForUnion(decl);
    return;
  }
  if (RecordDerivesDefault(decl)) {
    return;
  }
  EmitDefaultImpl(decl, [&] {
    StrCat(keyword_unsafe_);
    PushBrace unsafe_brace(*this);
    auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl);
    if (auto *default_ctor =
            cxx ? GetUserDefinedDefaultConstructor(cxx) : nullptr) {
      Convert(MakeConstructExpr(ctx_, ctx_.getCanonicalTagType(decl),
                                default_ctor, {}));
    } else {
      EmitDefaultStructLiteral(decl);
    }
  });
}

void Converter::EmitDefaultImpl(const clang::RecordDecl *decl,
                                llvm::function_ref<void()> body) {
  StrCat(std::format("impl Default for {}", GetRecordName(decl)));
  PushBrace impl_brace(*this);
  StrCat("fn default() -> Self");
  PushBrace fn_brace(*this);
  body();
}

void Converter::EmitDefaultStructLiteral(const clang::RecordDecl *decl) {
  StrCat(GetRecordName(decl));
  PushBrace brace(*this);
  for (auto *field : decl->fields()) {
    StrCat(GetNamedDeclAsString(field), token::kColon);
    ConvertFieldInit(field, field->getInClassInitializer());
    StrCat(token::kComma);
  }
}

void Converter::ConvertFieldInit(const clang::FieldDecl *field,
                                 clang::Expr *init) {
  if (init) {
    ConvertVarInit(field->getType(), init);
  } else {
    StrCat(GetDefaultAsString(field->getType()));
  }
}

void Converter::EmitByteSizeAttr(const clang::RecordDecl *decl) {}

void Converter::ConvertUnsignedArithBinaryOperator(clang::BinaryOperator *op,
                                                   clang::Expr *expr) {
  StrCat(token::kDot);
  auto opcode = op->getOpcode();
  switch (opcode) {
  case clang::BinaryOperator::Opcode::BO_Add:
  case clang::BinaryOperator::Opcode::BO_AddAssign:
    StrCat("wrapping_add");
    break;
  case clang::BinaryOperator::Opcode::BO_Sub:
  case clang::BinaryOperator::Opcode::BO_SubAssign:
    StrCat("wrapping_sub");
    break;
  case clang::BinaryOperator::Opcode::BO_Mul:
  case clang::BinaryOperator::Opcode::BO_MulAssign:
    StrCat("wrapping_mul");
    break;
  case clang::BinaryOperator::Opcode::BO_Div:
  case clang::BinaryOperator::Opcode::BO_DivAssign:
    StrCat("wrapping_div");
    break;
  case clang::BinaryOperator::Opcode::BO_Rem:
  case clang::BinaryOperator::Opcode::BO_RemAssign:
    StrCat("wrapping_rem");
    break;
  default:
    // FIXME: improve error handling
    llvm::errs() << "unsupported unsigned binary operator: " << opcode << '\n';
    op->dump();
    assert(0);
  }
  PushParen paren(*this);

  auto type = op->getType();
  bool is_pointer_plus_integer_op = false;

  if (auto *assign = llvm::dyn_cast<clang::CompoundAssignOperator>(op)) {
    if (op->getLHS()->getType()->isPointerType() &&
        op->getRHS()->getType()->isIntegralOrEnumerationType()) {
      type = op->getRHS()->getType();
      is_pointer_plus_integer_op = true;
    } else {
      type = assign->getComputationResultType();
    }
  }
  ConvertUnsignedArithOperand(expr, type);
  if (is_pointer_plus_integer_op) {
    StrCat("as usize");
  }
}

void Converter::ConvertAddrOf(clang::Expr *expr, clang::QualType pointer_type) {
  assert(pointer_type->isPointerType());
  if (auto ase =
          clang::dyn_cast<clang::ArraySubscriptExpr>(expr->IgnoreParens())) {
    auto base = ase->getBase();
    auto inner = base->IgnoreImplicit();
    if (base->IgnoreCasts()->getType()->isArrayType() &&
        IsFlexibleArrayMemberAccess(ctx_, inner)) {
      EmitFlexibleArrayElementPtr(
          inner, ase->getIdx(),
          !pointer_type->getPointeeType().isConstQualified());
      MarkFresh();
      return;
    }
  }
  auto *cond =
      clang::dyn_cast<clang::ConditionalOperator>(expr->IgnoreParens());
  if (IsReferenceType(expr) || pointer_type->isFunctionPointerType() ||
      (cond && cond->isLValue())) {
    PushExprKind push(*this, ExprKind::AddrOf);
    Convert(expr);
  } else {
    bool is_const = pointer_type->getPointeeType().isConstQualified();
    if (IsGlobalVar(expr)) {
      StrCat("&raw", is_const ? keyword::kConst : keyword_mut_);
    } else {
      StrCat(token::kRef, is_const ? "" : keyword_mut_);
    }
    PushExprKind push(*this, ExprKind::LValue);
    Convert(expr);
    ConvertCast(pointer_type);
    MarkFresh();
  }
}

void Converter::EmitDeref(std::string inner, clang::QualType pointee_type) {
  auto wrap = std::exchange(autoref_mut_, std::nullopt);
  PushParen outer(*this, wrap.has_value());
  if (wrap) {
    StrCat(*wrap ? "&mut" : "&");
  }
  PushParen paren(*this);
  StrCat(GetPointerDerefPrefix(pointee_type), std::move(inner));
  SetValueFreshness(pointee_type);
}

void Converter::ConvertDeref(clang::Expr *expr) {
  if (isAddrOf()) {
    Convert(expr);
  } else {
    EmitDeref(ToString(expr), expr->getType()->getPointeeType());
  }
}

void Converter::ConvertArrow(clang::Expr *expr) { ConvertDeref(expr); }

void Converter::ConvertCast(clang::QualType qual_type, int line) {
  log() << "[ConvertCast] Called from line " << line << '\n';
  StrCat(keyword::kAs, GetUnsafeTypeAsString(qual_type));
}

Converter::TempMaterializationCtx
Converter::CollectRefBindingTempArgs(clang::CallExpr *expr) {
  TempMaterializationCtx ctx(expr->getNumArgs());
  if (auto *fn = expr->getCalleeDecl() ? expr->getCalleeDecl()->getAsFunction()
                                       : nullptr) {
    for (unsigned i = 0; i < expr->getNumArgs() && i < fn->getNumParams();
         ++i) {
      auto param_type = fn->getParamDecl(i)->getType();
      if (NeedsRefBindingTemp(ctx_, expr->getArg(i), param_type)) {
        ctx.materialized_args[i] = param_type;
      }
    }
  }
  return ctx;
}

std::string Converter::TempMaterializationCtx::Wrap(std::string str) const {
  if (temporary_bindings.empty()) {
    return str;
  }
  return std::format("{{ {} {} }}", temporary_bindings, str);
}

const std::string &Converter::TempMaterializationCtx::GetOrMaterialize(
    unsigned argument_num,
    llvm::function_ref<std::pair<std::string, std::string>(clang::QualType)>
        materialize_fn) {
  auto &str = materialized_refs_.at(argument_num);
  if (!str.empty()) {
    return str;
  }

  if (auto m = materialized_args.at(argument_num)) {
    auto [binding, ref] = materialize_fn(*m);
    temporary_bindings += std::move(binding);
    str = std::move(ref);
    return str;
  }

  static const std::string empty_str;
  return empty_str;
}

void Converter::PlaceholderCtx::dump() const {
  llvm::errs() << "is_receiver: " << is_receiver
               << ", is_cpp_ptr: " << is_cpp_ptr
               << ", maps_to_rust_ptr: " << maps_to_rust_ptr
               << ", declared_in_rule_as_rust_ptr: "
               << declared_in_rule_as_rust_ptr
               << ", access: " << static_cast<int>(access)
               << ", arg_idx: " << arg_idx
               << ", materialize_idx: " << materialize_idx << '\n';
}

std::string Converter::ConvertPlaceholder(clang::Expr *expr, clang::Expr *arg,
                                          const PlaceholderCtx &ph_ctx) {
  if (arg->getType()->isFunctionPointerType()) {
    return ConvertFnPtrPlaceholder(arg);
  }

  if (ph_ctx.declared_in_rule_as_rust_ptr && arg->getType()->isArrayType()) {
    auto elem = ctx_.getAsArrayType(arg->getType())->getElementType();
    return std::format("({} as {})", ConvertFreshPointer(arg),
                       ToStringBase(ctx_.getPointerType(elem)));
  }

  if (auto *init =
          RuleRegistry::BuildParamConversion(ctx_, expr, ph_ctx.arg_idx, arg)) {
    Buffer buf(*this);
    PushExprKind push(*this, ExprKind::RValue);
    ConvertInit(init->getType(), init);
    return std::move(buf).str();
  }

  if (ph_ctx.needs_materialization()) {
    auto materialized = ph_ctx.materialize_ctx->GetOrMaterialize(
        static_cast<unsigned>(ph_ctx.materialize_idx),
        [this, arg](clang::QualType type) {
          return MaterializeTemp(NewTempName(), type, arg);
        });
    if (!materialized.empty()) {
      return materialized;
    }
  }

  // A pointer to the elements of the container is a new pointer, whatever the
  // rule does with it.
  if ((ph_ctx.needs_pointer_receiver() || ph_ctx.needs_ptr_wrap()) &&
      ParamIsElementPtr(expr, arg, ph_ctx.arg_idx)) {
    return ConvertElementPtr(arg);
  }

  if (ph_ctx.needs_pointer_receiver()) {
    auto param_type = Mapper::GetParamType(*this, expr, ph_ctx.arg_idx);
    bool borrowed = ph_ctx.access == TranslationRule::Access::kBorrow ||
                    ph_ctx.access == TranslationRule::Access::kBorrowMut;
    PushExprKind push(*this, borrowed ? ExprKind::Object : ExprKind::XValue);
    return ConvertPointerReceiver(arg, param_type);
  }

  if (ph_ctx.needs_object_receiver()) {
    Buffer buf(*this);
    ScopedValue autoref(autoref_mut_,
                        ph_ctx.is_index_base
                            ? std::optional(ph_ctx.access ==
                                            TranslationRule::Access::kBorrowMut)
                            : std::nullopt);
    PushExprKind push(*this, ExprKind::RValue);
    ConvertDeref(arg);
    return std::move(buf).str();
  }

  if (ph_ctx.needs_ptr_wrap()) {
    if (clang::isa<clang::MaterializeTemporaryExpr>(arg)) {
      return ConvertTemporaryPointer(arg, ph_ctx.implicit_convert_to);
    }
    return ConvertFreshObject(arg);
  }

  if (ph_ctx.needs_lvalue()) {
    return ConvertBorrowMut(arg);
  }

  // The rule takes the argument by value from an rvalue reference parameter,
  // e.g., push_back(T &&), and so C++ moves it.
  auto param_type = GetParamType(expr, ph_ctx.arg_idx);
  bool moved = ph_ctx.access == TranslationRule::Access::kTake ||
               (ph_ctx.access == TranslationRule::Access::kMove && param_type &&
                (*param_type)->isRValueReferenceType());
  if (moved && !clang::isa<clang::MaterializeTemporaryExpr>(arg)) {
    if (auto *record = arg->getType()->getAsCXXRecordDecl();
        record && IsUserDefinedDecl(record)) {
      for (auto *ctor : record->ctors()) {
        if (!IsConvertibleMoveConstructor(ctor)) {
          continue;
        }
        Buffer buf(*this);
        Convert(MakeConstructExpr(ctx_, arg->getType(), ctor, arg));
        return std::move(buf).str();
      }
      if (TypeIsCopyable(arg->getType())) {
        return ConvertRValue(arg);
      }
      return ConvertFreshRValue(arg);
    }
  }

  if (ph_ctx.access == TranslationRule::Access::kTake) {
    if (clang::isa<clang::MaterializeTemporaryExpr>(arg)) {
      return ConvertRValue(arg);
    }
    return ConvertTake(arg);
  }

  if (ph_ctx.access == TranslationRule::Access::kMove) {
    return ConvertFreshRValue(arg, ph_ctx.implicit_convert_to);
  }

  return ConvertRValue(arg, ph_ctx.implicit_convert_to);
}

std::string Converter::ConvertBorrowMut(clang::Expr *arg) {
  return ConvertLValue(arg);
}

std::string Converter::ConvertTake(clang::Expr *arg) {
  auto lvalue = ConvertLValue(arg);
  MarkFresh();
  return std::format("std::mem::take(&mut {})", std::move(lvalue));
}

std::string Converter::ConvertMappedMethodCall(
    clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
    llvm::ArrayRef<clang::TemplateArgument> bindings, clang::Expr **args,
    unsigned num_args, TempMaterializationCtx *ctx) {
  return ConvertIRFragment(mc.receiver, bindings, expr, args, num_args, ctx) +
         ConvertIRFragment(mc.body, bindings, expr, args, num_args, ctx);
}

std::string Converter::GetMappedAsString(clang::Expr *expr, clang::Expr **args,
                                         unsigned num_args,
                                         TempMaterializationCtx *ctx) {
  auto [tgt_ir, bindings] = RuleRegistry::Search(ctx_, expr);
  if (!tgt_ir)
    return {};

  CheckRuleReturnType(expr);

  auto saved_borrows = std::exchange(mut_borrows_, {});
  auto *operand = tgt_ir->refExprOperand();
  bool lvalue = operand && !isAddrOf();
  auto result = ConvertIRFragment(lvalue ? *operand : tgt_ir->body, bindings,
                                  expr, args, num_args, ctx);
  if (tgt_ir->multi_statement) {
    result = '{' + result + '}';
  }
  // The arguments borrowed through pointers are borrowed for the whole call.
  for (auto &[place, name] : std::ranges::reverse_view(mut_borrows_)) {
    result = std::format("{}.with_mut(|{}| {})", place, name, result);
  }
  mut_borrows_ = std::move(saved_borrows);
  return result;
}

std::string Converter::ConvertIRFragment(
    const std::vector<TranslationRule::BodyFragment> &fragments,
    llvm::ArrayRef<clang::TemplateArgument> bindings, clang::Expr *expr,
    clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx) {
  using namespace TranslationRule;

  std::vector<clang::Expr *> all_args;
  if (expr) {
    all_args = BuildUnifiedArgs(expr, args, num_args);
  }

  std::string result;
  for (auto &frag : fragments) {
    if (auto *t = std::get_if<TextFragment>(&frag)) {
      result += t->text;
    } else if (auto *g = std::get_if<GenericFragment>(&frag)) {
      result += GetTemplateArgumentAsString(bindings[g->n - 1]);
    } else if (auto *ph = std::get_if<PlaceholderFragment>(&frag)) {
      assert(expr && "placeholder outside an expression rule");
      auto arg_idx = ph->n;
      assert(arg_idx < all_args.size());
      auto *arg = all_args[arg_idx];
      bool is_receiver = HasReceiver(expr) && arg_idx == 0;
      bool is_cpp_ptr = arg->getType()->isPointerType();

      PlaceholderCtx ph_ctx{
          .arg_idx = arg_idx,
          .implicit_convert_to = GetParamImplicitConvertTarget(expr, arg_idx),
          .materialize_ctx = ctx,
          .materialize_idx =
              is_receiver ? -1 : ((int)arg_idx - HasReceiver(expr)),
          .access = ph->access,
          .is_receiver = is_receiver,
          .is_cpp_ptr = is_cpp_ptr,
          // Whatever its spelling, a pointer in C++ is converted as one.
          .maps_to_rust_ptr =
              !is_cpp_ptr && RuleRegistry::MapsToPointer(ctx_, arg->getType()),
          .declared_in_rule_as_rust_ptr =
              RuleRegistry::ParamIsPointer(ctx_, expr, arg_idx),
          .is_index_base = ph->is_index_base,
      };
      result += ConvertPlaceholder(expr, arg, ph_ctx);
    } else if (std::get_if<TranslationRule::VaArgsFragment>(&frag)) {
      assert(expr && "variadic arguments outside an expression rule");
      result += ConvertVariadicTail(expr, all_args);
    } else if (std::get_if<TranslationRule::InitFragment>(&frag)) {
      assert(expr && "init outside an expression rule");
      result += ConvertInitFragment(expr, all_args);
    } else if (auto *mc =
                   std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
      result +=
          ConvertMappedMethodCall(expr, **mc, bindings, args, num_args, ctx);
    } else if (auto *re =
                   std::get_if<std::unique_ptr<RefExprFragment>>(&frag)) {
      result +=
          (*re)->prefix + ' ' +
          ConvertIRFragment((*re)->inner, bindings, expr, args, num_args, ctx);
    } else if (auto *call =
                   std::get_if<std::unique_ptr<GenericCallFragment>>(&frag)) {
      result += ConvertGenericCall(**call, bindings, expr, args, num_args, ctx);
    }
  }

  return result;
}

std::string
Converter::ConvertGenericCall(const TranslationRule::GenericCallFragment &call,
                              llvm::ArrayRef<clang::TemplateArgument> bindings,
                              clang::Expr *expr, clang::Expr **args,
                              unsigned num_args, TempMaterializationCtx *ctx) {
  std::vector<std::string> call_args;
  for (const auto &arg : call.args) {
    call_args.push_back(
        ConvertIRFragment(arg, bindings, expr, args, num_args, ctx));
  }
  clang::QualType type;
  if (call.generic <= bindings.size() &&
      bindings[call.generic - 1].getKind() == clang::TemplateArgument::Type) {
    type = bindings[call.generic - 1].getAsType();
  } else {
    assert(expr && "unbound generic outside an expression rule");
    type = RuleRegistry::GetGenericType(ctx_, expr, call.generic);
  }
  if (call.method == "default") {
    assert(call_args.empty() && "default takes no arguments");
    return GetUnboxedDefaultAsString(type);
  }
  if (call.method == "cpp_from" || call.method == "cpp_from_move") {
    assert(call_args.size() == 1 && "cpp_from takes one argument");
    return ConvertConversion(
        type, RuleRegistry::GetConversionSourceType(ctx_, expr, call.generic),
        call.method == "cpp_from_move", call_args[0]);
  }
  return std::format("<{}>::{}({})", GetUnboxedTypeAsString(type), call.method,
                     llvm::join(call_args, ", "));
}

std::string Converter::ConvertConversion(clang::QualType to,
                                         clang::QualType from, bool is_move,
                                         const std::string &ptr) {
  if (to->isArithmeticType() && from->isArithmeticType()) {
    auto value = DerefPtrExpr(ptr, from);
    if (ctx_.hasSameUnqualifiedType(to, from)) {
      return value;
    }
    if (to->isBooleanType()) {
      return std::format("({} != {})", value,
                         from->isFloatingType() ? "0.0" : "0");
    }
    if (from->isBooleanType() && to->isFloatingType()) {
      return std::format("({} as u8 as {})", value, GetUnsafeTypeAsString(to));
    }
    return std::format("({} as {})", value, GetUnsafeTypeAsString(to));
  }
  return std::format("<{} as {}<{}>>::{}({})", GetUnboxedTypeAsString(to),
                     GetConversionTraitName(), GetUnboxedTypeAsString(from),
                     is_move ? "cpp_from_move" : "cpp_from", ptr);
}

const char *Converter::GetConversionTraitName() const {
  return "CppFromUnsafe";
}

std::string Converter::DerefPtrExpr(std::string_view ptr_expr,
                                    clang::QualType pointee_type) {
  return std::format("({}{})", GetPointerDerefPrefix(pointee_type), ptr_expr);
}

std::string
Converter::ConvertVariadicTail(clang::Expr *expr,
                               const std::vector<clang::Expr *> &all_args) {
  const auto *tgt_ir = RuleRegistry::GetExprRule(ctx_, expr);
  unsigned fixed = tgt_ir ? tgt_ir->params.size() : 0;

  Buffer buf(*this);
  StrCat("Box::<[VaArg]>::from([");
  EmitVariadicArgs(llvm::ArrayRef(all_args).drop_front(fixed),
                   /*into_va_arg=*/true);
  StrCat("])");
  return std::move(buf).str();
}

std::string
Converter::ConvertInitFragment(clang::Expr *expr,
                               const std::vector<clang::Expr *> &all_args) {
  const auto *tgt_ir = RuleRegistry::GetExprRule(ctx_, expr);
  if (!tgt_ir || !tgt_ir->usesInit()) {
    llvm::report_fatal_error("expression rule does not use Init");
  }
  auto type = RuleRegistry::GetInitType(expr);

  Buffer buf(*this);
  PushExprKind push(*this, ExprKind::RValue);
  ConvertConstructFromArgs(
      type, llvm::ArrayRef(all_args).drop_front(tgt_ir->params.size()),
      expr->getExprLoc());
  return std::move(buf).str();
}

void Converter::ConvertConstructFromArgs(clang::QualType type,
                                         llvm::ArrayRef<clang::Expr *> args,
                                         clang::SourceLocation loc) {
  auto *init = BuildInitExpr(GetSema(), type, args, loc);
  assert(init && "type cannot be initialized from the arguments");
  if (args.empty() &&
      !clang::isa<clang::CXXConstructExpr>(init->IgnoreImplicit())) {
    StrCat(GetDefaultAsString(type));
    return;
  }
  ConvertInit(type, init);
}

void Converter::ConvertInit(clang::QualType type, clang::Expr *init) {
  if (auto *ctor =
          clang::dyn_cast<clang::CXXConstructExpr>(init->IgnoreImplicit())) {
    ConvertConstructedValue(type, ctor);
    return;
  }
  Convert(init);
}

std::string Converter::AccessLValueObject(clang::MemberExpr *member) {
  auto *object = member->getBase();
  auto type = object->getType();
  if (type->isPointerType() ||
      (IsReferenceType(object) && clang::isa<clang::CallExpr>(object))) {
    return std::format("({}{})", GetPointerDerefPrefix(type->getPointeeType()),
                       ToString(object));
  }
  return ToString(object);
}

bool Converter::isLValue() const {
  return curr_expr_kind_.empty() || curr_expr_kind_.back() == ExprKind::LValue;
}

bool Converter::isRValue() const {
  return curr_expr_kind_.empty() || curr_expr_kind_.back() == ExprKind::RValue;
}

bool Converter::isXValue() const {
  return !curr_expr_kind_.empty() && curr_expr_kind_.back() == ExprKind::XValue;
}

bool Converter::isAddrOf() const {
  return !curr_expr_kind_.empty() &&
         (curr_expr_kind_.back() == ExprKind::AddrOf ||
          curr_expr_kind_.back() == ExprKind::Object);
}

bool Converter::isObject() const {
  return !curr_expr_kind_.empty() && curr_expr_kind_.back() == ExprKind::Object;
}

bool Converter::isVoid() const {
  return curr_expr_kind_.empty() || curr_expr_kind_.back() == ExprKind::Void;
}

bool Converter::isCallee() const {
  return !curr_expr_kind_.empty() && curr_expr_kind_.back() == ExprKind::Callee;
}

bool Converter::ShouldReplaceWithMappedBody(
    const clang::ValueDecl *decl) const {
  return !clang::isa<clang::FunctionDecl>(decl) || !isAddrOf();
}

void Converter::EmitFresh(std::string_view str) {
  StrCat(str);
  MarkFresh();
}

void Converter::SetValueFreshness(clang::QualType type) {
  if (TypeIsCopyable(type)) {
    MarkFresh();
  } else {
    MarkNotFresh();
  }
}

void Converter::dump_expr_kinds() {
  log() << "isRValue: " << isRValue() << ", isXValue: " << isXValue()
        << ", isAddrOf: " << isAddrOf() << ", isObject: " << isObject()
        << ", isVoid: " << isVoid() << '\n';
}

void Converter::ConvertConstructedValue(clang::QualType type,
                                        clang::CXXConstructExpr *ctor) {
  ConvertVarInit(type, ctor);
}

const char *Converter::GetPointerDerefPrefix(clang::QualType pointee_type) {
  return token::kStar;
}

} // namespace cpp2rust
