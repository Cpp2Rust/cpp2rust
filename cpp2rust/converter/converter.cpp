// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter.h"

#include <clang/AST/APValue.h>
#include <clang/AST/ParentMapContext.h>
#include <llvm/ADT/DenseMap.h>
#include <llvm/Support/ConvertUTF.h>

#include <format>
#include <utility>

#include "compiler.h"
#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/mapper.h"

namespace cpp2rust {
std::unordered_map<std::string, std::string> Converter::inner_structs_;
std::unordered_set<std::string> Converter::record_decls_;
std::unordered_set<std::string> Converter::decl_ids_;
std::unordered_set<std::string> Converter::globals_;
std::unordered_set<std::string> Converter::abstract_structs_;

void Converter::ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr) {
  bool is_star = expr->getOperator() == clang::OverloadedOperatorKind::OO_Star;
  PushParen paren(*this, is_star);
  if (is_star) {
    StrCat(token::kStar);
  }
  if (expr->getArg(0)->IgnoreImplicit()->getType().isConstQualified()) {
    StrCat("(*(std::ptr::addr_of!(");
    Convert(expr->getArg(0));
    StrCat(").cast_mut())).as_deref_mut().unwrap()");
  } else {
    Convert(expr->getArg(0));
    StrCat(".as_deref_mut().unwrap()");
  }
}

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

bool Converter::VisitRecoveryExpr(clang::RecoveryExpr *expr) {
  llvm::errs() << "RecoveryExpr: ";
  expr->dump();
  exit(1);
  return false;
}

bool Converter::Convert(clang::QualType qual_type) {
  // Catch va_list before desugaring
  if (IsVaListType(qual_type)) {
    StrCat("VaList");
    return false;
  }

  auto mapped = Mapper::Map(qual_type);
  if (!mapped.empty() && mapped != ignore_rule_type_) {
    StrCat(mapped);
    return false;
  }

  qual_type = qual_type.getUnqualifiedType().getDesugaredType(ctx_);
  return TraverseType(qual_type);
}

bool Converter::ConvertMappedType(clang::QualType qual_type) {
  std::string type_as_string = Mapper::Map(qual_type);
  if (type_as_string == ignore_rule_type_) {
    return false;
  }
  StrCat(type_as_string);
  return true;
}

bool Converter::VisitBuiltinType(clang::BuiltinType *type) {
  switch (type->getKind()) {
  case clang::BuiltinType::Bool:
    StrCat("bool");
    break;
  case clang::BuiltinType::Float:
    StrCat("f32");
    break;
  case clang::BuiltinType::Double:
    StrCat("f64");
    break;
  case clang::BuiltinType::Char_S:
  case clang::BuiltinType::UChar:
  case clang::BuiltinType::SChar:
    StrCat("u8");
    break;
  case clang::BuiltinType::UShort:
  case clang::BuiltinType::UInt:
  case clang::BuiltinType::ULong:
  case clang::BuiltinType::ULongLong:
  case clang::BuiltinType::Short:
  case clang::BuiltinType::Int:
  case clang::BuiltinType::Long:
  case clang::BuiltinType::LongLong:
    StrCat(std::format("{}{}", type->isSignedInteger() ? 'i' : 'u',
                       ctx_.getTypeSize(type)));
    break;
  case clang::BuiltinType::Void:
    StrCat("::libc::c_void");
    break;
  case clang::BuiltinType::UInt128:
    StrCat("u128");
    break;
  case clang::BuiltinType::Int128:
    StrCat("i128");
    break;
  default:
    // FIXME: improve error handling
    log() << "unsupported builtin type\n";
    break;
  }
  return false;
}

bool Converter::VisitRecordType(clang::RecordType *type) {
  auto *decl = type->getDecl();
  if (auto lambda = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (lambda->isLambda()) {
      if (in_function_formals_) {
        StrCat(
            ConvertFunctionPointerType(lambda->getLambdaCallOperator()
                                           ->getType()
                                           ->getAs<clang::FunctionProtoType>(),
                                       FnProtoType::LambdaCallOperator));
      } else {
        StrCat("_");
      }
      return false;
    }
  }

  StrCat(GetRecordName(decl));
  Mapper::AddRuleForUserDefinedType(decl);
  return false;
}

std::string Converter::ConvertPointer(clang::Expr *expr, int line) {
  log() << "ConvertPointer called from line " << line << '\n';
  PushExprKind push(*this, ExprKind::AddrOf);
  return ToString(expr);
}

std::string Converter::ConvertFreshPointer(clang::Expr *expr) {
  auto str = ConvertPointer(expr);
  if (isFresh()) {
    return str;
  }
  SetFresh();
  return str;
}

std::string Converter::ConvertFreshObject(clang::Expr *expr) {
  return ConvertFreshPointer(expr);
}

std::string Converter::ConvertLValue(clang::Expr *expr) {
  PushExprKind push(*this, ExprKind::LValue);
  return ToString(expr);
}

std::string Converter::ConvertRValue(clang::Expr *expr, int line) {
  log() << "ConvertRValue called from line " << line << '\n';
  PushExprKind push(*this, ExprKind::RValue);
  return ToString(expr);
}

std::string Converter::ConvertFreshRValue(clang::Expr *expr) {
  auto str = ConvertRValue(expr);
  if (!isFresh() && !expr->getType()->isVoidType() &&
      !expr->getType()->isPointerType()) {
    SetFresh();
    return std::format("({}).clone()", std::move(str));
  }
  SetFresh();
  return str;
}

std::pair<std::string, std::string>
Converter::MaterializeTemp(const std::string &binding_name,
                           clang::QualType param_type, clang::Expr *expr) {
  return {std::format("let mut {} = {} ;", binding_name, ConvertRValue(expr)),
          std::format("& mut {}", binding_name)};
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
  StrCat(']');
  return false;
}

bool Converter::VisitLValueReferenceType(clang::LValueReferenceType *type) {
  StrCat(token::kStar);
  auto pointee_type = type->getPointeeType();
  StrCat(pointee_type.isConstQualified() ? keyword::kConst : keyword_mut_);
  return Convert(pointee_type);
}

std::string
Converter::ConvertFunctionPointerType(const clang::FunctionProtoType *proto,
                                      FnProtoType kind) {
  std::string result =
      (kind == FnProtoType::LambdaCallOperator ? "impl Fn(" : "fn(");
  for (auto p_ty : proto->param_types()) {
    result += ToString(p_ty);
    result += ',';
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

  if (IsVaListType(clang::QualType(type, 0))) {
    StrCat("VaList");
    return false;
  }

  StrCat(token::kStar);
  auto pointee_type = type->getPointeeType();
  StrCat(pointee_type.isConstQualified() ? keyword::kConst : keyword_mut_);
  if (pointee_type->isRecordType() &&
      abstract_structs_.contains(GetID(pointee_type->getAsRecordDecl()))) {
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
  if (auto main_file_name = GetMainFileName(ctx_); main_file_name != "input") {
    StrCat("\n//", main_file_name + ".rs\n");
  }
  for (auto *child : decl->decls()) {
    if (IsConvertibleDecl(child) &&
        (IsInMainFile(child) || !decl_ids_.contains(GetID(child)))) {
      Convert(child);
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
  curr_function_ = decl;
  std::string function_name;
  if (decl->isMain()) {
    function_name = "main_0";
    ConvertFunctionMain(decl, function_name);
  } else if (decl->isOverloadedOperator()) {
    function_name = GetOverloadedOperator(decl);
  } else {
    function_name = GetNamedDeclAsString(decl->getCanonicalDecl());
  }
  // main_0 should be static
  if (!decl->isMain())
    ConvertFunctionQualifiers(decl);
  StrCat(decl->isConstexpr() ? keyword_const_fn_ : "", keyword_unsafe_,
         keyword::kFn, std::move(function_name));
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

  if (decl->isOverloadedOperator()) {
    switch (decl->getOverloadedOperator()) {
    case clang::OverloadedOperatorKind::OO_Less: {
      auto type = decl->getParamDecl(0)->getType().getNonReferenceType();
      if (auto cxx_record_decl = type->getAsCXXRecordDecl()) {
        ConvertOrdAndPartialOrdTraits(cxx_record_decl, decl);
        return false;
      }
      break;
    }
    default:
      assert(0 && "Unsupported out-of-line operator");
    }
  }
  return false;
}

void Converter::ConvertFunctionBody(clang::FunctionDecl *decl) {
  Convert(decl->getBody());
  if (!decl->getReturnType()->isVoidType()) {
    if (auto compound = clang::dyn_cast<clang::CompoundStmt>(decl->getBody())) {
      if (!compound->body_empty()) {
        if (!clang::isa<clang::ReturnStmt>(compound->body_back())) {
          StrCat(R"(panic!("ub: non-void function does not return a value"))");
        }
      }
    }
  }
}

bool Converter::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl *decl) {
  for (auto *function_decl : decl->specializations()) {
    VisitFunctionDecl(function_decl);
  }
  return false;
}

void Converter::ConvertVaListVarDecl(clang::VarDecl *decl) {
  if (clang::isa<clang::ParmVarDecl>(decl)) {
    // va_list parameter (decayed to __va_list_tag *)
  } else {
    // va_list local variable
    StrCat(keyword::kLet);
  }
  StrCat(keyword_mut_, GetNamedDeclAsString(decl), token::kColon, "VaList");
}

bool Converter::ConvertVarDeclSkipInit(clang::VarDecl *decl) {
  auto qual_type = decl->getType();
  auto name = GetNamedDeclAsString(decl);

  if (IsVaListType(qual_type) && decl->isLocalVarDecl()) {
    ConvertVaListVarDecl(decl);
    return true;
  }

  if (decl->isFileVarDecl()) {
    name = ReplaceAll(Mapper::ToString(decl), "::", "_");
    if (decl->isThisDeclarationADefinition() ==
            clang::VarDecl::DeclarationOnly ||
        !globals_.insert(name).second) {
      return false;
    }
    StrCat(AccessSpecifierAsString(decl->getAccess()), keyword::kStatic,
           keyword_mut_);
    ENSURE(decl_ids_.insert(GetID(decl)).second);
  } else if (decl->isStaticLocal()) {
    StrCat(keyword::kStatic, keyword_mut_);
  } else if (decl->isLocalVarDecl()) {
    StrCat(keyword::kLet);
  }

  auto *method_or_null =
      curr_function_ ? clang::dyn_cast<clang::CXXMethodDecl>(curr_function_)
                     : nullptr;
  if (!qual_type.isConstQualified() && !qual_type->isReferenceType() &&
      ((method_or_null == nullptr) || !method_or_null->isVirtual()) &&
      !IsGlobalVar(decl)) {
    StrCat(keyword_mut_);
  }

  StrCat(name, token::kColon);

  bool is_parm_with_default_value = false;
  if (auto parm = clang::dyn_cast<clang::ParmVarDecl>(decl)) {
    is_parm_with_default_value = parm->hasDefaultArg();
  }

  if (is_parm_with_default_value) {
    StrCat("Option<");
  }
  Convert(qual_type);
  if (is_parm_with_default_value) {
    StrCat('>');
  }
  return true;
}

bool Converter::ConvertLambdaVarDecl(clang::VarDecl *decl) {
  if (decl->getType()->isFunctionPointerType()) {
    return false;
  }
  if (decl->hasInit()) {
    if (clang::isa<clang::LambdaExpr>(
            decl->getInit()->IgnoreUnlessSpelledInSource())) {
      // Lambdas are inlined at the call site.
      return true;
    }
  }
  return false;
}

void Converter::ConvertVarDecl(clang::VarDecl *decl) {
  if (!ConvertVarDeclSkipInit(decl)) {
    // Skip global variables declared extern
    return;
  }
  auto qual_type = decl->getType();
  PushConstInitializer static_init(*this, decl->isFileVarDecl() ||
                                              decl->isStaticLocal());
  if (decl->hasInit()) {
    StrCat(token::kAssign);
    ConvertVarInit(qual_type, decl->getInit());
  } else if (!clang::isa<clang::ParmVarDecl>(decl)) {
    StrCat(token::kAssign, ConvertVarDefaultInit(qual_type));
  }
  StrCat(token::kSemiColon);
}

void Converter::ConvertGlobalVarDecl(clang::VarDecl *decl) {
  ConvertVarDecl(decl);
}

bool Converter::VisitVarDecl(clang::VarDecl *decl) {
  if (ConvertLambdaVarDecl(decl)) {
    return false;
  }

  if (IsGlobalVar(decl)) {
    ConvertGlobalVarDecl(decl);
  } else {
    ConvertVarDecl(decl);
  }

  return false;
}

static bool hasUserDefinedNonDefaultCopyOrMoveCtor(clang::CXXRecordDecl *decl) {
  for (const auto *ctor : decl->ctors()) {
    if (ctor->isCopyConstructor() || ctor->isMoveConstructor()) {
      auto source = ctor->getDefinition() ? ctor->getDefinition() : ctor;
      if (source->isUserProvided() && !source->isDefaulted()) {
        return true;
      }
    }
  }

  for (const auto *method : decl->methods()) {
    if (method->isCopyAssignmentOperator() ||
        method->isMoveAssignmentOperator()) {
      auto source = method->getDefinition() ? method->getDefinition() : method;
      if (source->isUserProvided() && !source->isDefaulted()) {
        return true;
      }
    }
  }

  return false;
}

void Converter::materializeTemplateSpecialization(clang::CXXRecordDecl *decl) {
  for (auto method : decl->methods()) {
    const clang::FunctionDecl *definition = nullptr;
    if (method->isDefined(definition)) {
      continue;
    }

    if (auto pattern = method->getTemplateInstantiationPattern()) {
      if (pattern->doesThisDeclarationHaveABody()) {
        sema_->InstantiateFunctionDefinition(method->getLocation(), method,
                                             /*Recursive=*/true);
      }
    }
  }
}

bool IsPointerType(clang::QualType qual_type) {
  return qual_type->isPointerType() ||
         (qual_type->isArrayType() &&
          IsPointerType(qual_type->getArrayElementTypeNoTypeQual()
                            ->getCanonicalTypeInternal()));
}

bool Converter::RecordDerivesDefault(const clang::RecordDecl *decl) {
  if (auto cxx_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (GetUserDefinedDefaultConstructor(cxx_decl)) {
      return false;
    }
  }

  for (auto f : decl->fields()) {
    // Records that contain function pointer do not derive Default
    if (auto ptr_ty = f->getType()->getAs<clang::PointerType>()) {
      if (ptr_ty->getPointeeType()->isFunctionType()) {
        return false;
      }
    }

    // Records that contain std::array do not derive Default
    if (Mapper::ToString(f->getType()).contains("std::array")) {
      return false;
    }

    // Records that contain C arrays do not derive Default
    if (f->getType()->isArrayType()) {
      return false;
    }
  }

  return true;
}

static bool recordDerivesCopy(const clang::RecordDecl *decl) {
  for (auto f : decl->fields()) {
    // Records that contain std::vector, std::array, std::string or anything
    // that is translated to Vec<>, do not derive Copy
    auto mapped = Mapper::Map(f->getType());
    if (mapped.starts_with("Vec<")) {
      return false;
    }

    if (IsUniquePtr(f->getType())) {
      return false;
    }

    if (mapped.starts_with("BTreeMap<")) {
      return false;
    }

    // Records that contain function pointer do not derive Copy
    if (auto ptr_ty = f->getType()->getAs<clang::PointerType>()) {
      if (ptr_ty->getPointeeType()->isFunctionType()) {
        return false;
      }
    }

    // Look recursively into fields that are RecordDecl
    if (auto field_record = f->getType()->getAsRecordDecl()) {
      if (!recordDerivesCopy(field_record)) {
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

  if (!record_decls_.insert(GetID(decl)).second) {
    return false;
  }

  Mapper::AddRuleForUserDefinedType(decl);
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
      VisitVarDecl(var_decl);
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
  }

  // Derived traits
  if (EmitsReprCForRecords()) {
    StrCat("#[repr(C)]");
  }
  StrCat("#[derive(");
  for (auto *attr : GetStructAttributes(decl)) {
    StrCat(attr, ',');
  }
  StrCat(")]");

  // Fields
  auto access = clang::dyn_cast<clang::CXXRecordDecl>(decl)
                    ? AccessSpecifierAsString(decl->getAccess())
                    : keyword::kPub;
  StrCat(access, decl->isUnion() ? keyword::kUnion : keyword::kStruct,
         GetRecordName(decl));
  {
    PushBrace brace(*this);
    for (auto *field : decl->fields()) {
      VisitFieldDecl(field);
    }
  }

  // C++ method decls
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    auto struct_name = GetRecordName(cxx);

    ConvertCXXMethodDecls(
        cxx, std::format("{} {}", keyword::kImpl, struct_name),
        [](auto *method) {
          return !method->isImplicit() &&
                 !(method->getDefinition() &&
                   method->getDefinition()->isDefaulted()) &&
                 (method->isThisDeclarationADefinition() ||
                  clang::isa<clang::CXXConstructorDecl>(method)) &&
                 !method->isVirtual() &&
                 !clang::isa<clang::CXXDestructorDecl>(method);
        });

    if (cxx->bases_begin() != cxx->bases_end()) {
      ConvertCXXMethodDecls(
          cxx,
          std::format("{} impl {} for {}", keyword_unsafe_,
                      GetUnsafeTypeAsString(cxx->bases_begin()->getType()),
                      struct_name),
          [](auto *method) {
            return !method->isImplicit() && method->isVirtual();
          });
    }
  }

  // Traits
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    AddOrdTrait(cxx);
    AddCloneTrait(cxx);
    AddDropTrait(cxx);
  }
  AddDefaultTrait(decl);
  AddByteReprTrait(decl);
}

bool Converter::VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
  if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    materializeTemplateSpecialization(decl);
  }

  decl->dump(log());

  Mapper::AddRuleForUserDefinedType(decl);
  if (!IsConvertibleCXXRecordDecl(decl)) {
    return false;
  }

  if (decl->isStruct() || decl->isClass()) {
    for (auto c : GetTemplateInstantiatedCtors(decl)) {
      if (!decl_ids_.contains(GetID(c))) {
        StrCat(keyword::kImpl, GetRecordName(decl));
        PushBrace brace(*this);
        VisitCXXMethodDecl(c);
      }
    }

    if (!record_decls_.insert(GetID(decl)).second) {
      return false;
    }

    if (decl->isAbstract()) {
      ConvertAbstractClass(decl);
      return false;
    }

    if (hasUserDefinedNonDefaultCopyOrMoveCtor(decl)) {
      assert(0 && "unsupported user-defined copy ctor, move ctor");
    }

    sema_->ForceDeclarationOfImplicitMembers(decl);
    for (auto ctor : decl->ctors()) {
      if (ctor->isCopyConstructor() && ctor->isImplicit() &&
          !ctor->doesThisDeclarationHaveABody() && !ctor->isDeleted()) {
        sema_->DefineImplicitCopyConstructor(decl->getLocation(), ctor);
      }
    }

    EmitRustStructOrUnion(decl);
  } else {
    // FIXME: improve error handling
    assert(0 && "unsupported record kind");
  }

  return false;
}

bool Converter::VisitCXXMethodDecl(clang::CXXMethodDecl *decl) {
  decl->dump(log());
  if (!IsConvertibleCXXMethodDecl(decl)) {
    return false;
  }
  curr_function_ = decl;

  if (decl->isOutOfLine() && !decl->overridden_methods().empty()) {
    return false;
  }
  bool out_of_line = decl->isOutOfLine();
  if (out_of_line) {
    StrCat(keyword::kImpl, GetRecordName(decl->getParent()));
  }
  PushBrace impl_brace(*this, out_of_line);

  if (auto *ctor = clang::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return VisitCXXConstructorDecl(ctor);
  }

  if (decl->isStatic() ||
      (!decl->isVirtual() && !decl->getParent()->isAbstract())) {
    ConvertFunctionQualifiers(decl);
  }
  StrCat(keyword_unsafe_, keyword::kFn);

  std::string function_name;
  if (decl->isOverloadedOperator()) {
    function_name = GetOverloadedOperator(decl);
  } else if (IsOverloadedMethod(decl)) {
    function_name = GetOverloadedFunctionName(decl);
  } else {
    function_name = GetNamedDeclAsString(decl);
  }
  StrCat(std::move(function_name));

  {
    PushParen paren(*this);
    if (!decl->isStatic()) {
      StrCat(GetSelfMaybeWithMut(decl), token::kComma);
    }
    ConvertFunctionParameters(decl);
  }
  ConvertFunctionReturnType(decl);
  if (decl->isPureVirtual()) {
    StrCat(token::kSemiColon);
  } else {
    PushBrace body(*this);
    EmitFunctionPreamble(decl);
    ConvertFunctionBody(decl);
  }
  return false;
}

std::string Converter::GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) {
  // This assumes that all overloaded comparison operators are declared const
  return (decl->isConst() || IsOverloadedComparisonOperator(decl))
             ? "&self"
             : std::format("&mut {}", keyword::kSelfValue);
}

bool Converter::VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) {
  if (decl->isOutOfLine() || decl->isImplicit()) {
    return false;
  }
  curr_function_ = decl;

  if (decl->isCopyOrMoveConstructor()) {
    // FIXME: improve error handling
    assert(0 && "user-defined copy or move constructor are not supported");
  }

  ConvertFunctionQualifiers(decl);
  auto ctor_name = GetRecordName(decl->getParent()) +
                   (GetNumberOfConvertingCtors(decl->getParent()) != 1
                        ? std::to_string(GetCtorIndex(decl))
                        : "");
  StrCat(keyword_unsafe_, keyword::kFn, ctor_name);
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
  StrCat(keyword::kLet, "mut", "this", token::kAssign, "Self");
  {
    PushBrace this_init(*this);
    const auto *record_decl = decl->getParent();
    auto *definition_or_null = decl->getDefinition();
    assert(definition_or_null);
    auto *definition =
        clang::cast<clang::CXXConstructorDecl>(definition_or_null);

    bool has_inits = !definition->inits().empty();
    auto **ctor_initializer_list = definition->inits().begin();
    int curr_init =
        has_inits ? (ctor_initializer_list[0]->isBaseInitializer() ? 1 : 0) : 0;

    for (const auto *field : record_decl->fields()) {
      auto field_name = GetNamedDeclAsString(field);
      auto field_type = field->getType();
      auto *ctor_initializer =
          has_inits ? ctor_initializer_list[curr_init] : nullptr;

      if (has_inits &&
          GetNamedDeclAsString(ctor_initializer->getMember()) == field_name) {
        auto *ctor_init_expr = ctor_initializer->getInit();
        StrCat(field_name, token::kColon);
        ConvertVarInit(field_type, ctor_init_expr);
        curr_init = (curr_init + 1) % definition->getNumCtorInitializers();
      } else {
        StrCat(field_name, token::kColon, GetDefaultAsString(field_type));
      }
      StrCat(token::kComma);
    }
  }

  StrCat(token::kSemiColon);
  Convert(decl->getBody());
  StrCat("this");
}

bool Converter::VisitFieldDecl(clang::FieldDecl *decl) {
  auto access_spec = AccessSpecifierAsString(decl->getAccess());
  auto field_name = GetNamedDeclAsString(decl);
  StrCat(access_spec, std::move(field_name), token::kColon);
  Convert(decl->getType());
  StrCat(token::kComma);
  return false;
}

void Converter::EmitFunctionPreamble(clang::FunctionDecl *decl) {
  // In the header, the function might be declared as `int foo(int name_1)',
  // while in the source file the function might be defined as `int foo(int
  // name_2)'. We want to get the parameters from the definition if possible,
  // i.e. name_2.
  auto params = decl->getDefinition() ? decl->getDefinition()->parameters()
                                      : decl->parameters();
  for (auto *param : params) {
    if (param->hasDefaultArg()) {
      auto name = GetNamedDeclAsString(param);
      auto type = ToString(param->getType());
      auto init = std::format("{}.unwrap_or({})", name,
                              ToString(param->getDefaultArg()));
      StrCat(std::format("let mut {} : {} = {}", name, type, init),
             token::kSemiColon);
    }
  }
}

bool Converter::VisitNamespaceDecl(clang::NamespaceDecl *decl) {
  for (auto *child : decl->decls()) {
    if (IsInMainFile(child) || !decl_ids_.contains(GetID(child))) {
      Convert(child);
    }
  }
  return false;
}

bool Converter::VisitTypedefDecl([[maybe_unused]] clang::TypedefDecl *decl) {
  return false;
}

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

bool Converter::VisitCompoundStmt(clang::CompoundStmt *stmt) {
  for (auto *child : stmt->body()) {
    Convert(child);
  }
  return false;
}

bool Converter::VisitDeclStmt(clang::DeclStmt *stmt) {
  for (auto *decl : stmt->decls()) {
    Convert(decl);
    StrCat(token::kSemiColon);
  }
  return false;
}

bool Converter::VisitReturnStmt(clang::ReturnStmt *stmt) {
  auto return_type = curr_function_->getReturnType();
  if (!return_type->isVoidType()) {
    StrCat(keyword::kReturn);
    ConvertVarInit(return_type, stmt->getRetValue());
  } else {
    Convert(stmt->getRetValue());
    StrCat(token::kSemiColon, keyword::kReturn, token::kSemiColon);
  }
  return false;
}

void Converter::ConvertCondition(clang::Expr *cond) {
  PushExprKind push(*this, ExprKind::RValue);
  Convert(NormalizeToBool(cond, ctx_));
}

bool Converter::VisitIfStmt(clang::IfStmt *stmt) {
  StrCat(keyword::kIf);
  ConvertCondition(stmt->getCond());
  {
    PushBrace brace(*this);
    Convert(stmt->getThen());
  }
  if (stmt->hasElseStorage()) {
    StrCat(keyword::kElse);
    if (clang::isa<clang::IfStmt>(stmt->getElse())) {
      Convert(stmt->getElse());
    } else {
      PushBrace brace(*this);
      Convert(stmt->getElse());
    }
  }
  return false;
}

bool Converter::VisitWhileStmt(clang::WhileStmt *stmt) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  StrCat("'loop_:");
  StrCat(keyword::kWhile);
  ConvertCondition(stmt->getCond());
  {
    PushBrace brace(*this);
    curr_for_inc_.emplace(nullptr);
    Convert(stmt->getBody());
    curr_for_inc_.pop();
  }
  return false;
}

bool Converter::VisitDoStmt(clang::DoStmt *stmt) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  StrCat("'loop_:");
  StrCat(keyword::kLoop);
  {
    PushBrace loop_brace(*this);
    curr_for_inc_.emplace(nullptr);
    Convert(stmt->getBody());
    curr_for_inc_.pop();
    StrCat(keyword::kIf, token::kNot);
    {
      PushParen paren(*this);
      ConvertCondition(stmt->getCond());
    }
    {
      PushBrace if_brace(*this);
      StrCat(keyword::kBreak, token::kSemiColon);
    }
  }
  return false;
}

bool Converter::VisitForStmt(clang::ForStmt *stmt) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
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
    curr_for_inc_.emplace(stmt->getInc());
    Convert(stmt->getBody());
    curr_for_inc_.pop();
    Convert(stmt->getInc());
    StrCat(token::kSemiColon);
  }
  return false;
}

void Converter::ConvertLoopVariable(clang::VarDecl *decl,
                                    clang::Expr *range_init) {
  auto loop_var_type = decl->getType();
  auto loop_var_name = GetNamedDeclAsString(decl);

  if (loop_var_type->isReferenceType()) {
    auto pointee_type = loop_var_type->getPointeeType();
    Convert(range_init);
    if (pointee_type.isConstQualified()) {
      StrCat(std::format(".as_ptr().add({})", loop_var_name));
    } else {
      StrCat(std::format(".as_mut_ptr().add({})", loop_var_name));
    }
  } else {
    {
      PushExplicitAutoref autoref(*this, /*is_mut=*/false);
      Convert(range_init);
    }
    StrCat(std::format("[{}]", loop_var_name));
    StrCat(".clone()");
  }
}

void Converter::ConvertForRangeBody(clang::CXXForRangeStmt *stmt,
                                    const clang::VarDecl *map_iter_decl) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  std::optional<ScopedMapIterDecl> skip;
  if (map_iter_decl)
    skip.emplace(*this, map_iter_decl);
  curr_for_inc_.emplace(nullptr);
  Convert(stmt->getBody());
  curr_for_inc_.pop();
}

bool Converter::VisitCXXForRangeStmt(clang::CXXForRangeStmt *stmt) {
  auto range_init_type = stmt->getRangeInit()->getType();

  if (!Mapper::Contains(range_init_type.getUnqualifiedType())) {
    // FIXME: improve error handling
    log() << "for range stmts only for types in std namespace\n";
  }

  log() << "GetClassName: " << GetClassName(range_init_type) << '\n';

  if (GetClassName(range_init_type) == "std::map") {
    return VisitCXXForRangeStmtMap(stmt);
  }
  if (GetClassName(range_init_type) == "std::basic_string") {
    return VisitCXXForRangeStmtString(stmt);
  }
  return VisitCXXForRangeStmtVector(stmt);
}

bool Converter::VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  StrCat("'loop_:");
  auto map_type = Mapper::Map(stmt->getRangeInit()->getType());
  StrCat(keyword::kFor, loop_var_name, keyword::kIn,
         "UnsafeMapIterator::begin(&");
  Convert(stmt->getRangeInit());
  StrCat(std::format(" as *const {})", map_type));
  {
    PushBrace brace(*this);
    ConvertForRangeBody(stmt, loop_var);
  }

  return false;
}

bool Converter::VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt) {
  return VisitCXXForRangeStmtIndexBased(stmt, "len()-1");
}

bool Converter::VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt) {
  return VisitCXXForRangeStmtIndexBased(stmt, "len()");
}

bool Converter::VisitCXXForRangeStmtIndexBased(clang::CXXForRangeStmt *stmt,
                                               const char *len_suffix) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  StrCat("'loop_:");
  StrCat(keyword::kFor, loop_var_name, keyword::kIn, "0..");
  {
    PushParen range(*this);
    Convert(stmt->getRangeInit());
    StrCat(token::kDot, len_suffix);
  }
  {
    PushBrace body(*this);
    StrCat(keyword::kLet);

    auto loop_var_type = loop_var->getType();
    if (!loop_var_type.isConstQualified()) {
      StrCat(keyword_mut_);
    }

    StrCat(loop_var_name);
    StrCat(token::kAssign);

    ConvertLoopVariable(loop_var, stmt->getRangeInit());

    StrCat(token::kSemiColon);
    ConvertForRangeBody(stmt);
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
    Convert(curr_for_inc_.top());
    StrCat(token::kSemiColon);
  }
  StrCat(keyword::kContinue);
  StrCat("'loop_");
  return false;
}

bool Converter::Convert(clang::Expr *expr) { return TraverseStmt(expr); }

const clang::Expr *Converter::GetParentExpr(const clang::Expr *expr) {
  if (!expr) {
    return nullptr;
  }
  auto parents = ctx_.getParentMapContext().getParents(*expr);
  if (!parents.empty()) {
    auto parent_node = *parents.begin();
    if (auto parent_stmt = parent_node.get<clang::Stmt>()) {
      return dyn_cast<clang::Expr>(parent_stmt);
    }
  }
  return nullptr;
}

bool Converter::IsSubExprOf(const clang::Expr *sub_expr,
                            const clang::Expr *parent_expr) {
  if (sub_expr == nullptr || parent_expr == nullptr)
    return false;

  if (parent_expr == sub_expr)
    return true;

  for (auto *child : parent_expr->children()) {
    if (auto *child_expr = llvm::dyn_cast<clang::Expr>(child)) {
      if (IsSubExprOf(sub_expr, child_expr))
        return true;
    }
  }

  return false;
}

bool Converter::GetFmtArg(clang::Expr *arg, std::string &fmt,
                          std::string &fmt_args, std::string &fmt_trait,
                          std::string &fmt_width) {
  std::string arg_str = Mapper::ToString(arg);
  if (clang::isa<clang::StringLiteral>(arg->IgnoreImplicit())) {
    auto str = GetEscapedStringLiteral(arg);
    // Delete " from string
    str.erase(std::remove(str.begin(), str.end(), '"'), str.end());
    fmt += std::move(str);
  } else if (auto ch = GetEscapedUTF8CharLiteral(arg); !ch.empty()) {
    fmt += std::move(ch);
  } else if (arg_str.contains("std::endl")) {
    fmt += "\\n";
  } else if (arg_str.contains("std::hex")) {
    fmt_trait = 'x';
  } else if (arg_str.contains("std::dec")) {
    fmt_trait = "";
  } else if (arg_str.contains("Setw")) {
    fmt_width = ToString(arg);
    // Delete leading and trailing whitespaces
    fmt_width.erase(0, fmt_width.find_first_not_of(' '));
    fmt_width.erase(fmt_width.find_last_not_of(' ') + 1);
  } else if (!arg->getType()->isCharType() &&
             Mapper::Map(arg->getType()) != "Vec<u8>") {
    fmt += ("{:" + fmt_width + fmt_trait + "}");
    fmt_width.clear(); // Reset setw after first usage
    arg_str = ToString(arg);
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
    raw_args += "(&[" + ToString(arg) + ']';
  } else if (Mapper::Map(arg->getType()) == "Vec<u8>") {
    PushExprKind push(*this, ExprKind::RValue);
    std::string str = ToString(arg);
    raw_args += "(&(" + str + ")[..(" + str + ").len() - 1]";
  } else if (Mapper::ToString(arg).contains("std::endl")) {
    raw_args += "(&[b'\\n']";
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
  std::string fmt_trait;
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

  assert(fmt_trait == "" && "Stream state was not restored after call");
}

void Converter::ConvertPrintf(clang::CallExpr *expr) {
  bool is_fprintf =
      Mapper::ToString(expr->getCallee()).starts_with("int fprintf");

  StrCat("printf(");
  for (unsigned i = is_fprintf; i < expr->getNumArgs(); ++i) {
    if (i == is_fprintf ? 1 : 0) {
      Convert(expr->getArg(i));
      StrCat("as *const i8");
    } else {
      Convert(expr->getArg(i));
    }
    StrCat(token::kComma);
  }
  StrCat(')');
}

std::optional<std::string> Converter::TryPluginConvert(clang::CallExpr *call) {
  if (emplace_back_plugin_match(call)) {
    Buffer buf(*this);
    emplace_back_plugin_convert(call);
    return std::move(buf).str();
  }
  return std::nullopt;
}

void Converter::ConvertVAArgCall(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr)) {
    StrCat(ToString(expr->getArg(0)->IgnoreImpCasts()),
           "= VaList::new(__args)");
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
  if (IsBuiltinVaStart(expr) || IsBuiltinVaEnd(expr) || IsBuiltinVaCopy(expr)) {
    ConvertVAArgCall(expr);
    return false;
  }

  if (auto plugin_str = TryPluginConvert(expr)) {
    StrCat(*plugin_str);
    return false;
  }

  if (Mapper::Contains(expr->getCallee())) {
    auto **args = expr->getArgs();
    auto num_args = expr->getNumArgs();
    auto ctx = CollectPrvalueToLRefArgs(expr);
    std::string str;
    {
      PushExprKind push(*this, ExprKind::RValue);
      str = GetMappedAsString(expr, args, num_args, &ctx);
    };

    if ((IsReferenceType(expr) ||
         GetReturnTypeOfFunction(expr)->isReferenceType()) &&
        !isAddrOf() && !isVoid()) {
      str = "( * " + std::move(str) + " )";
    }

    if (!ctx.temporary_bindings.empty()) {
      str = std::format("{{ {} {} }}", ctx.temporary_bindings, str);
    }

    StrCat(str);
    return false;
  }

  if (expr->isCallToStdMove()) {
    StrCat(std::format("{}", ToString(expr->getArg(0))));
    computed_expr_type_ = ComputedExprType::FreshValue;
    return false;
  }

  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
      opcall && !Mapper::Contains(expr->getCallee())) {
    return ConvertCXXOperatorCallExpr(opcall);
  }

  std::string str;
  {
    Buffer buf(*this);
    Converter::ConvertCallExpr(expr);
    str = std::move(buf).str();
  }

  auto ty = GetReturnTypeOfFunction(expr);
  auto ref = clang::dyn_cast<clang::ReferenceType>(ty);

  if (ref && !isAddrOf() && !isVoid()) {
    {
      PushParen paren(*this);
      StrCat(GetPointerDerefPrefix(ref->getPointeeType()), str);
    }
    SetValueFreshness(ref->getPointeeType());
    return false;
  }

  StrCat(str);
  return false;
}

void Converter::EmitFnPtrCall(clang::Expr *callee) {
  {
    PushParen paren(*this);
    Convert(callee);
  }
  StrCat(".unwrap()");
}

void Converter::ConvertFunctionToFunctionPointer(
    const clang::FunctionDecl *fn_decl) {
  StrCat(std::format("Some({})", Mapper::MapFunctionName(fn_decl)));
}

void Converter::ConvertGenericCallExpr(clang::CallExpr *expr) {
  clang::Expr *callee = expr->getCallee();
  auto convert_param_ty = [&](clang::QualType param_type, clang::Expr *expr) {
    if (param_type->isLValueReferenceType()) {
      PushExprKind push(*this, ExprKind::AddrOf);
      ConvertVarInit(param_type, expr);
    } else {
      ConvertVarInit(param_type, expr);
    }
  };

  unsigned arg_begin = 0; // skip count for operator()'s implicit object arg
  if (auto op_call = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (op_call->getOperator() == clang::OO_Call) {
      callee = op_call->getArg(0);
      arg_begin = 1;
    }
  }

  PushParen outer(*this);
  StrCat(keyword_unsafe_);
  PushBrace unsafe_brace(*this);
  const auto *function =
      expr->getCalleeDecl() ? expr->getCalleeDecl()->getAsFunction() : nullptr;
  const clang::FunctionProtoType *proto = nullptr;

  if (!function) {
    auto callee_ty = callee->getType().getDesugaredType(ctx_);
    if (auto ptr_ty = callee_ty->getAs<clang::PointerType>()) {
      proto = ptr_ty->getPointeeType()->getAs<clang::FunctionProtoType>();
    }
  }

  assert((function || proto) &&
         "Either function decl or function prototype should be known");

  auto num_args = expr->getNumArgs() - arg_begin;
  bool is_variadic =
      function ? function->isVariadic() : (proto && proto->isVariadic());
  unsigned num_named_params = function
                                  ? function->getNumParams()
                                  : (proto ? proto->getNumParams() : num_args);

  // Track which args are materialized temps bound to reference params
  std::vector<std::string> temp_refs(num_args);

  for (unsigned i = 0; i < num_named_params && i < num_args; ++i) {
    auto *arg = expr->getArg(i + arg_begin);
    std::string param_name = function
                                 ? function->getParamDecl(i)->getNameAsString()
                                 : ("arg" + std::to_string(i));
    clang::QualType param_type = function ? function->getParamDecl(i)->getType()
                                          : proto->getParamType(i);

    bool is_materialize_to_ref =
        clang::isa<clang::MaterializeTemporaryExpr>(arg) &&
        param_type->isLValueReferenceType();

    if (is_materialize_to_ref) {
      auto [binding, ref] =
          MaterializeTemp(std::format("_{}", param_name), param_type, arg);
      StrCat(binding);
      temp_refs[i] = std::move(ref);
    } else if (!clang::isa<clang::MaterializeTemporaryExpr>(arg)) {
      StrCat("let", std::format("_{}: {}", param_name, ToString(param_type)),
             "=");
      convert_param_ty(param_type, arg);
      StrCat(";");
    }
  }

  if (proto && !function) {
    EmitFnPtrCall(callee);
  } else {
    PushExprKind push(*this, ExprKind::Callee);
    Convert(callee);
  }
  {
    PushParen call_args(*this);
    for (unsigned i = 0; i < num_named_params && i < num_args; ++i) {
      auto *arg = expr->getArg(i + arg_begin);
      std::string param_name =
          function ? function->getParamDecl(i)->getNameAsString()
                   : ("arg" + std::to_string(i));
      clang::QualType param_type = function
                                       ? function->getParamDecl(i)->getType()
                                       : proto->getParamType(i);
      bool is_parm_with_default_value =
          function && function->getParamDecl(i)->hasDefaultArg();

      if (is_parm_with_default_value) {
        StrCat("Some(");
      }
      if (!temp_refs[i].empty()) {
        StrCat(temp_refs[i]);
      } else if (clang::isa<clang::MaterializeTemporaryExpr>(arg)) {
        convert_param_ty(param_type, arg);
      } else {
        StrCat(std::format("_{}", param_name));
      }
      if (is_parm_with_default_value) {
        StrCat(')');
      }
      StrCat(token::kComma);
    }

    // Variadic args: wrap in &[arg.into(), ...]
    if (is_variadic) {
      StrCat("& [");
      for (unsigned i = num_named_params; i < num_args; ++i) {
        auto *arg = expr->getArg(i + arg_begin);
        Convert(arg);
        StrCat(".into()", token::kComma);
      }
      StrCat(']');
    }
  }
}

std::optional<Converter::TempMaterializationCtx>
Converter::ConvertCallExpr(clang::CallExpr *expr) {
  auto *callee = expr->getCallee();

  if (auto fn = Mapper::ToString(callee);
      fn.starts_with("int printf") || fn.starts_with("int fprintf")) {
    ConvertPrintf(expr);
  } else if (expr->isCallToStdMove()) {
    Convert(expr->getArg(0));
  } else if (IsBuiltinConstantP(callee)) {
    StrCat(expr->getArg(0)->isCXX11ConstantExpr(ctx_) ? token::kOne
                                                      : token::kZero);
  } else if (Mapper::Contains(callee)) {
    auto **args = expr->getArgs();
    auto num_args = expr->getNumArgs();
    auto ctx = CollectPrvalueToLRefArgs(expr);
    StrCat(GetMappedAsString(expr, args, num_args, &ctx));
    return ctx;
  } else if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    ConvertCXXOperatorCallExpr(opcall);
  } else {
    ConvertGenericCallExpr(expr);
  }
  return std::nullopt;
}

std::string Converter::getIntegerLiteral(clang::IntegerLiteral *expr,
                                         bool incl_type,
                                         const clang::QualType *type) {
  auto num_as_string = GetNumAsString(expr->getValue());
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
    return std::format("{}_{}", num_as_string.c_str(), type_as_string);
  }

  return static_cast<std::string>(num_as_string);
}

bool Converter::VisitIntegerLiteral(clang::IntegerLiteral *expr) {
  if (auto str = GetMappedAsString(expr); !str.empty()) {
    StrCat(str);
    computed_expr_type_ = ComputedExprType::FreshValue;
    return false;
  }
  StrCat(getIntegerLiteral(expr, Mapper::Map(expr->getType()) != "i32"));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return false;
}

bool Converter::VisitFloatingLiteral(clang::FloatingLiteral *expr) {
  StrCat(GetNumAsString(expr->getValue()));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return false;
}

bool Converter::VisitCharacterLiteral(clang::CharacterLiteral *expr) {
  std::string ch = GetEscapedCharLiteral(expr->getValue());
  ch = "'" + std::move(ch) + "'";
  {
    PushParen paren(*this);
    StrCat(ch, keyword::kAs, ToStringBase(expr->getType()));
  }
  computed_expr_type_ = ComputedExprType::FreshValue;
  return false;
}

std::string Converter::GetEscapedCharLiteral(char character) const {
  switch (character) {
  case '"':
    return "\\\"";
  case '\'':
    return "\\'";
  case '\\':
    return "\\\\";
  case '\n':
    return "\\n";
  case '\r':
    return "\\r";
  case '\t':
    return "\\t";
  case '\0':
    return "\\0";
  }
  auto uc = static_cast<unsigned char>(character);
  if (uc < 0x20 || uc == 0x7F) {
    return std::format("\\x{:02x}", uc);
  }
  return std::string(1, character);
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

bool Converter::VisitStringLiteral(clang::StringLiteral *expr) {
  if (!curr_init_type_.empty() && curr_init_type_.top()->isArrayType()) {
    if (auto *arr_ty = ctx_.getAsConstantArrayType(curr_init_type_.top())) {
      uint64_t arr_size = arr_ty->getSize().getZExtValue();
      if (expr->getString().empty()) {
        StrCat(std::format("[0u8; {}]", arr_size));
        return false;
      }
      uint64_t pad = arr_size > expr->getString().size()
                         ? arr_size - expr->getString().size()
                         : 0;
      StrCat(token::kStar,
             std::format("b{}", GetEscapedStringLiteral(expr, pad)));
      return false;
    }
    StrCat(token::kStar);
  }
  StrCat(std::format("b{}", GetEscapedStringLiteral(expr, 1)));
  return false;
}

bool Converter::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr *expr) {
  StrCat(expr->getValue() ? keyword::kTrue : keyword::kFalse);
  computed_expr_type_ = ComputedExprType::FreshValue;
  return false;
}

void Converter::ConvertIntegerToEnumeralCast(clang::Expr *to,
                                             clang::Expr *from) {
  // Short circuit `Enum::from(X as i32)` to `X`
  if (auto ref =
          clang::dyn_cast<clang::DeclRefExpr>(from->IgnoreParenImpCasts())) {
    if (auto ec = clang::dyn_cast<clang::EnumConstantDecl>(ref->getDecl())) {
      auto src_enum = clang::dyn_cast<clang::EnumDecl>(ec->getDeclContext());
      auto dst_enum = to->getType()->getAs<clang::EnumType>();
      if (src_enum && dst_enum && dst_enum->getDecl() == src_enum) {
        StrCat(std::format("{}::{}", GetRecordName(src_enum),
                           std::string_view(ec->getName())));
        return;
      }
    }
  }
  StrCat(GetUnsafeTypeAsString(to->getType()), "::from");
  PushParen paren(*this);
  Convert(from);
  if (!from->getType()->isSpecificBuiltinType(clang::BuiltinType::Int)) {
    StrCat(keyword::kAs, "i32");
  }
}

void Converter::ConvertIntegralToBooleanCast(clang::ImplicitCastExpr *expr) {
  auto sub_expr = expr->getSubExpr();
  auto *stripped = sub_expr->IgnoreParenImpCasts();

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
  if (sub_expr->getType()->isEnumeralType()) {
    StrCat(GetUnsafeTypeAsString(sub_expr->getType()), "::from(0)");
  } else /* sub_expr->getType()->isIntegerType() */ {
    StrCat(token::kZero);
  }
}

bool Converter::IsCastRedundantInRust(clang::Expr *expr,
                                      clang::QualType target_type) {
  auto target = GetUnsafeTypeAsString(target_type);
  if (const auto *rule = Mapper::GetExprRule(expr)) {
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
    SetValueFreshness(type);
    break;
  }
  case clang::CastKind::CK_ArrayToPointerDecay: {
    // __va_list_tag [1] decays to __va_list_tag *. Just pass through by value
    if (IsVaListType(sub_expr->getType())) {
      Convert(sub_expr);
      break;
    }
    Convert(sub_expr);
    bool dest_pointee_const =
        expr->getType()->getPointeeType().isConstQualified();
    if (clang::isa<clang::StringLiteral>(sub_expr) ||
        clang::isa<clang::PredefinedExpr>(sub_expr)) {
      StrCat(".as_ptr()");
      if (!dest_pointee_const) {
        StrCat(".cast_mut()");
      }
    } else {
      StrCat(dest_pointee_const ? ".as_ptr()" : ".as_mut_ptr()");
    }
    break;
  }
  case clang::CastKind::CK_BitCast: {
    PushParen paren(*this);
    Convert(sub_expr);
    if (type->isVoidPointerType()) {
      StrCat(keyword::kAs,
             type->getPointeeType().isConstQualified() ? "*const" : "*mut");
      Convert(sub_expr->getType()->getPointeeType());
    }
    ConvertCast(type);
    break;
  }
  case clang::CastKind::CK_NoOp: {
    Convert(sub_expr);
    if (expr->getType()->isPointerType() &&
        sub_expr->getType()->isPointerType() &&
        !clang::isa<clang::CXXThisExpr>(expr->IgnoreImplicit())) {
      switch (GetConstCastType(expr->getType()->getPointeeType(),
                               sub_expr->getType()->getPointeeType())) {
      case ConstCastType::MutableToConst:
        StrCat(".cast_const()");
        break;
      case ConstCastType::ConstToMutable:
        StrCat(".cast_mut()");
        break;
      default:
        break;
      }
    }
    break;
  }
  case clang::CastKind::CK_FunctionToPointerDecay:
  case clang::CastKind::CK_BuiltinFnToFnPtr: {
    if (isCallee()) {
      Convert(sub_expr);
    } else {
      PushExprKind push(*this, ExprKind::AddrOf);
      Convert(sub_expr);
    }
    break;
  }
  case clang::CastKind::CK_ConstructorConversion:
  case clang::CastKind::CK_DerivedToBase:
    Convert(sub_expr);
    break;
  case clang::CastKind::CK_IntegralToBoolean:
    ConvertIntegralToBooleanCast(expr);
    break;
  case clang::CastKind::CK_PointerToBoolean:
    StrCat(token::kNot);
    ConvertEqualsNullPtr(sub_expr);
    break;
  case clang::CastKind::CK_NullToPointer:
    StrCat(GetDefaultAsString(type));
    computed_expr_type_ = ComputedExprType::FreshPointer;
    break;
  default:
    if (auto *literal = clang::dyn_cast<clang::IntegerLiteral>(sub_expr)) {
      auto type = expr->getType();
      StrCat(getIntegerLiteral(literal, true, &type));
      computed_expr_type_ = ComputedExprType::FreshValue;
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
      if (clang::isa<clang::BinaryOperator>(sub_expr)) {
        {
          PushParen inner(*this);
          Convert(sub_expr);
        }
        ConvertCast(type);
      } else {
        PushParen inner(*this);
        Convert(sub_expr);
        ConvertCast(type);
      }
    }
  }
  return false;
}

bool Converter::VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
  auto type = expr->getTypeAsWritten();
  auto *sub_expr = expr->getSubExpr();
  if (type->isVoidType()) {
    PushExprKind push(*this, ExprKind::Void);
    Convert(expr->getSubExpr());
    return false;
  }
  switch (expr->getStmtClass()) {
  case clang::Stmt::CXXReinterpretCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass:
  case clang::Stmt::CStyleCastExprClass:
    if (expr->getType() == sub_expr->getType()) {
      return Convert(sub_expr);
    }
    if (type->isFunctionPointerType() ||
        sub_expr->getType()->isFunctionPointerType()) {
      StrCat("std::mem::transmute::<");
      Convert(sub_expr->getType());
      StrCat(',');
      Convert(type);
      StrCat(">(");
      Convert(sub_expr);
      StrCat(')');
      return false;
    }
    if (type->isEnumeralType() && !sub_expr->getType()->isEnumeralType()) {
      ConvertIntegerToEnumeralCast(expr, sub_expr);
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

bool Converter::VisitBinaryOperator(clang::BinaryOperator *expr) {
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
      lhs_type != cmpd_assign_op->getComputationResultType()) {
    auto computation_result_type = cmpd_assign_op->getComputationResultType();
    if (IsUnsignedArithOp(cmpd_assign_op)) {
      Convert(lhs);
      StrCat(token::kAssign);
      PushParen outer(*this);
      {
        PushParen inner(*this);
        Convert(lhs);
        ConvertCast(computation_result_type);
      }
      ConvertUnsignedArithBinaryOperator(expr, rhs);
    } else {
      Convert(lhs);
      StrCat(token::kAssign);
      PushParen outer(*this);
      {
        PushParen inner(*this);
        Convert(lhs);
        ConvertCast(computation_result_type);
      }
      std::string op(opcode_as_string);
      op.erase(std::remove(op.begin(), op.end(), '='), op.end());
      StrCat(op);
      Convert(rhs);
    }
    if (lhs_type->isBooleanType()) {
      StrCat(token::kDiff, token::kZero);
    } else {
      ConvertCast(lhs_type);
    }
  } else if (expr->isCommaOp()) {
    {
      PushExprKind push(*this, ExprKind::Void);
      Convert(lhs);
    }
    StrCat(token::kSemiColon);
    Convert(rhs);
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
  } else if (expr->isAssignmentOp()) {
    if (expr->isCompoundAssignmentOp() &&
        expr->getLHS()->getType()->isPointerType() &&
        expr->getRHS()->getType()->isIntegralOrEnumerationType()) {
      Convert(lhs);
      StrCat(token::kAssign);
      {
        PushParen paren(*this);
        ConvertUnsignedArithOperand(lhs, type);
      }
      ConvertUnsignedArithBinaryOperator(expr, rhs);
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
      auto pointee_type_as_string = ToString(lhs_type->getPointeeType());
      auto size_of_as_string =
          std::format("::std::mem::size_of::<{}>()", pointee_type_as_string);
      StrCat(size_of_as_string);
    }
    ConvertCast(expr->getType());
    computed_expr_type_ = ComputedExprType::FreshValue;
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
  } else {
    ConvertGenericBinaryOperator(expr);
  }
}

void Converter::ConvertGenericBinaryOperator(clang::BinaryOperator *expr) {
  PushParen outer(*this);
  {
    PushParen lhs_paren(*this);
    Convert(expr->getLHS());
  }

  StrCat(expr->getOpcodeStr());

  {
    PushParen rhs_paren(*this);
    Convert(expr->getRHS());
  }
}

bool Converter::IsReferenceType(const clang::Expr *expr) const {
  const auto *e = expr->IgnoreCasts();
  if (const auto *call = clang::dyn_cast<clang::CallExpr>(e)) {
    return !clang::isa<clang::CXXOperatorCallExpr>(call) &&
           GetReturnTypeOfFunction(call)->isReferenceType();
  }
  if (const auto *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(e)) {
    return decl_ref->getDecl()->getType()->isReferenceType();
  }
  if (const auto *member = clang::dyn_cast<clang::MemberExpr>(e)) {
    return member->getMemberDecl()->getType()->isReferenceType();
  }
  return false;
}

bool Converter::ConvertIncAndDec(clang::UnaryOperator *expr) {
  auto opcode = expr->getOpcode();
  auto *sub_expr = expr->getSubExpr();
  switch (opcode) {
  case clang::UO_PostInc: {
    PushExprKind push(*this, ExprKind::RValue);
    Convert(sub_expr);
    StrCat(".postfix_inc()");
    SetFresh();
    return true;
  }
  case clang::UO_PostDec: {
    PushExprKind push(*this, ExprKind::RValue);
    Convert(sub_expr);
    StrCat(".postfix_dec()");
    SetFresh();
    return true;
  }
  case clang::UO_PreInc: {
    PushExprKind push(*this, ExprKind::RValue);
    Convert(sub_expr);
    StrCat(".prefix_inc()");
    SetFresh();
    return true;
  }
  case clang::UO_PreDec: {
    PushExprKind push(*this, ExprKind::RValue);
    Convert(sub_expr);
    StrCat(".prefix_dec()");
    SetFresh();
    return true;
  }
  default:
    return false;
  }
  std::unreachable();
}

bool Converter::VisitUnaryOperator(clang::UnaryOperator *expr) {
  if (auto str = GetMappedAsString(expr); !str.empty()) {
    StrCat(str);
    return false;
  }

  auto opcode = expr->getOpcode();
  auto *sub_expr = expr->getSubExpr();
  if (ConvertIncAndDec(expr)) {
    return false;
  }
  switch (opcode) {
  case clang::UO_Extension:
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
    computed_expr_type_ = ComputedExprType::FreshValue;
    break;
  case clang::UO_LNot: {
    bool needs_int_cast =
        expr->getType()->isIntegerType() && !expr->getType()->isBooleanType();
    PushParen paren_cast(*this, needs_int_cast);
    StrCat(token::kNot);
    ConvertCondition(sub_expr);
    if (needs_int_cast) {
      ConvertCast(expr->getType());
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
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
      computed_expr_type_ = ComputedExprType::FreshValue;
      break;
    }
    [[fallthrough]];
  default:
    StrCat(expr->getOpcodeStr(opcode));
    Convert(sub_expr);
  }
  return false;
}

bool Converter::VisitStmtExpr(clang::StmtExpr *expr) {
  auto *body = expr->getSubStmt();
  PushBrace brace(*this);
  auto stmts = body->body();
  size_t n = static_cast<size_t>(stmts.end() - stmts.begin());
  size_t i = 0;
  for (auto *s : stmts) {
    ++i;
    if (i == n) {
      if (auto *tail = clang::dyn_cast<clang::Expr>(s)) {
        EmitStmtExprTail(tail);
        continue;
      }
    }
    Convert(s);
  }
  return false;
}

void Converter::EmitStmtExprTail(clang::Expr *tail) { Convert(tail); }

bool Converter::VisitConditionalOperator(clang::ConditionalOperator *expr) {
  StrCat(keyword::kIf);
  ConvertCondition(expr->getCond());
  bool branch_is_addr =
      expr->isLValue() && !isRValue() && !expr->getType()->isFunctionType();
  {
    PushBrace then_brace(*this);
    if (branch_is_addr) {
      StrCat(token::kRef, keyword_mut_);
    }
    PushExplicitAutoref no_autoref(*this, branch_is_addr ? std::nullopt
                                                         : autoref_mut_);
    Convert(expr->getTrueExpr());
  }
  StrCat(keyword::kElse);
  {
    PushBrace else_brace(*this);
    if (branch_is_addr) {
      StrCat(token::kRef, keyword_mut_);
    }
    PushExplicitAutoref no_autoref(*this, branch_is_addr ? std::nullopt
                                                         : autoref_mut_);
    Convert(expr->getFalseExpr());
  }
  return false;
}

std::string Converter::ConvertDeclRefExpr(clang::DeclRefExpr *expr) {
  if (isAddrOf()) {
    clang::Expr *addrof_op = ToAddrOf(ctx_, expr);
    if (auto str = GetMappedAsString(addrof_op); !str.empty()) {
      return str;
    }
  }

  auto *decl = expr->getDecl();
  if (ShouldReplaceWithMappedBody(expr)) {
    if (auto str = GetMappedAsString(expr); !str.empty()) {
      return str;
    }
  }

  if (auto *function = decl->getAsFunction()) {
    if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(function)) {
      if (method->isStatic()) {
        return std::format("{}::{}", GetRecordName(method->getParent()),
                           GetNamedDeclAsString(method));
      }
    }
    return GetNamedDeclAsString(function->getCanonicalDecl());
  }

  if (auto enum_constant = clang::dyn_cast<clang::EnumConstantDecl>(decl)) {
    auto qualified = std::format("{}::{}",
                                 GetRecordName(clang::dyn_cast<clang::EnumDecl>(
                                     enum_constant->getDeclContext())),
                                 std::string_view(enum_constant->getName()));
    if (!expr->getType()->isEnumeralType()) {
      return std::format("({} as i32)", qualified);
    }
    return qualified;
  }

  if (IsGlobalVar(expr)) {
    return ReplaceAll(Mapper::ToString(expr->getDecl()), "::", "_");
  }

  return GetNamedDeclAsString(decl);
}

bool Converter::VisitDeclRefExpr(clang::DeclRefExpr *expr) {
  auto str = ConvertDeclRefExpr(expr);
  auto decl = expr->getDecl();

  if (decl->getType()->getAs<clang::ReferenceType>() && !isAddrOf() &&
      !map_iter_decls_.contains(clang::dyn_cast<clang::VarDecl>(decl))) {
    EmitDeref(std::move(str), decl->getType().getNonReferenceType());
    SetValueFreshness(expr->getType());
    return false;
  }

  if (auto *fn_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (isAddrOf()) {
      ConvertFunctionToFunctionPointer(fn_decl);
      return false;
    }
  }

  if (auto var_decl = clang::dyn_cast<clang::VarDecl>(decl)) {
    if (!var_decl->getType()->isFunctionPointerType()) {
      if (auto init = var_decl->getInit()) {
        if (auto lambda = clang::dyn_cast<clang::LambdaExpr>(
                init->IgnoreUnlessSpelledInSource())) {
          PushParen paren(*this);
          VisitLambdaExpr(lambda);
          return false;
        }
      }
    }
  }

  if (!decl->getType()->getAs<clang::ReferenceType>() && isAddrOf()) {
    StrCat(token::kRef, decl->getType().isConstQualified() ? "" : keyword_mut_,
           str);
    return false;
  }

  StrCat(str);
  return false;
}

bool Converter::VisitParenExpr(clang::ParenExpr *expr) {
  // Comma operator becomes (A, B, C) -> { A; B; C }
  if (auto *bin = clang::dyn_cast<clang::BinaryOperator>(expr->getSubExpr())) {
    if (bin->isCommaOp()) {
      PushBrace push(*this);
      Convert(expr->getSubExpr());
      return false;
    }
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
  case clang::OverloadedOperatorKind::OO_Star:
  case clang::OverloadedOperatorKind::OO_Arrow:
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      ConvertUniquePtrDeref(expr);
    } else if (GetStrongestIteratorCategory(expr->getArg(0)->getType()) ==
               IteratorCategory::Bidirectional) {
      Convert(expr->getArg(0));
    } else if (expr->getOperator() == clang::OverloadedOperatorKind::OO_Star) {
      PushParen paren(*this);
      StrCat(token::kStar);
      Convert(expr->getArg(0));
    } else {
      Convert(expr->getArg(0));
    }
    break;
  case clang::OverloadedOperatorKind::OO_Subscript: {
    PushExplicitAutoref autoref(*this, IsMutatingCall(expr));
    ConvertArraySubscript(expr->getArg(0), expr->getArg(1), expr->getType());
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
    computed_expr_type_ = ComputedExprType::FreshValue;
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
  std::string str;
  {
    Buffer buf(*this);
    Converter::ConvertMemberExpr(expr);
    str = std::move(buf).str();
  }

  if (isAddrOf()) {
    bool is_reference_type = member->getType()->isReferenceType();
    if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member)) {
      is_reference_type |= method->getReturnType()->isReferenceType();
    }

    if (is_reference_type) {
      computed_expr_type_ = ComputedExprType::Pointer;
    } else {
      StrCat(token::kRef);
      computed_expr_type_ = ComputedExprType::FreshPointer;
    }
    StrCat(str);
    return false;
  }

  if (!isAddrOf() && member->getType()->isReferenceType()) {
    EmitDeref(std::move(str), member->getType().getNonReferenceType());
    return false;
  }

  if (!isAddrOf() && member->getType()->isFunctionPointerType()) {
    PushParen paren(*this);
    StrCat(str);
    return false;
  }

  StrCat(str);
  return false;
}

void Converter::ConvertMemberExpr(clang::MemberExpr *expr) {
  if (auto mapped = GetMappedAsString(expr); !mapped.empty()) {
    if (Mapper::ReturnsPointer(expr)) {
      StrCat(token::kStar, mapped);
    } else {
      StrCat(mapped);
    }
    return;
  }

  auto *member = expr->getMemberDecl();
  auto *base = expr->getBase();
  bool base_is_this = clang::isa<clang::CXXThisExpr>(base->IgnoreCasts());
  PushExprKind push(*this, isLValue() ? ExprKind::LValue : ExprKind::RValue);
  if (expr->isArrow() && !base_is_this) {
    ConvertArrow(base);
  } else {
    Convert(base);
  }

  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && IsOverloadedMethod(method)) {
    StrCat(token::kDot);
    StrCat(GetOverloadedFunctionName(method));
  } else {
    if (member->getDeclName().isIdentifier()) {
      StrCat(token::kDot);
      StrCat(GetNamedDeclAsString(member));
    }
  }
}

bool Converter::VisitCXXThisExpr([[maybe_unused]] clang::CXXThisExpr *expr) {
  if (clang::isa<clang::CXXConstructorDecl>(curr_function_)) {
    StrCat("this");
  } else {
    StrCat(keyword::kSelfValue);
  }
  return false;
}

bool Converter::VisitInitListExpr(clang::InitListExpr *expr) {
  if (auto form = expr->getSemanticForm())
    expr = form;

  auto qual_type = expr->getType();
  if (qual_type->isScalarType()) {
    assert(expr->getNumInits() < 2 && "Excess elements in scalar initializer");
    if (expr->getNumInits() > 0) {
      auto init = expr->getInit(0);
      ConvertVarInit(init->getType(), init);
    } else {
      StrCat(GetDefaultAsString(qual_type));
    }
  } else if (qual_type->isRecordType()) {
    const auto *record = qual_type->getAsRecordDecl();
    if (record->getQualifiedNameAsString() == "std::array") {
      StrCat("vec!");
      if (auto init = clang::dyn_cast<clang::InitListExpr>(expr->getInit(0))) {
        VisitInitListExpr(init);
      } else {
        StrCat("[]");
      }
      return false;
    }

    StrCat(GetUnsafeTypeAsString(qual_type));
    PushBrace brace(*this);
    int i = 0;
    for (const auto *field : record->fields()) {
      StrCat(GetNamedDeclAsString(field), token::kColon);
      ConvertVarInit(field->getType(), expr->getInit(i++));
      StrCat(token::kComma);
    }
  } else {
    PushBracket bracket(*this);
    for (auto *init : expr->inits()) {
      ConvertVarInit(init->getType(), init);
      StrCat(token::kComma);
    }
    if (expr->hasArrayFiller()) {
      if (auto arr_ty = ctx_.getAsConstantArrayType(expr->getType())) {
        assert(
            (arr_ty->getSize().getZExtValue() - expr->getNumInits()) &&
            "Number of initializers should be less that total size of array");
        for (unsigned i = 0;
             i < arr_ty->getSize().getZExtValue() - expr->getNumInits(); ++i) {
          ConvertVarInit(expr->getArrayFiller()->getType(),
                         expr->getArrayFiller());
          StrCat(token::kComma);
        }
      }
    }
  }
  return false;
}

bool Converter::VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  if (base->IgnoreCasts()->getType()->isPointerType()) {
    ConvertPointerSubscript(expr);
  } else {
    ConvertArraySubscript(base, expr->getIdx(), expr->getType());
  }
  return false;
}

bool Converter::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr *expr) {
  StrCat(keyword_default_);
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return false;
}

bool Converter::VisitVAArgExpr(clang::VAArgExpr *expr) {
  auto va_list_expr = expr->getSubExpr();
  if (auto *cast = clang::dyn_cast<clang::ImplicitCastExpr>(va_list_expr)) {
    va_list_expr = cast->getSubExpr();
  }
  Convert(va_list_expr);
  StrCat(".arg::<");
  Convert(expr->getType());
  StrCat(">()");
  return false;
}

bool Converter::VisitGNUNullExpr(clang::GNUNullExpr *expr) {
  StrCat(keyword_default_);
  computed_expr_type_ = ComputedExprType::FreshPointer;
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
    if (!curr_init_type_.empty() && curr_init_type_.top()->isPointerType()) {
      StrCat(".as_mut_ptr()");
    }
  } else {
    auto initializer_as_string = ToString(expr->getInitializer());
    auto new_as_string =
        std::format("(Box::leak(Box::new({})) as {})", initializer_as_string,
                    ToString(expr->getType()));
    StrCat(new_as_string);
  }
  return false;
}

bool Converter::VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) {
  auto *argument = expr->getArgument();
  auto argument_as_string = ToString(argument);
  if (expr->isArrayForm()) {
    auto destroyed_type = expr->getDestroyedType();
    auto destroyed_type_as_string = ToString(destroyed_type);
    if (destroyed_type.isConstQualified()) {
      StrCat(std::format(
          R"(
        ::std::mem::drop(Box::from_raw(
          ::std::slice::from_raw_parts({},
            libcc2rs::malloc_usable_size({} as *mut ::libc::c_void) /
            ::std::mem::size_of::<{}>()) as *const [{}] as *mut [{}])))",
          argument_as_string, argument_as_string, destroyed_type_as_string,
          destroyed_type_as_string, destroyed_type_as_string));
    } else {
      StrCat(std::format(
          R"(
        ::std::mem::drop(Box::from_raw(
          ::std::slice::from_raw_parts_mut({},
            libcc2rs::malloc_usable_size({} as *mut ::libc::c_void) /
            ::std::mem::size_of::<{}>()))))",
          argument_as_string, argument_as_string, destroyed_type_as_string));
    }
  } else {
    StrCat(
        std::format("::std::mem::drop(Box::from_raw({}))", argument_as_string));
  }
  return false;
}

void Converter::ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) {
  StrCat(std::format("std::array::from_fn::<_, {}, _>",
                     GetArraySize(expr->getType())));
  PushParen paren(*this);
  StrCat("|_|");
  ConvertCXXConstructExprArgs(expr);
}

void Converter::ConvertCXXConstructExprArgs(clang::CXXConstructExpr *expr) {
  auto ctor = expr->getConstructor();
  auto ctor_name = GetRecordName(ctor->getParent());
  StrCat(ctor_name, token::kDoubleColon,
         ctor_name + (GetNumberOfConvertingCtors(ctor->getParent()) != 1
                          ? std::to_string(GetCtorIndex(ctor))
                          : ""));
  PushParen paren(*this);

  unsigned arg_idx = 0;
  for (unsigned param_idx = 0; param_idx < ctor->getNumParams(); ++param_idx) {
    auto param = ctor->getParamDecl(param_idx);
    auto param_type = param->getType();
    bool has_default = param->hasDefaultArg();

    if (arg_idx < expr->getNumArgs()) {
      clang::Expr *arg = expr->getArg(arg_idx++);

      if (has_default) {
        StrCat("Some(");
        ConvertVarInit(param_type, arg);
        StrCat(')');
      } else {
        ConvertVarInit(param_type, arg);
      }
    } else {
      assert(has_default);
      StrCat("None");
    }
    StrCat(token::kComma);
  }
}

bool Converter::VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
  if (auto str = GetMappedAsString(expr, expr->getArgs(), expr->getNumArgs());
      !str.empty()) {
    StrCat(str);
    return false;
  }

  auto *ctor = expr->getConstructor();
  if (ctor->isCopyOrMoveConstructor() ||
      (ctor->isConvertingConstructor(false) && ctor->getNumParams() == 1 &&
       ctor->getParamDecl(0)->getType()->isRValueReferenceType())) {
    Convert(expr->getArg(0));
    if (ctor->isCopyConstructor() && !IsRedundantCopyInConversion(ctx_, expr)) {
      StrCat(".clone()");
    }
    return false;
  }

  if (ctor->isDefaultConstructor() && !ctor->isUserProvided()) {
    auto ty = expr->getType();
    StrCat(GetDefaultAsString(ty));
    return false;
  }

  assert(ctor->isUserProvided());
  if (expr->getType()->isArrayType()) {
    ConvertArrayCXXConstructExpr(expr);
  } else {
    ConvertCXXConstructExprArgs(expr);
  }
  return false;
}

bool Converter::VisitUnaryExprOrTypeTraitExpr(
    clang::UnaryExprOrTypeTraitExpr *expr) {
  switch (expr->getKind()) {
  case clang::UnaryExprOrTypeTrait::UETT_SizeOf:
    StrCat(std::format(
        "::std::mem::size_of::<{}>() as u64",
        GetUnsafeTypeAsString(expr->isArgumentType()
                                  ? expr->getArgumentType()
                                  : expr->getArgumentExpr()->getType())));
    computed_expr_type_ = ComputedExprType::FreshValue;
    break;
  default:
    // FIXME: improve error handling
    log() << "unsupported unary expr or type trait expr\n";
  }
  return false;
}

bool Converter::VisitTypeTraitExpr(clang::TypeTraitExpr *expr) {
  clang::Expr::EvalResult result;
  ENSURE(expr->EvaluateAsInt(result, ctx_));
  StrCat(std::to_string(result.Val.getInt().getExtValue()));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return false;
}

bool Converter::VisitEnumDecl(clang::EnumDecl *decl) {
  ENSURE(decl_ids_.insert(GetID(decl)).second);
  if (Mapper::Contains(ctx_.getCanonicalTagType(decl))) {
    return false;
  }
  Mapper::AddRuleForUserDefinedType(decl);
  StrCat("#[derive(Clone, Copy, PartialEq, Debug, Default)]");
  StrCat(std::format("enum {}", GetRecordName(decl)));
  StrCat("{");
  bool first_enumerator = true;
  for (auto e : decl->enumerators()) {
    llvm::SmallVector<char, 32> init;
    e->getInitVal().toString(init, 10);
    if (first_enumerator) {
      StrCat("#[default]");
      first_enumerator = false;
    }
    StrCat(std::format("{} = {},", std::string_view(e->getName()),
                       std::string_view(init.data(), init.size())));
  }
  StrCat("}");

  AddFromImpl(decl);
  return false;
}

void Converter::AddFromImpl(clang::EnumDecl *decl) {
  auto name = GetRecordName(decl);
  StrCat(std::format("impl From<i32> for {}", name));
  PushBrace impl(*this);
  StrCat(std::format("fn from(n: i32) -> {}", name));
  PushBrace fn(*this);
  StrCat("match n");
  PushBrace match(*this);
  for (auto e : decl->enumerators()) {
    llvm::SmallVector<char, 32> init;
    e->getInitVal().toString(init, 10);
    StrCat(std::format("{} => {}::{},",
                       std::string_view(init.data(), init.size()), name,
                       std::string_view(e->getName())));
  }
  StrCat(std::format("_ => panic!(\"invalid {} value: {{}}\", n),", name));
}

bool Converter::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) {
  if (expr->getType()->isPointerType()) {
    StrCat(keyword_default_);
  }
  return false;
}

bool Converter::VisitLambdaExpr(clang::LambdaExpr *expr) {
  if (isAddrOf() && expr->capture_size() == 0) {
    StrCat("Some");
  }
  PushParen paren(*this);
  StrCat("|");
  for (auto p : expr->getLambdaClass()->getLambdaCallOperator()->parameters()) {
    StrCat(GetNamedDeclAsString(p), token::kColon, ToString(p->getType()),
           token::kComma);
  }
  StrCat("| {");
  EmitFunctionPreamble(expr->getLambdaClass()->getLambdaCallOperator());
  // TODO: replace with a stack
  auto old_function = curr_function_;
  curr_function_ = expr->getLambdaClass()->getLambdaCallOperator();
  ConvertFunctionBody(curr_function_);
  curr_function_ = old_function;
  StrCat("}");
  return false;
}

bool Converter::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) {
  if (auto arr_ty = clang::dyn_cast<clang::ArrayType>(
          expr->getType()->getCanonicalTypeInternal().getTypePtr())) {
    if (auto const_arr_ty = clang::dyn_cast<clang::ConstantArrayType>(arr_ty)) {
      StrCat(
          std::format("std::array::from_fn::<_, {}, _>(|_| Default::default())",
                      const_arr_ty->getSize().getZExtValue()));
      return false;
    }
  }

  StrCat(GetDefaultAsString(expr->getType()));
  return false;
}

bool Converter::ConvertSwitchCaseCondition(clang::SwitchCase *stmt) {
  clang::Stmt *cur = stmt;
  clang::SwitchCase *last = nullptr;
  bool first = true;

  while (auto *sc = clang::dyn_cast<clang::SwitchCase>(cur)) {
    if (auto *case_stmt = clang::dyn_cast<clang::CaseStmt>(sc)) {
      if (!first) {
        StrCat("|| v == ");
      }
      Convert(case_stmt->getLHS());
    }
    last = sc;
    first = false;
    cur = sc->getSubStmt();
  }

  if (clang::isa<clang::CaseStmt>(last)) {
    StrCat(" => {");
  } else /* DefaultStmt */ {
    StrCat("_ => {");
  }
  return false;
}

void Converter::EmitSwitchArm(clang::CompoundStmt *body, clang::SwitchCase *sc,
                              bool is_default) {
  if (is_default) {
    StrCat("_ => {");
  } else {
    StrCat("v if v == ");
    ConvertSwitchCaseCondition(sc);
  }
  for (auto *t : GetSwitchCaseBody(body, sc)) {
    Convert(t);
  }
  StrCat("},");
}

bool Converter::VisitSwitchStmt(clang::SwitchStmt *stmt) {
  bool has_fallthrough = SwitchHasFallthrough(stmt);
  PushBreakTarget push(break_target_, has_fallthrough
                                          ? BreakTarget::FallthroughSwitch
                                          : BreakTarget::Switch);
  auto *body = clang::dyn_cast<clang::CompoundStmt>(stmt->getBody());
  assert(body);

  if (has_fallthrough) {
    // Use the switch-with-fallthrough macro
    StrCat("switch!");
  } else {
    StrCat("'switch:");
  }

  PushParen switch_macro_paren(*this, has_fallthrough);
  PushBrace switch_label_brace(*this, !has_fallthrough);

  if (has_fallthrough) {
    StrCat("match", ToString(stmt->getCond()));
  } else {
    StrCat(std::format("let __match_cond = {};", ToString(stmt->getCond())));
    StrCat("match __match_cond");
  }

  PushBrace match_brace(*this);

  clang::SwitchCase *default_case = nullptr;
  for (auto *sc : GetTopLevelSwitchCases(stmt)) {
    if (SwitchCaseContainsDefault(sc)) {
      default_case = sc;
      continue;
    }
    EmitSwitchArm(body, sc, /*is_default=*/false);
  }

  if (default_case) {
    EmitSwitchArm(body, default_case, /*is_default=*/true);
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
  if (expr->getSubExpr()->getType()->isArrayType()) {
    // Arrays become Vec's
    StrCat("vec!");
  }
  Convert(expr->getSubExpr());
  return false;
}

std::string Converter::GetDefaultAsString(clang::QualType qual_type) {
  if (IsVaListType(qual_type)) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return "VaList::default()";
  }

  if (qual_type->isPointerType()) {
    auto pointee = qual_type->getPointeeType();
    if (pointee->isFunctionType()) {
      return "None";
    }
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return pointee.isConstQualified() ? "std::ptr::null()"
                                      : "std::ptr::null_mut()";
  }

  computed_expr_type_ = ComputedExprType::FreshValue;

  if (auto *array_type = clang::dyn_cast<clang::ConstantArrayType>(qual_type)) {
    auto size_as_string = GetNumAsString(array_type->getSize());
    auto element_type = array_type->getElementType();
    auto element_type_as_string = GetDefaultAsString(element_type);
    return std::format("[{}; {}]", element_type_as_string,
                       size_as_string.c_str());
  } else if (auto *array_type =
                 clang::dyn_cast<clang::IncompleteArrayType>(qual_type)) {
    return GetDefaultAsString(array_type->getElementType());
  } else {
    auto qual_type_str = Mapper::ToString(qual_type);
    if (qual_type_str == "struct std::pair") {
      auto template_args = *GetTemplateArgs(qual_type);
      auto first_type = template_args[0].getAsType();
      auto second_type = template_args[1].getAsType();
      return std::format("({}, {})", GetDefaultAsString(first_type),
                         GetDefaultAsString(second_type));
    } else if (qual_type_str.contains("std::array")) {
      assert(GetTemplateArgs(qual_type).has_value());
      auto template_args = *GetTemplateArgs(qual_type);
      assert(template_args.size() == 2);
      auto array_size = template_args[1];
      unsigned size = 0;
      switch (array_size.getKind()) {
      case clang::TemplateArgument::Expression: {
        auto array_size_expr = array_size.getAsExpr();
        assert(array_size_expr && !array_size_expr->isValueDependent());
        clang::Expr::EvalResult result;
        ENSURE(array_size_expr->EvaluateAsInt(result, ctx_));
        size = result.Val.getInt().getZExtValue();
        break;
      }
      case clang::TemplateArgument::Integral: {
        size = array_size.getAsIntegral().getZExtValue();
        break;
      }
      default:
        assert(0 && "Unsupported array size kind");
        break;
      }
      return std::format(
          "std::array::from_fn::<_, {}, _>(|_| Default::default()).to_vec()",
          size);
    } else {
      return GetDefaultAsStringFallback(qual_type);
    }
  }
}

std::string Converter::GetDefaultAsStringFallback(clang::QualType qual_type) {
  qual_type = qual_type.getUnqualifiedType().getCanonicalType();

  if (qual_type->isBooleanType()) {
    return "false";
  }

  if (qual_type->isIntegerType() && !qual_type->isEnumeralType()) {
    return std::format("0_{}", ToString(qual_type));
  }

  if (qual_type->isFloatingType()) {
    return std::format("0.0_{}", ToString(qual_type));
  }

  if (auto record = qual_type->getAsRecordDecl();
      record && in_const_initializer_) {
    if (auto cxx = clang::dyn_cast<clang::CXXRecordDecl>(record)) {
      ENSURE(GetUserDefinedDefaultConstructor(cxx) == nullptr &&
             "Default initializing globals using default constructor is not "
             "supported");
    }
    Buffer buf(*this);
    EmitDefaultStructLiteral(record);
    return std::move(buf).str();
  }

  return std::format("<{}>::default()", ToString(qual_type));
}

std::string Converter::ConvertVarDefaultInit(clang::QualType qual_type) {
  return GetDefaultAsString(qual_type);
}

std::string
Converter::GetOverloadedFunctionName(const clang::FunctionDecl *decl) {
  auto name = decl->getNameAsString();

  if (decl->getNumParams() != 0U) {
    name += '_';
  }

  for (auto *parameter : decl->parameters()) {
    name += GetUnsafeTypeAsString(parameter->getType());
    name += '_';
  }

  auto pred = [](char ch) { return ch != ' ' && ch != '_'; };
  name.erase(std::find_if(name.rbegin(), name.rend(), pred).base(), name.end());
  if (const auto *method = clang::dyn_cast<clang::CXXMethodDecl>(decl);
      method && method->isConst()) {
    name += "_const";
  }

  name.erase(std::remove_if(name.begin(), name.end(),
                            [](char c) {
                              return c == '<' || c == '>' || c == ' ' ||
                                     c == ':';
                            }),
             name.end());
  std::replace(name.begin(), name.end(), '*', 'p');

  return name;
}

std::string Converter::GetRecordName(const clang::NamedDecl *decl) const {
  auto ID = GetID(decl);
  if (auto it = inner_structs_.find(ID); it != inner_structs_.end()) {
    return it->second;
  }
  return ReplaceAll(Mapper::ToString(decl), "::", "_");
}

std::vector<const char *>
Converter::GetStructAttributes(const clang::RecordDecl *decl) {
  if (decl->isUnion()) {
    return {"Copy", "Clone"};
  }

  std::vector<const char *> struct_attrs = {};

  if (recordDerivesCopy(decl)) {
    struct_attrs.emplace_back("Copy");
  }

  if (auto cxx_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (!cxx_decl->defaultedCopyConstructorIsDeleted()) {
      struct_attrs.emplace_back("Clone");
    }
  } else /* RecordDecl */ {
    struct_attrs.emplace_back("Clone");
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
  if (qual_type->isReferenceType() && !IsReferenceType(expr)) {
    StrCat(token::kRef);
    if (IsMut(qual_type)) {
      StrCat(keyword_mut_);
    }
  }
  if (qual_type->isFunctionPointerType()) {
    if (auto *lambda = clang::dyn_cast<clang::LambdaExpr>(
            expr->IgnoreUnlessSpelledInSource())) {
      PushExprKind push(*this, ExprKind::AddrOf);
      PushInitType init_type(*this, qual_type);
      VisitLambdaExpr(lambda);
      return;
    }
  }
  auto *ignore_casts = expr->IgnoreCasts();
  // FIXME: this looks very complicated
  if (auto *ctor = clang::dyn_cast<clang::CXXConstructExpr>(ignore_casts);
      ctor && ctor->getNumArgs() != 0 && IsReferenceType(ctor->getArg(0)) &&
      clang::isa<clang::CallExpr>(ctor->getArg(0)->IgnoreCasts()) &&
      !Mapper::Contains(
          clang::cast<clang::CallExpr>(ctor->getArg(0)->IgnoreCasts())
              ->getCallee()) &&
      Mapper::ToString(ctor->getConstructor()->getThisType()) ==
          "std::string") {
    {
      PushParen paren(*this);
      StrCat(token::kStar);
      PushInitType init_type(*this, qual_type);
      Convert(expr);
    }
    StrCat(".clone()");
  } else if (IsReferenceType(expr) || qual_type->isFunctionPointerType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    PushInitType init_type(*this, qual_type);
    Convert(expr);
  } else {
    PushExprKind push(*this, ExprKind::RValue);
    PushInitType init_type(*this, qual_type);
    Convert(expr);
  }
  if (qual_type->isReferenceType() && !IsReferenceType(expr)) {
    StrCat(keyword::kAs);
    Convert(qual_type);
  }
}

void Converter::ConvertUnsignedArithOperand(clang::Expr *expr,
                                            clang::QualType type) {
  Convert(expr);
  if ((expr->isIntegerConstantExpr(ctx_) &&
       !clang::isa<clang::ImplicitCastExpr>(expr)) ||
      Mapper::Map(expr->getType()) != Mapper::Map(type)) {
    ConvertCast(type);
  }
}

void Converter::ConvertEqualsNullPtr(clang::Expr *expr) {
  StrCat('(');
  Convert(expr);
  if (IsUniquePtr(expr->getType()) ||
      expr->getType()->isFunctionPointerType()) {
    StrCat(").is_none()");
  } else {
    StrCat(").is_null()");
  }
}

void Converter::ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  auto *idx = expr->getIdx();
  if (isAddrOf()) {
    ConvertPointerOffset(base, idx);
  } else {
    PushParen paren(*this);
    StrCat(token::kStar);
    ConvertPointerOffset(base, idx);
  }
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
    Convert(idx);
  }
  StrCat(keyword::kAs, "isize");
  computed_expr_type_ = ComputedExprType::FreshPointer;
}

void Converter::ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                                      clang::QualType type) {
  if (IsUniquePtr(base->getType())) {
    PushExplicitAutoref no_autoref(*this, std::nullopt);
    Convert(base->IgnoreImplicit());
    StrCat(".as_mut().unwrap()");
  } else {
    Convert(base->IgnoreImplicit());
  }
  PushExplicitAutoref no_autoref(*this, std::nullopt);
  PushBracket bracket(*this);
  {
    PushParen paren(*this);
    Convert(idx);
  }
  StrCat(keyword::kAs, "usize");
}

void Converter::ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                  std::string_view assign_operator) {
  std::string lhs_as_string;
  {
    PushInitType init_type(*this, lhs->getType());
    lhs_as_string = ConvertLValue(lhs);
  }
  auto rhs_as_string = ConvertFreshRValue(rhs);

  PushBrace brace(*this, !isVoid());

  StrCat(lhs_as_string, assign_operator, rhs_as_string);
  if (!isVoid()) {
    StrCat(token::kSemiColon, ConvertRValue(lhs));
  }
}

void Converter::ConvertFunctionParameters(clang::FunctionDecl *decl) {
  in_function_formals_ = true;
  auto *definition =
      decl->getDefinition() != nullptr ? decl->getDefinition() : decl;
  for (auto *parameter : definition->parameters()) {
    ConvertVarDeclSkipInit(parameter);
    StrCat(token::kComma);
  }
  if (decl->isVariadic()) {
    StrCat("__args: &[VaArg]", token::kComma);
  }
  in_function_formals_ = false;
}

void Converter::ConvertFunctionQualifiers(clang::FunctionDecl *decl) {
  StrCat(AccessSpecifierAsString(decl->getAccess()));
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
    let mut argv: Vec<*mut u8> = args.iter().map(|arg| arg.as_ptr() as *mut u8).collect();
    argv.push(::std::ptr::null_mut());
    unsafe {{
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }}
}})",
                       main_function_name));
  } else {
    StrCat(std::format(
        "pub fn main() {{ unsafe {{ std::process::exit({}() as i32); }} }}",
        main_function_name));
  }
}

void Converter::ConvertAbstractClass(clang::CXXRecordDecl *decl) {
  ENSURE(abstract_structs_.insert(GetID(decl)).second);
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
  for (auto *method : decl->methods()) {
    if (predicate(method)) {
      if (first) {
        StrCat(signature, token::kOpenCurlyBracket);
        first = false;
      }
      VisitCXXMethodDecl(method);
    }
  }
  if (!first) {
    StrCat(token::kCloseCurlyBracket);
  }
}

void Converter::ConvertOrdAndPartialOrdTraitsBase(
    std::string_view first_branch, std::string_view second_branch,
    std::string_view first_return, std::string_view second_return,
    std::string_view record_name) {
  StrCat(keyword::kImpl, "Ord for ", record_name, "{");
  StrCat("fn cmp(&self, other: &Self) -> std::cmp::Ordering {");
  StrCat(std::format("{} {{", keyword_unsafe_));
  StrCat("if", first_branch, "{", first_return, "} else if", second_branch, "{",
         second_return, "} else { std::cmp::Ordering::Equal }");
  StrCat("}}}");

  StrCat(keyword::kImpl, "PartialOrd for", record_name, "{");
  StrCat(R"(
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
    }
  })");

  StrCat(keyword::kImpl, "PartialEq for", record_name, "{");
  StrCat("fn eq(&self, other: &Self) -> bool {");
  StrCat(std::format("{} {{", keyword_unsafe_));
  StrCat("!(", first_branch, ") && !(", second_branch, ')');
  StrCat("}}}");

  StrCat(keyword::kImpl, "Eq for", record_name, "{}");
}

void Converter::ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                              const clang::FunctionDecl *op) {
  std::string first_branch, second_branch, first_return, second_return;

  switch (op->getOverloadedOperator()) {
  case clang::OO_Less:
    if (clang::isa<clang::CXXMethodDecl>(op)) {
      first_branch = std::format("self.{}(other)", GetOverloadedOperator(op));
      second_branch = std::format("other.{}(other)", GetOverloadedOperator(op));
    } else {
      first_branch = std::format("{}(self, other)", GetOverloadedOperator(op));
      second_branch = std::format("{}(other, self)", GetOverloadedOperator(op));
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

void Converter::AddOrdTrait(const clang::CXXRecordDecl *decl) {
  std::vector<clang::CXXMethodDecl *> methods;
  std::copy_if(decl->method_begin(), decl->method_end(),
               std::back_inserter(methods), [](const auto *method) {
                 if (method->isOverloadedOperator()) {
                   auto opKind = method->getOverloadedOperator();
                   if (opKind == clang::OO_Less ||
                       opKind == clang::OO_Spaceship) {
                     return true;
                   }
                 }
                 return false;
               });

  if (methods.empty()) {
    return;
  }

  if (methods.size() > 1) {
    llvm::errs()
        << "Currently supporting only one overloaded comparison operator\n";
    abort();
  }

  ConvertOrdAndPartialOrdTraits(decl, methods[0]);
}

void Converter::AddCloneTrait(const clang::CXXRecordDecl *decl) {}

void Converter::AddDropTrait(const clang::CXXRecordDecl *decl) {}

void Converter::AddDefaultTraitForUnion(const clang::RecordDecl *decl) {
  StrCat(std::format("impl Default for {}", GetRecordName(decl)));
  PushBrace impl_brace(*this);
  StrCat("fn default() -> Self");
  PushBrace fn_brace(*this);
  StrCat("unsafe");
  PushBrace unsafe_brace(*this);
  StrCat("std::mem::zeroed()");
}

void Converter::AddDefaultTrait(const clang::RecordDecl *decl) {
  if (decl->isUnion()) {
    AddDefaultTraitForUnion(decl);
    return;
  }
  if (RecordDerivesDefault(decl)) {
    return;
  }
  auto struct_name = GetRecordName(decl);
  StrCat(std::format("impl Default for {}", struct_name));
  PushBrace impl_brace(*this);
  StrCat("fn default() -> Self");
  PushBrace fn_brace(*this);

  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (auto *default_ctor = GetUserDefinedDefaultConstructor(cxx)) {
      StrCat(keyword_unsafe_);
      PushBrace unsafe_brace(*this);
      Convert(clang::CXXConstructExpr::Create(
          ctx_, ctx_.getCanonicalTagType(decl), clang::SourceLocation(),
          default_ctor,
          /*Elidable=*/false, llvm::ArrayRef<clang::Expr *>(),
          /*HadMultipleCandidates=*/false,
          /*ListInitialization=*/false,
          /*StdInitListInitialization=*/false,
          /*ZeroInitialization=*/false, clang::CXXConstructionKind::Complete,
          clang::SourceRange()));
      return;
    }
  }

  EmitDefaultStructLiteral(decl);
}

void Converter::EmitDefaultStructLiteral(const clang::RecordDecl *decl) {
  StrCat(GetRecordName(decl));
  PushBrace brace(*this);
  for (auto *field : decl->fields()) {
    StrCat(GetNamedDeclAsString(field), token::kColon,
           GetDefaultAsString(field->getType()), token::kComma);
  }
}

void Converter::AddByteReprTrait(const clang::RecordDecl *decl) {}

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
  if (IsReferenceType(expr) || pointer_type->isFunctionPointerType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    Convert(expr);
  } else if (IsGlobalVar(expr)) {
    StrCat("&raw", pointer_type->getPointeeType().isConstQualified()
                       ? keyword::kConst
                       : keyword_mut_);
    Convert(expr);
    ConvertCast(pointer_type);
  } else {
    StrCat(token::kRef);
    if (!pointer_type->getPointeeType().isConstQualified()) {
      StrCat(keyword_mut_);
    }
    Convert(expr);
    ConvertCast(pointer_type);
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
}

void Converter::ConvertDeref(clang::Expr *expr) {
  if (!isAddrOf()) {
    EmitDeref(ToString(expr), expr->getType()->getPointeeType());
  } else {
    Convert(expr);
  }
}

void Converter::ConvertArrow(clang::Expr *expr) { ConvertDeref(expr); }

void Converter::ConvertCast(clang::QualType qual_type, int line) {
  log() << "[ConvertCast] Called from line " << line << '\n';
  StrCat(keyword::kAs, GetUnsafeTypeAsString(qual_type));
}

Converter::TempMaterializationCtx
Converter::CollectPrvalueToLRefArgs(clang::CallExpr *expr) {
  TempMaterializationCtx ctx(expr->getNumArgs());
  if (auto *fn = expr->getCalleeDecl() ? expr->getCalleeDecl()->getAsFunction()
                                       : nullptr) {
    for (unsigned i = 0; i < expr->getNumArgs() && i < fn->getNumParams();
         ++i) {
      auto param_type = fn->getParamDecl(i)->getType();
      if (param_type->isLValueReferenceType() &&
          clang::isa<clang::MaterializeTemporaryExpr>(expr->getArg(i))) {
        ctx.materialized_args[i] = param_type;
      }
    }
  }
  return ctx;
}

const std::string &Converter::TempMaterializationCtx::GetOrMaterialize(
    unsigned argument_num,
    std::function<std::pair<std::string, std::string>(const std::string &,
                                                      clang::QualType)>
        materialize_fn) {
  auto &str = materialized_refs_.at(argument_num);
  if (!str.empty()) {
    return str;
  }

  if (auto m = materialized_args.at(argument_num)) {
    auto [binding, ref] =
        materialize_fn(std::format("__tmp_{}", argument_num), *m);
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
               << declared_in_rule_as_rust_ptr << ", access: "
               << (access == TranslationRule::Access::kRead ? "read" : "write")
               << ", param_type: " << param_type
               << ", materialize_idx: " << materialize_idx << '\n';
}

std::string Converter::ConvertPlaceholder(clang::Expr *expr, clang::Expr *arg,
                                          const PlaceholderCtx &ph_ctx) {
  if (arg->getType()->isFunctionPointerType()) {
    PushExprKind push(*this, ExprKind::Callee);
    Buffer buf(*this);
    Convert(arg);
    return std::move(buf).str();
  }

  if (ph_ctx.needs_materialization()) {
    auto materialized = ph_ctx.materialize_ctx->GetOrMaterialize(
        static_cast<unsigned>(ph_ctx.materialize_idx),
        [this, arg](const std::string &name, clang::QualType type) {
          return MaterializeTemp(name, type, arg);
        });
    if (!materialized.empty()) {
      return materialized;
    }
  }

  if (ph_ctx.needs_pointer_receiver()) {
    return std::format("({} as {})", ConvertFreshObject(arg),
                       ph_ctx.param_type);
  }

  if (ph_ctx.needs_object_receiver()) {
    Buffer buf(*this);
    ConvertDeref(arg);
    return std::move(buf).str();
  }

  if (ph_ctx.needs_ptr_wrap()) {
    return ConvertFreshObject(arg);
  }

  if (ph_ctx.needs_lvalue()) {
    return ConvertLValue(arg);
  }

  if (ph_ctx.access == TranslationRule::Access::kMove) {
    if (clang::isa<clang::MaterializeTemporaryExpr>(arg)) {
      return ConvertRValue(arg);
    }
    return std::format("std::mem::take(&mut {})", ConvertLValue(arg));
  }

  return ConvertRValue(arg);
}

std::string Converter::ConvertMappedMethodCall(
    clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
    clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx) {
  return ConvertIRFragment(mc.receiver, expr, args, num_args, ctx) +
         ConvertIRFragment(mc.body, expr, args, num_args, ctx);
}

std::string Converter::GetMappedAsString(clang::Expr *expr, clang::Expr **args,
                                         unsigned num_args,
                                         TempMaterializationCtx *ctx) {
  auto *tgt_ir = Mapper::GetExprRule(GetCalleeOrExpr(expr));
  if (!tgt_ir)
    return {};

  auto result = ConvertIRFragment(tgt_ir->body, expr, args, num_args, ctx);
  if (tgt_ir->multi_statement) {
    return '{' + result + '}';
  }
  return result;
}

std::string Converter::ConvertIRFragment(
    const std::vector<TranslationRule::BodyFragment> &fragments,
    clang::Expr *expr, clang::Expr **args, unsigned num_args,
    TempMaterializationCtx *ctx) {
  using namespace TranslationRule;

  auto all_args = BuildUnifiedArgs(expr, args, num_args);

  std::string result;
  for (auto &frag : fragments) {
    if (auto *t = std::get_if<TextFragment>(&frag)) {
      result += t->text;
    } else if (auto *g = std::get_if<GenericFragment>(&frag)) {
      result += Mapper::InstantiateTemplate(GetCalleeOrExpr(expr), g->n);
    } else if (auto *ph = std::get_if<PlaceholderFragment>(&frag)) {
      auto arg_idx = ph->n;
      assert(arg_idx < all_args.size());
      auto *arg = all_args[arg_idx];
      bool is_receiver = HasReceiver(expr) && arg_idx == 0;

      PlaceholderCtx ph_ctx{
          .param_type = Mapper::GetParamType(GetCalleeOrExpr(expr), arg_idx),
          .materialize_ctx = ctx,
          .materialize_idx =
              is_receiver ? -1 : ((int)arg_idx - HasReceiver(expr)),
          .access = ph->access,
          .is_receiver = is_receiver,
          .is_cpp_ptr = arg->getType()->isPointerType(),
          .maps_to_rust_ptr = Mapper::MapsToPointer(arg->getType()),
          .declared_in_rule_as_rust_ptr =
              Mapper::ParamIsPointer(GetCalleeOrExpr(expr), arg_idx),
      };
      result += ConvertPlaceholder(expr, arg, ph_ctx);
    } else if (auto *mc =
                   std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
      result += ConvertMappedMethodCall(expr, **mc, args, num_args, ctx);
    }
  }

  return result;
}

std::string Converter::AccessLValueObject(clang::MemberExpr *member) {
  auto *object = member->getBase();
  auto type = object->getType();
  if (member->isArrow()) {
    auto *op =
        clang::dyn_cast<clang::CXXOperatorCallExpr>(object->IgnoreImplicit());
    if (op && GetStrongestIteratorCategory(op->getArg(0)->getType()) ==
                  IteratorCategory::Bidirectional) {
      return ToString(object);
    }
  }
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

bool Converter::ShouldReplaceWithMappedBody(clang::DeclRefExpr *expr) const {
  if (clang::isa<clang::FunctionDecl>(expr->getDecl()) && isAddrOf()) {
    return false;
  }
  return true;
}

void Converter::SetFresh() {
  switch (computed_expr_type_) {
  case ComputedExprType::Value:
    computed_expr_type_ = ComputedExprType::FreshValue;
    break;
  case ComputedExprType::Pointer:
    computed_expr_type_ = ComputedExprType::FreshPointer;
    break;
  case ComputedExprType::FreshValue:
  case ComputedExprType::FreshPointer:
    break;
  }
}

static bool hasCopyTrait(clang::QualType type) {
  if (type->isBuiltinType())
    return true;

  return false;
}

void Converter::SetValueFreshness(clang::QualType type) {
  if (hasCopyTrait(type)) {
    computed_expr_type_ = ComputedExprType::FreshValue;
  } else if (type->isPointerType() || type->isReferenceType()) {
    computed_expr_type_ = ComputedExprType::Pointer;
  } else {
    computed_expr_type_ = ComputedExprType::Value;
  }
}

void Converter::SetFreshType(clang::QualType type) {
  computed_expr_type_ = type->isPointerType() || type->isReferenceType()
                            ? ComputedExprType::FreshPointer
                            : ComputedExprType::FreshValue;
}

void Converter::dump_expr_kinds() {
  log() << "isRValue: " << isRValue() << ", isXValue: " << isXValue()
        << ", isAddrOf: " << isAddrOf() << ", isObject: " << isObject()
        << ", isVoid: " << isVoid() << '\n';
}

void Converter::emplace_back_plugin_construct_arg(
    clang::QualType elem_type, clang::CXXConstructExpr *ctor) {
  ConvertVarInit(elem_type, ctor);
}

const char *Converter::GetPointerDerefPrefix(clang::QualType pointee_type) {
  return token::kStar;
}

} // namespace cpp2rust
