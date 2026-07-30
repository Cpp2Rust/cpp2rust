// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter.h"

#include <clang/AST/APValue.h>
#include <clang/AST/ParentMapContext.h>
#include <clang/Basic/LangOptions.h>
#include <clang/Basic/SourceManager.h>
#include <llvm/ADT/DenseMap.h>
#include <llvm/Support/ConvertUTF.h>

#include <algorithm>
#include <format>
#include <utility>

#include "compiler.h"
#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/mapper.h"

namespace cpp2rust {
std::unordered_map<std::string, std::string> Converter::inner_structs_;
std::unordered_set<std::string> Converter::decl_ids_;
std::unordered_set<std::string> Converter::globals_;
std::unordered_set<std::string> Converter::abstract_structs_;
Converter::RecordIndex Converter::record_decls_;

RsExpr *Converter::ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr) {
  bool is_star = expr->getOperator() == clang::OverloadedOperatorKind::OO_Star;
  auto *arg = ConvertExpr(expr->getArg(0));
  RsExpr *node = nullptr;
  if (expr->getArg(0)->IgnoreImplicit()->getType().isConstQualified()) {
    node = Cat(Text("(*(std::ptr::addr_of!("), arg,
               Text(").cast_mut())).as_deref_mut().unwrap()"));
  } else {
    node = Cat(arg, Text(".as_deref_mut().unwrap()"));
  }
  if (is_star) {
    return Parens(Cat(Text(token::kStar), node));
  }
  return node;
}

std::string Converter::EmitFilePreamble() {
  return R"(
extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Write, Seek};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
)";
}

std::string Converter::EmitOpaqueRecords() {
  std::string out;
  record_decls_.ForEachUndefined([&](const std::string &name) {
    out += "pub struct ";
    out += name;
    out += ";\n";
  });
  return out;
}

RsExpr *Converter::VisitRecoveryExpr(clang::RecoveryExpr *expr) {
  llvm::errs() << "RecoveryExpr: ";
  expr->dump();
  exit(1);
  return nullptr;
}

RsExpr *Converter::Convert(clang::QualType qual_type) {
  // Catch va_list before desugaring
  if (IsVaListType(qual_type)) {
    return Text("VaList");
  }

  auto mapped = Mapper::Map(qual_type);
  if (!mapped.empty() && mapped != token::kIgnoreRule) {
    return Text(std::move(mapped));
  }

  qual_type = qual_type.getUnqualifiedType().getDesugaredType(ctx_);
  const clang::Type *type = qual_type.getTypePtr();

  if (auto *builtin = clang::dyn_cast<clang::BuiltinType>(type)) {
    switch (builtin->getKind()) {
    case clang::BuiltinType::Bool:
      return Text("bool");
    case clang::BuiltinType::Float:
      return Text("f32");
    case clang::BuiltinType::Double:
      return Text("f64");
    case clang::BuiltinType::Char_S:
    case clang::BuiltinType::Char_U:
      return Text(CharRustType());
    case clang::BuiltinType::SChar:
      return Text("i8");
    case clang::BuiltinType::UChar:
      return Text("u8");
    case clang::BuiltinType::UShort:
    case clang::BuiltinType::UInt:
    case clang::BuiltinType::ULong:
    case clang::BuiltinType::ULongLong:
    case clang::BuiltinType::Short:
    case clang::BuiltinType::Int:
    case clang::BuiltinType::Long:
    case clang::BuiltinType::LongLong:
      return Text(std::format("{}{}", builtin->isSignedInteger() ? 'i' : 'u',
                              ctx_.getTypeSize(builtin)));
    case clang::BuiltinType::Void:
      return Text("::libc::c_void");
    case clang::BuiltinType::UInt128:
      return Text("u128");
    case clang::BuiltinType::Int128:
      return Text("i128");
    default:
      // FIXME: improve error handling
      log() << "unsupported builtin type\n";
      return Text("");
    }
  }

  if (auto *record = clang::dyn_cast<clang::RecordType>(type)) {
    auto *decl = record->getDecl();
    if (auto lambda = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
      if (lambda->isLambda()) {
        if (in_function_formals_) {
          return ConvertFunctionPointerType(
              lambda->getLambdaCallOperator()
                  ->getType()
                  ->getAs<clang::FunctionProtoType>(),
              FnProtoType::LambdaCallOperator);
        }
        return Text("_");
      }
    }

    auto name = GetRecordName(decl);
    if (!ctx_.getSourceManager().isInSystemHeader(decl->getLocation())) {
      record_decls_.MarkReferenced(name);
    }
    Mapper::AddRuleForUserDefinedType(decl);
    return Text(std::move(name));
  }

  if (auto *constant_array = clang::dyn_cast<clang::ConstantArrayType>(type)) {
    auto *elem = Convert(constant_array->getElementType());
    auto size = GetNumAsString(constant_array->getSize());
    return Cat(Text('['), elem, Text(std::format("; {}]", size.c_str())));
  }

  if (auto *incomplete_array =
          clang::dyn_cast<clang::IncompleteArrayType>(type)) {
    return Cat(Text('['), Convert(incomplete_array->getElementType()),
               Text(']'));
  }

  if (auto *reference = clang::dyn_cast<clang::LValueReferenceType>(type)) {
    auto pointee_type = reference->getPointeeType();
    return Cat(Text(pointee_type.isConstQualified() ? "*const" : "*mut"),
               Convert(pointee_type));
  }

  if (auto *decayed = clang::dyn_cast<clang::DecayedType>(type)) {
    return Convert(decayed->getDecayedType());
  }

  if (auto *pointer = clang::dyn_cast<clang::PointerType>(type)) {
    if (auto proto =
            pointer->getPointeeType()->getAs<clang::FunctionProtoType>()) {
      return Cat(Text(std::format("Option<{}", keyword_unsafe_)),
                 ConvertFunctionPointerType(proto), Text('>'));
    }

    if (IsVaListType(clang::QualType(pointer, 0))) {
      return Text("VaList");
    }

    auto pointee_type = pointer->getPointeeType();
    std::vector<RsExpr *> parts;
    parts.push_back(Text(pointee_type.isConstQualified() ? "*const" : "*mut"));
    if (pointee_type->isRecordType() &&
        abstract_structs_.contains(GetID(pointee_type->getAsRecordDecl()))) {
      parts.push_back(Text(keyword::kDyn));
    }
    parts.push_back(Convert(pointee_type));
    return arena_.New<Concat>(std::move(parts));
  }

  if (auto *typedef_type = clang::dyn_cast<clang::TypedefType>(type)) {
    return Convert(typedef_type->desugar());
  }
  if (auto *using_type = clang::dyn_cast<clang::UsingType>(type)) {
    return Convert(using_type->desugar());
  }
  return Text("");
}

std::string Converter::RenderType(clang::QualType qual_type) {
  return std::string(Trim(Convert(qual_type)->print()));
}

RsExpr *Converter::ConvertPointeeType(clang::QualType ptr_type) {
  assert(!ptr_type.isNull() && ptr_type->isPointerType());
  auto pointee = ptr_type->getPointeeType();
  if (!pointee->isRecordType()) {
    return Convert(pointee);
  }

  auto str = RenderType(ptr_type);
  Unwrap(str, "*mut ", "");
  Unwrap(str, "*const ", "");
  return Text(std::move(str));
}

RsExpr *
Converter::ConvertFunctionPointerType(const clang::FunctionProtoType *proto,
                                      FnProtoType kind) {
  std::vector<RsExpr *> parts;
  parts.push_back(
      Text(kind == FnProtoType::LambdaCallOperator ? "impl Fn(" : "fn("));
  for (auto p_ty : proto->param_types()) {
    parts.push_back(Convert(p_ty));
    parts.push_back(Text(','));
  }
  parts.push_back(Text(')'));
  if (!proto->getReturnType()->isVoidType()) {
    parts.push_back(Text("->"));
    parts.push_back(Convert(proto->getReturnType()));
  }
  return arena_.New<Concat>(std::move(parts));
}

bool Converter::Convert(clang::Decl *decl) {
  if (decl == nullptr) {
    return true;
  }
  *rs_code_ += ConvertDecl(decl)->print();
  return false;
}

RsExpr *Converter::ConvertDecl(clang::Decl *decl) {
  switch (decl->getKind()) {
  case clang::Decl::TranslationUnit:
    return VisitTranslationUnitDecl(
        clang::cast<clang::TranslationUnitDecl>(decl));
  case clang::Decl::Function:
  case clang::Decl::CXXMethod:
  case clang::Decl::CXXConstructor:
  case clang::Decl::CXXDestructor:
  case clang::Decl::CXXConversion:
    return VisitFunctionDecl(clang::cast<clang::FunctionDecl>(decl));
  case clang::Decl::CXXRecord:
  case clang::Decl::ClassTemplateSpecialization:
    return VisitCXXRecordDecl(clang::cast<clang::CXXRecordDecl>(decl));
  case clang::Decl::Record:
    return VisitRecordDecl(clang::cast<clang::RecordDecl>(decl));
  case clang::Decl::Enum:
    return VisitEnumDecl(clang::cast<clang::EnumDecl>(decl));
  case clang::Decl::Var:
    return VisitVarDecl(clang::cast<clang::VarDecl>(decl));
  case clang::Decl::Field:
    return VisitFieldDecl(clang::cast<clang::FieldDecl>(decl));
  case clang::Decl::Namespace:
    return VisitNamespaceDecl(clang::cast<clang::NamespaceDecl>(decl));
  case clang::Decl::Typedef:
    return VisitTypedefDecl(clang::cast<clang::TypedefDecl>(decl));
  case clang::Decl::ClassTemplate:
    return VisitClassTemplateDecl(clang::cast<clang::ClassTemplateDecl>(decl));
  case clang::Decl::FunctionTemplate:
    return VisitFunctionTemplateDecl(
        clang::cast<clang::FunctionTemplateDecl>(decl));
  case clang::Decl::LinkageSpec: {
    std::vector<RsExpr *> parts;
    for (auto *child : clang::cast<clang::LinkageSpecDecl>(decl)->decls()) {
      parts.push_back(ConvertDecl(child));
    }
    return arena_.New<Concat>(std::move(parts));
  }
  case clang::Decl::TypeAlias:
  case clang::Decl::Empty:
    return arena_.New<Verbatim>("");
  default:
    llvm::errs() << "ConvertDecl: unhandled declaration kind "
                 << decl->getDeclKindName() << '\n';
    assert(false && "declaration kind not handled by ConvertDecl dispatch");
    return arena_.New<Verbatim>("");
  }
}

RsExpr *Converter::VisitTranslationUnitDecl(clang::TranslationUnitDecl *decl) {
  std::vector<RsExpr *> parts;
  for (auto *child : decl->decls()) {
    if (IsUserDefinedDecl(child) &&
        (IsInMainFile(child) || !decl_ids_.contains(GetID(child)))) {
      parts.push_back(ConvertDecl(child));
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitFunctionDecl(clang::FunctionDecl *decl) {
  if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(decl)) {
    return VisitCXXMethodDecl(method);
  }
  if (!IsConvertibleFunctionDecl(decl)) {
    return arena_.New<Verbatim>("");
  }
  if (!IsInMainFile(decl) && !decl_ids_.insert(GetID(decl)).second) {
    return arena_.New<Verbatim>("");
  }
  decl->dump(log());
  curr_function_ = decl;
  std::vector<RsExpr *> parts;
  std::string function_name;
  if (decl->isMain()) {
    function_name = "main_0";
    parts.push_back(ConvertFunctionMain(decl, function_name));
  } else if (decl->isOverloadedOperator()) {
    function_name = GetOverloadedOperator(decl);
  } else {
    function_name = GetNamedDeclAsString(decl->getCanonicalDecl());
  }
  // main_0 should be static
  if (!decl->isMain()) {
    parts.push_back(ConvertFunctionQualifiers(decl));
  }
  parts.push_back(Text(decl->isConstexpr() ? keyword_const_fn_ : ""));
  parts.push_back(Text(keyword_unsafe_));
  parts.push_back(Text(keyword::kFn));
  parts.push_back(Text(std::move(function_name)));
  parts.push_back(Parens(ConvertFunctionParameters(decl)));
  parts.push_back(ConvertFunctionReturnType(decl));
  {
    auto *preamble = EmitFunctionPreamble(decl);
    auto *body = ConvertFunctionBody(decl);
    parts.push_back(Braces(Cat(preamble, body)));
  }

  if (decl->isOverloadedOperator()) {
    switch (decl->getOverloadedOperator()) {
    case clang::OverloadedOperatorKind::OO_Less: {
      auto type = decl->getParamDecl(0)->getType().getNonReferenceType();
      if (auto cxx_record_decl = type->getAsCXXRecordDecl()) {
        parts.push_back(ConvertOrdAndPartialOrdTraits(cxx_record_decl, decl));
        return arena_.New<Concat>(std::move(parts));
      }
      break;
    }
    default:
      assert(0 && "Unsupported out-of-line operator");
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::EmitHoistedDecls(clang::CompoundStmt *body) {
  std::vector<RsExpr *> parts;
  for (auto *child : body->body()) {
    if (auto *decl_stmt = clang::dyn_cast<clang::DeclStmt>(child)) {
      for (auto *decl : decl_stmt->decls()) {
        if (auto *var = clang::dyn_cast<clang::VarDecl>(decl);
            var && var->isLocalVarDecl() && !IsGlobalVar(var)) {
          hoisted_decls_.insert(var);
          auto [header, proceed] = ConvertVarDeclSkipInit(var);
          if (proceed) {
            parts.push_back(Cat(header, Text(token::kAssign),
                                ConvertVarDefaultInit(var->getType()),
                                Text(token::kSemiColon)));
          }
        }
      }
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::ConvertGotoBlock(clang::CompoundStmt *body) {
  PushHoistedDecls push(hoisted_decls_);
  auto *hoisted = EmitHoistedDecls(body);

  std::vector<RsExpr *> block;
  block.push_back(Text("'__entry: "));
  std::vector<RsExpr *> arm;
  for (auto *child : body->body()) {
    if (auto *label = clang::dyn_cast<clang::LabelStmt>(child)) {
      block.push_back(Braces(arena_.New<Concat>(std::move(arm))));
      arm.clear();
      block.push_back(
          Text(std::format("'{}: ", label->getDecl()->getName().str())));
      arm.push_back(ConvertFullStmt(label->getSubStmt()));
    } else {
      arm.push_back(ConvertFullStmt(child));
    }
  }
  block.push_back(Braces(arena_.New<Concat>(std::move(arm))));

  return Cat(hoisted, Text("goto_block!"),
             Parens(Braces(arena_.New<Concat>(std::move(block)))),
             Text(token::kSemiColon));
}

RsExpr *Converter::ConvertFunctionBody(clang::FunctionDecl *decl) {
  if (auto compound = clang::dyn_cast<clang::CompoundStmt>(decl->getBody())) {
    if (CompoundHasTopLevelLabel(compound)) {
      auto *node = ConvertGotoBlock(compound);
      if (!decl->getReturnType()->isVoidType()) {
        node = Cat(
            node,
            Text(R"(panic!("ub: non-void function does not return a value"))"));
      }
      return node;
    }
  }

  auto *node = ConvertFullStmt(decl->getBody());
  if (!decl->getReturnType()->isVoidType()) {
    if (auto compound = clang::dyn_cast<clang::CompoundStmt>(decl->getBody())) {
      if (!compound->body_empty()) {
        if (!clang::isa<clang::ReturnStmt>(compound->body_back())) {
          node = Cat(
              node,
              Text(
                  R"(panic!("ub: non-void function does not return a value"))"));
        }
      }
    }
  }
  return node;
}

RsExpr *
Converter::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl *decl) {
  std::vector<RsExpr *> parts;
  for (auto *function_decl : decl->specializations()) {
    parts.push_back(VisitFunctionDecl(function_decl));
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::ConvertVaListVarDecl(clang::VarDecl *decl) {
  std::vector<RsExpr *> parts;
  if (clang::isa<clang::ParmVarDecl>(decl)) {
    // va_list parameter (decayed to __va_list_tag *)
  } else {
    // va_list local variable
    parts.push_back(Text(keyword::kLet));
  }
  parts.push_back(Text(keyword_mut_));
  parts.push_back(Text(GetNamedDeclAsString(decl)));
  parts.push_back(Text(token::kColon));
  parts.push_back(Text("VaList"));
  return arena_.New<Concat>(std::move(parts));
}

std::pair<RsExpr *, bool>
Converter::ConvertVarDeclSkipInit(clang::VarDecl *decl) {
  auto qual_type = decl->getType();
  auto name = GetNamedDeclAsString(decl);

  if (IsVaListType(qual_type) && decl->isLocalVarDecl()) {
    return {ConvertVaListVarDecl(decl), true};
  }

  std::vector<RsExpr *> parts;
  if (decl->isFileVarDecl()) {
    if ((decl->isThisDeclarationADefinition() ==
             clang::VarDecl::DeclarationOnly &&
         !decl->hasInit()) ||
        !globals_.insert(name).second) {
      return {Text(""), false};
    }
    parts.push_back(Text(AccessSpecifierAsString(decl->getAccess())));
    parts.push_back(Text(keyword::kStatic));
    parts.push_back(Text(keyword_mut_));
    ENSURE(decl_ids_.insert(GetID(decl)).second);
  } else if (decl->isStaticLocal()) {
    parts.push_back(Text(keyword::kStatic));
    parts.push_back(Text(keyword_mut_));
  } else if (decl->isLocalVarDecl()) {
    parts.push_back(Text(keyword::kLet));
  }

  auto *method_or_null =
      curr_function_ ? clang::dyn_cast<clang::CXXMethodDecl>(curr_function_)
                     : nullptr;
  if ((hoisted_decls_.contains(decl) || !qual_type.isConstQualified()) &&
      !qual_type->isReferenceType() &&
      ((method_or_null == nullptr) || !method_or_null->isVirtual()) &&
      !IsGlobalVar(decl) && name != "_") {
    parts.push_back(Text(keyword_mut_));
  }

  parts.push_back(Text(std::move(name)));
  parts.push_back(Text(token::kColon));

  bool is_parm_with_default_value = false;
  if (auto parm = clang::dyn_cast<clang::ParmVarDecl>(decl)) {
    is_parm_with_default_value = parm->hasDefaultArg();
  }

  if (is_parm_with_default_value) {
    parts.push_back(Text("Option<"));
  }
  parts.push_back(Convert(qual_type));
  if (is_parm_with_default_value) {
    parts.push_back(Text('>'));
  }
  return {arena_.New<Concat>(std::move(parts)), true};
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

RsExpr *Converter::ConvertVarDeclInitializer(clang::VarDecl *decl) {
  if (decl->hasInit()) {
    return ConvertVarInit(decl->getType(), decl->getInit());
  }
  if (!clang::isa<clang::ParmVarDecl>(decl)) {
    return ConvertVarDefaultInit(decl->getType());
  }
  return Text("");
}

RsExpr *Converter::EmitHoistedInArmAssignment(clang::VarDecl *decl) {
  if (!decl->hasInit()) {
    return Text("");
  }
  auto *init = ConvertVarInit(decl->getType(), decl->getInit());
  return Cat(Text(GetNamedDeclAsString(decl)), Text(token::kAssign), init,
             Text(token::kSemiColon));
}

RsExpr *Converter::ConvertVarDecl(clang::VarDecl *decl) {
  if (hoisted_decls_.contains(decl)) {
    return EmitHoistedInArmAssignment(decl);
  }
  auto [header, proceed] = ConvertVarDeclSkipInit(decl);
  if (!proceed) {
    // Skip global variables declared extern
    return Text("");
  }
  PushConstInitializer static_init(*this, decl->isFileVarDecl() ||
                                              decl->isStaticLocal());
  auto *init = ConvertVarDeclInitializer(decl);
  return Cat(header, Text(token::kAssign), init, Text(token::kSemiColon));
}

RsExpr *Converter::ConvertGlobalVarDecl(clang::VarDecl *decl) {
  auto [header, proceed] = ConvertVarDeclSkipInit(decl);
  if (!proceed) {
    // Skip global variables declared extern
    return Text("");
  }
  PushConstInitializer static_init(*this, decl->isFileVarDecl() ||
                                              decl->isStaticLocal());
  auto *init = ConvertVarDeclInitializer(decl);
  return Cat(header, Text(token::kAssign), Text(keyword_unsafe_),
             Braces(init), Text(token::kSemiColon));
}

RsExpr *Converter::VisitVarDecl(clang::VarDecl *decl) {
  if (ConvertLambdaVarDecl(decl)) {
    return arena_.New<Verbatim>("");
  }

  if (IsGlobalVar(decl)) {
    return ConvertGlobalVarDecl(decl);
  }
  return ConvertVarDecl(decl);
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

    // Records that contain libc types do not derive Default
    if (auto record = f->getType()->getAsRecordDecl()) {
      if (ctx_.getSourceManager().isInSystemHeader(record->getLocation()) &&
          f->getType().isPODType(ctx_)) {
        return false;
      }
    }
  }

  return true;
}

bool Converter::RecordDerivesCopy(const clang::RecordDecl *decl) const {
  auto *derives = Mapper::MappedDerives(ctx_.getCanonicalTagType(decl));
  return derives &&
         std::find(derives->begin(), derives->end(), "Copy") != derives->end();
}

bool Converter::RecordHasCopyableFields(const clang::RecordDecl *decl) {
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

    if (auto ptr_ty = f->getType()->getAs<clang::PointerType>()) {
      if (ptr_ty->getPointeeType()->isFunctionType()) {
        if (!FunctionPointerImplementsCopy()) {
          return false;
        }
      }
    }

    // Look recursively into fields that are RecordDecl
    if (auto field_record = f->getType()->getAsRecordDecl()) {
      if (!RecordDerivesCopy(field_record)) {
        return false;
      }
    }
  }

  return true;
}

RsExpr *Converter::VisitRecordDecl(clang::RecordDecl *decl) {
  decl->dump(log());

  // VisitCXXRecordDecl already visited the record
  if (clang::isa<clang::CXXRecordDecl>(decl)) {
    return arena_.New<Verbatim>("");
  }

  if (!decl->isCompleteDefinition()) {
    return arena_.New<Verbatim>("");
  }

  if (!record_decls_.MarkDefined(GetRecordName(decl))) {
    return arena_.New<Verbatim>("");
  }

  Mapper::AddRuleForUserDefinedType(decl);
  return EmitRustStructOrUnion(decl);
}

RsExpr *Converter::EmitRustStructOrUnion(clang::RecordDecl *decl) {
  std::vector<RsExpr *> parts;
  // Enums and static variables. In rust they live outside the record
  for (auto *d : decl->decls()) {
    if (auto *enum_decl = llvm::dyn_cast<clang::EnumDecl>(d)) {
      parts.push_back(VisitEnumDecl(enum_decl));
    }
    if (auto *var_decl = clang::dyn_cast<clang::VarDecl>(d)) {
      parts.push_back(VisitVarDecl(var_decl));
    }
  }

  // Inner records. In rust they live outside the record
  for (auto *d : decl->decls()) {
    if (auto *nested = clang::dyn_cast<clang::RecordDecl>(d)) {
      if (!nested->isImplicit()) {
        inner_structs_[GetID(nested)] = GetRecordName(nested);
        if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(nested)) {
          parts.push_back(VisitCXXRecordDecl(cxx));
        } else {
          parts.push_back(VisitRecordDecl(nested));
        }
      }
    }
  }

  if (decl->isUnion()) {
    parts.push_back(EmitRustUnion(decl));
    return arena_.New<Concat>(std::move(parts));
  }

  // Derived traits
  if (EmitsReprCForRecords()) {
    parts.push_back(Text("#[repr(C)]"));
  }
  auto attrs = GetStructAttributes(decl);
  Mapper::SetDerives(ctx_.getCanonicalTagType(decl),
                     std::vector<std::string>(attrs.begin(), attrs.end()));
  std::string derive = "#[derive(";
  for (auto *attr : attrs) {
    derive += attr;
    derive += ',';
  }
  derive += ")]";
  parts.push_back(Text(std::move(derive)));

  // Fields
  auto access = clang::dyn_cast<clang::CXXRecordDecl>(decl)
                    ? AccessSpecifierAsString(decl->getAccess())
                    : keyword::kPub;
  parts.push_back(Text(access));
  parts.push_back(Text(keyword::kStruct));
  parts.push_back(Text(GetRecordName(decl)));
  std::vector<RsExpr *> fields;
  for (auto *field : decl->fields()) {
    fields.push_back(VisitFieldDecl(field));
  }
  parts.push_back(Braces(arena_.New<Concat>(std::move(fields))));

  // C++ method decls
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    auto struct_name = GetRecordName(cxx);

    parts.push_back(ConvertCXXMethodDecls(
        cxx, std::format("{} {}", keyword::kImpl, struct_name),
        [](auto *method) {
          return !method->isImplicit() &&
                 !(method->getDefinition() &&
                   method->getDefinition()->isDefaulted()) &&
                 (method->isThisDeclarationADefinition() ||
                  clang::isa<clang::CXXConstructorDecl>(method)) &&
                 !method->isVirtual() &&
                 !clang::isa<clang::CXXDestructorDecl>(method);
        }));

    if (cxx->bases_begin() != cxx->bases_end()) {
      parts.push_back(ConvertCXXMethodDecls(
          cxx,
          std::format("{} impl {} for {}", keyword_unsafe_,
                      GetUnsafeTypeAsString(cxx->bases_begin()->getType()),
                      struct_name),
          [](auto *method) {
            return !method->isImplicit() && method->isVirtual();
          }));
    }
  }

  // Traits
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    parts.push_back(AddOrdTrait(cxx));
    parts.push_back(AddDropTrait(cxx));
  }
  parts.push_back(AddCloneTrait(decl));
  parts.push_back(AddDefaultTrait(decl));
  parts.push_back(AddByteReprTrait(decl));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::EmitRustUnion(clang::RecordDecl *decl) {
  std::vector<RsExpr *> parts;
  parts.push_back(Text("#[repr(C)]"));
  auto attrs = GetStructAttributes(decl);
  Mapper::SetDerives(ctx_.getCanonicalTagType(decl),
                     std::vector<std::string>(attrs.begin(), attrs.end()));
  std::string derive = "#[derive(";
  for (auto *attr : attrs) {
    derive += attr;
    derive += ',';
  }
  derive += ")]";
  parts.push_back(Text(std::move(derive)));

  parts.push_back(Text(keyword::kPub));
  parts.push_back(Text(keyword::kUnion));
  parts.push_back(Text(GetRecordName(decl)));
  std::vector<RsExpr *> fields;
  for (auto *field : decl->fields()) {
    fields.push_back(VisitFieldDecl(field));
  }
  parts.push_back(Braces(arena_.New<Concat>(std::move(fields))));

  parts.push_back(AddDefaultTrait(decl));
  parts.push_back(AddByteReprTrait(decl));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
  if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    materializeTemplateSpecialization(decl);
  }

  decl->dump(log());

  Mapper::AddRuleForUserDefinedType(decl);
  if (!IsConvertibleCXXRecordDecl(decl)) {
    return arena_.New<Verbatim>("");
  }

  std::vector<RsExpr *> parts;
  if (decl->isStruct() || decl->isClass()) {
    for (auto c : GetTemplateInstantiatedCtors(decl)) {
      if (!decl_ids_.contains(GetID(c))) {
        parts.push_back(Text(keyword::kImpl));
        parts.push_back(Text(GetRecordName(decl)));
        parts.push_back(Braces(VisitCXXMethodDecl(c)));
      }
    }

    if (!record_decls_.MarkDefined(GetRecordName(decl))) {
      return arena_.New<Concat>(std::move(parts));
    }

    if (decl->isAbstract()) {
      parts.push_back(ConvertAbstractClass(decl));
      return arena_.New<Concat>(std::move(parts));
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

    parts.push_back(EmitRustStructOrUnion(decl));
  } else if (decl->isUnion()) {
    if (!record_decls_.MarkDefined(GetRecordName(decl))) {
      return arena_.New<Concat>(std::move(parts));
    }
    parts.push_back(EmitRustStructOrUnion(decl));
  } else {
    // FIXME: improve error handling
    assert(0 && "unsupported record kind");
  }

  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitCXXMethodDecl(clang::CXXMethodDecl *decl) {
  decl->dump(log());
  if (!IsConvertibleCXXMethodDecl(decl)) {
    return arena_.New<Verbatim>("");
  }
  curr_function_ = decl;

  if (decl->isOutOfLine() && !decl->overridden_methods().empty()) {
    return arena_.New<Verbatim>("");
  }
  bool out_of_line = decl->isOutOfLine();

  RsExpr *inner = nullptr;
  if (auto *ctor = clang::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    inner = VisitCXXConstructorDecl(ctor);
  } else {
    std::vector<RsExpr *> parts;
    if (decl->isStatic() ||
        (!decl->isVirtual() && !decl->getParent()->isAbstract())) {
      parts.push_back(ConvertFunctionQualifiers(decl));
    }
    parts.push_back(Text(keyword_unsafe_));
    parts.push_back(Text(keyword::kFn));

    std::string function_name;
    if (decl->isOverloadedOperator()) {
      function_name = GetOverloadedOperator(decl);
    } else if (IsOverloadedMethod(decl)) {
      function_name = GetOverloadedFunctionName(decl);
    } else {
      function_name = GetNamedDeclAsString(decl);
    }
    parts.push_back(Text(std::move(function_name)));

    std::vector<RsExpr *> param_parts;
    if (!decl->isStatic()) {
      param_parts.push_back(Text(GetSelfMaybeWithMut(decl)));
      param_parts.push_back(Text(token::kComma));
    }
    param_parts.push_back(ConvertFunctionParameters(decl));
    parts.push_back(Parens(arena_.New<Concat>(std::move(param_parts))));
    parts.push_back(ConvertFunctionReturnType(decl));
    if (decl->isPureVirtual()) {
      parts.push_back(Text(token::kSemiColon));
    } else {
      auto *preamble = EmitFunctionPreamble(decl);
      auto *body = ConvertFunctionBody(decl);
      parts.push_back(Braces(Cat(preamble, body)));
    }
    inner = arena_.New<Concat>(std::move(parts));
  }

  if (out_of_line) {
    return Cat(Text(keyword::kImpl), Text(GetRecordName(decl->getParent())),
               Braces(inner));
  }
  return inner;
}

std::string Converter::GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) {
  // This assumes that all overloaded comparison operators are declared const
  return (decl->isConst() || IsOverloadedComparisonOperator(decl))
             ? "&self"
             : std::format("&mut {}", keyword::kSelfValue);
}

RsExpr *Converter::VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) {
  if (decl->isOutOfLine() || decl->isImplicit()) {
    return arena_.New<Verbatim>("");
  }
  curr_function_ = decl;

  if (decl->isCopyOrMoveConstructor()) {
    // FIXME: improve error handling
    assert(0 && "user-defined copy or move constructor are not supported");
  }

  auto *qualifiers = ConvertFunctionQualifiers(decl);
  auto ctor_name = GetRecordName(decl->getParent()) +
                   (GetNumberOfConvertingCtors(decl->getParent()) != 1
                        ? std::to_string(GetCtorIndex(decl))
                        : "");
  auto *params = ConvertFunctionParameters(decl);
  auto *body = ConvertCXXConstructorBody(decl);
  return Cat(qualifiers, Text(keyword_unsafe_), Text(keyword::kFn),
             Text(std::move(ctor_name)), Parens(params), Text(token::kArrow),
             Text("Self"), Braces(body));
}

RsExpr *Converter::ConvertCXXConstructorBody(clang::CXXConstructorDecl *decl) {
  auto *preamble = EmitFunctionPreamble(decl);

  std::vector<RsExpr *> inits;
  const auto *record_decl = decl->getParent();
  auto *definition_or_null = decl->getDefinition();
  assert(definition_or_null);
  auto *definition = clang::cast<clang::CXXConstructorDecl>(definition_or_null);

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
      inits.push_back(Text(std::move(field_name)));
      inits.push_back(Text(token::kColon));
      inits.push_back(ConvertVarInit(field_type, ctor_init_expr));
      curr_init = (curr_init + 1) % definition->getNumCtorInitializers();
    } else {
      inits.push_back(Text(std::move(field_name)));
      inits.push_back(Text(token::kColon));
      inits.push_back(GetDefaultAsString(field_type));
    }
    inits.push_back(Text(token::kComma));
  }

  auto *body = ConvertFullStmt(decl->getBody());
  return Cat(preamble, Text(keyword::kLet), Text("mut"), Text("this"),
             Text(token::kAssign), Text("Self"),
             Braces(arena_.New<Concat>(std::move(inits))),
             Text(token::kSemiColon), body, Text("this"));
}

RsExpr *Converter::VisitFieldDecl(clang::FieldDecl *decl) {
  auto access_spec = AccessSpecifierAsString(decl->getAccess());
  auto field_name = GetNamedDeclAsString(decl);
  auto *type_node = Convert(decl->getType());
  return Cat(Text(access_spec), Text(std::move(field_name)),
             Text(token::kColon), type_node, Text(token::kComma));
}

RsExpr *Converter::EmitFunctionPreamble(clang::FunctionDecl *decl) {
  // In the header, the function might be declared as `int foo(int name_1)',
  // while in the source file the function might be defined as `int foo(int
  // name_2)'. We want to get the parameters from the definition if possible,
  // i.e. name_2.
  auto params = decl->getDefinition() ? decl->getDefinition()->parameters()
                                      : decl->parameters();
  std::vector<RsExpr *> parts;
  for (auto *param : params) {
    if (param->hasDefaultArg()) {
      auto name = GetNamedDeclAsString(param);
      auto *type_node = Convert(param->getType());
      auto *default_arg = ConvertExpr(param->getDefaultArg());
      parts.push_back(Text(std::format("let mut {} :", name)));
      parts.push_back(type_node);
      parts.push_back(Text("="));
      parts.push_back(Text(std::format("{}.unwrap_or(", name)));
      parts.push_back(default_arg);
      parts.push_back(Text(')'));
      parts.push_back(Text(token::kSemiColon));
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitNamespaceDecl(clang::NamespaceDecl *decl) {
  std::vector<RsExpr *> parts;
  for (auto *child : decl->decls()) {
    if (IsInMainFile(child) || !decl_ids_.contains(GetID(child))) {
      parts.push_back(ConvertDecl(child));
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitTypedefDecl([[maybe_unused]] clang::TypedefDecl *decl) {
  return arena_.New<Verbatim>("");
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

RsExpr *Converter::ConvertFullStmt(clang::Stmt *stmt) {
  if (stmt == nullptr) {
    return Text("");
  }
  PushExprKind push(*this, ExprKind::Void);
  auto *node = ConvertStmt(stmt);
  if (IsaSemiColonStmt(stmt)) {
    return Cat(node, Text(token::kSemiColon));
  }
  return node;
}

RsExpr *Converter::ConvertStmt(clang::Stmt *stmt) {
  if (auto *expr = clang::dyn_cast<clang::Expr>(stmt)) {
    return ConvertExpr(expr);
  }
  switch (stmt->getStmtClass()) {
  case clang::Stmt::CompoundStmtClass:
    return VisitCompoundStmt(clang::cast<clang::CompoundStmt>(stmt));
  case clang::Stmt::DeclStmtClass:
    return VisitDeclStmt(clang::cast<clang::DeclStmt>(stmt));
  case clang::Stmt::ReturnStmtClass:
    return VisitReturnStmt(clang::cast<clang::ReturnStmt>(stmt));
  case clang::Stmt::GotoStmtClass:
    return VisitGotoStmt(clang::cast<clang::GotoStmt>(stmt));
  case clang::Stmt::IfStmtClass:
    return VisitIfStmt(clang::cast<clang::IfStmt>(stmt));
  case clang::Stmt::WhileStmtClass:
    return VisitWhileStmt(clang::cast<clang::WhileStmt>(stmt));
  case clang::Stmt::DoStmtClass:
    return VisitDoStmt(clang::cast<clang::DoStmt>(stmt));
  case clang::Stmt::ForStmtClass:
    return VisitForStmt(clang::cast<clang::ForStmt>(stmt));
  case clang::Stmt::CXXForRangeStmtClass:
    return VisitCXXForRangeStmt(clang::cast<clang::CXXForRangeStmt>(stmt));
  case clang::Stmt::BreakStmtClass:
    return VisitBreakStmt(clang::cast<clang::BreakStmt>(stmt));
  case clang::Stmt::ContinueStmtClass:
    return VisitContinueStmt(clang::cast<clang::ContinueStmt>(stmt));
  case clang::Stmt::SwitchStmtClass:
    return VisitSwitchStmt(clang::cast<clang::SwitchStmt>(stmt));
  case clang::Stmt::AttributedStmtClass:
    return ConvertStmt(clang::cast<clang::AttributedStmt>(stmt)->getSubStmt());
  case clang::Stmt::NullStmtClass:
    return arena_.New<Verbatim>("");
  default:
    llvm::errs() << "ConvertStmt: unhandled statement class "
                 << stmt->getStmtClassName() << '\n';
    assert(false && "statement class not handled by ConvertStmt dispatch");
    return arena_.New<Verbatim>("");
  }
}

RsExpr *Converter::VisitCompoundStmt(clang::CompoundStmt *stmt) {
  if (CompoundHasTopLevelLabel(stmt)) {
    return ConvertGotoBlock(stmt);
  }
  std::vector<RsExpr *> parts;
  for (auto *child : stmt->body()) {
    parts.push_back(ConvertFullStmt(child));
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitDeclStmt(clang::DeclStmt *stmt) {
  std::vector<RsExpr *> parts;
  for (auto *decl : stmt->decls()) {
    parts.push_back(ConvertDecl(decl));
    parts.push_back(Text(token::kSemiColon));
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitReturnStmt(clang::ReturnStmt *stmt) {
  auto return_type = curr_function_->getReturnType();
  if (!return_type->isVoidType()) {
    auto *init = ConvertVarInit(return_type, stmt->getRetValue());
    return Cat(Text(keyword::kReturn), init);
  }
  auto *value = ConvertExpr(stmt->getRetValue());
  return Cat(value, Text(token::kSemiColon), Text(keyword::kReturn),
             Text(token::kSemiColon));
}

RsExpr *Converter::VisitGotoStmt(clang::GotoStmt *stmt) {
  return arena_.New<Verbatim>(
      std::format("goto!('{})", stmt->getLabel()->getName().str()));
}

RsExpr *Converter::ConvertCondition(clang::Expr *cond) {
  PushExprKind push(*this, ExprKind::RValue);
  return ConvertExpr(NormalizeToBool(cond, ctx_));
}

RsExpr *Converter::VisitIfStmt(clang::IfStmt *stmt) {
  auto *cond = ConvertCondition(stmt->getCond());
  auto *then = Braces(ConvertFullStmt(stmt->getThen()));
  auto *node = Cat(Text(keyword::kIf), cond, then);
  if (stmt->hasElseStorage()) {
    auto *els = ConvertFullStmt(stmt->getElse());
    if (!clang::isa<clang::IfStmt>(stmt->getElse())) {
      els = Braces(els);
    }
    node = Cat(node, Text(keyword::kElse), els);
  }
  return node;
}

RsExpr *Converter::VisitWhileStmt(clang::WhileStmt *stmt) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  auto *cond = ConvertCondition(stmt->getCond());
  curr_for_inc_.emplace_back(nullptr);
  auto *body = ConvertFullStmt(stmt->getBody());
  curr_for_inc_.pop_back();
  return Cat(Text("'loop_:"), Text(keyword::kWhile), cond, Braces(body));
}

RsExpr *Converter::VisitDoStmt(clang::DoStmt *stmt) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  const char *control_var = "__do_while";
  auto *cond = ConvertCondition(stmt->getCond());
  curr_for_inc_.emplace_back(nullptr);
  auto *body = ConvertFullStmt(stmt->getBody());
  curr_for_inc_.pop_back();
  return Cat(
      Text(keyword::kLet), Text("mut"), Text(control_var),
      Text(token::kAssign), Text(keyword::kTrue), Text(token::kSemiColon),
      Text("'loop_:"), Text(keyword::kWhile), Text(control_var), Text("||"),
      Parens(cond),
      Braces(Cat(Text(control_var), Text(token::kAssign),
                 Text(keyword::kFalse), Text(token::kSemiColon), body)));
}

RsExpr *Converter::VisitForStmt(clang::ForStmt *stmt) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  auto *init = ConvertFullStmt(stmt->getInit());
  RsExpr *cond = nullptr;
  if (stmt->getCond() == nullptr) {
    cond = Text("true");
  } else {
    cond = ConvertCondition(stmt->getCond());
  }
  curr_for_inc_.emplace_back(stmt->getInc());
  auto *body = ConvertFullStmt(stmt->getBody());
  curr_for_inc_.pop_back();
  auto *inc = Cat(ConvertExpr(stmt->getInc()), Text(token::kSemiColon));
  return Cat(init, Text("'loop_:"), Text(keyword::kWhile), cond,
             Braces(Cat(body, inc)));
}

RsExpr *Converter::ConvertLoopVariable(clang::VarDecl *decl,
                                       clang::Expr *range_init) {
  auto loop_var_type = decl->getType();
  auto loop_var_name = GetNamedDeclAsString(decl);

  if (loop_var_type->isReferenceType()) {
    auto pointee_type = loop_var_type->getPointeeType();
    auto *node = ConvertExpr(range_init);
    if (pointee_type.isConstQualified()) {
      return Cat(node, Text(std::format(".as_ptr().add({})", loop_var_name)));
    }
    return Cat(node,
               Text(std::format(".as_mut_ptr().add({})", loop_var_name)));
  }
  RsExpr *node = nullptr;
  {
    PushExplicitAutoref autoref(*this, /*is_mut=*/false);
    node = ConvertExpr(range_init);
  }
  return Cat(node, Text(std::format("[{}]", loop_var_name)),
             Text(".clone()"));
}

RsExpr *Converter::ConvertForRangeBody(clang::CXXForRangeStmt *stmt,
                                       const clang::VarDecl *map_iter_decl) {
  PushBreakTarget push(break_target_, BreakTarget::Loop);
  std::optional<ScopedMapIterDecl> skip;
  if (map_iter_decl)
    skip.emplace(*this, map_iter_decl);
  curr_for_inc_.emplace_back(nullptr);
  auto *body = ConvertFullStmt(stmt->getBody());
  curr_for_inc_.pop_back();
  return body;
}

RsExpr *Converter::VisitCXXForRangeStmt(clang::CXXForRangeStmt *stmt) {
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

RsExpr *Converter::VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  auto map_type = Mapper::Map(stmt->getRangeInit()->getType());
  auto *range_init = ConvertExpr(stmt->getRangeInit());
  auto *body = ConvertForRangeBody(stmt, loop_var);

  return Cat(Text("'loop_:"), Text(keyword::kFor), Text(loop_var_name),
             Text(keyword::kIn), Text("UnsafeMapIterator::begin(&"),
             range_init, Text(std::format(" as *const {})", map_type)),
             Braces(body));
}

RsExpr *Converter::VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt) {
  return VisitCXXForRangeStmtIndexBased(stmt, "len()-1");
}

RsExpr *Converter::VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt) {
  return VisitCXXForRangeStmtIndexBased(stmt, "len()");
}

RsExpr *Converter::VisitCXXForRangeStmtIndexBased(clang::CXXForRangeStmt *stmt,
                                                  const char *len_suffix) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  auto *range_init = ConvertExpr(stmt->getRangeInit());
  auto *range = Parens(Cat(range_init, Text(token::kDot), Text(len_suffix)));

  std::vector<RsExpr *> body_parts;
  body_parts.push_back(Text(keyword::kLet));
  auto loop_var_type = loop_var->getType();
  if (!loop_var_type.isConstQualified()) {
    body_parts.push_back(Text(keyword_mut_));
  }
  body_parts.push_back(Text(loop_var_name));
  body_parts.push_back(Text(token::kAssign));
  body_parts.push_back(ConvertLoopVariable(loop_var, stmt->getRangeInit()));
  body_parts.push_back(Text(token::kSemiColon));
  body_parts.push_back(ConvertForRangeBody(stmt));

  return Cat(Text("'loop_:"), Text(keyword::kFor), Text(loop_var_name),
             Text(keyword::kIn), Text("0.."), range,
             Braces(arena_.New<Concat>(std::move(body_parts))));
}

RsExpr *Converter::VisitBreakStmt([[maybe_unused]] clang::BreakStmt *stmt) {
  if (isSwitchBreak()) {
    return Cat(Text(keyword::kBreak), Text("'switch"));
  }
  return Text(keyword::kBreak);
}

RsExpr *
Converter::VisitContinueStmt([[maybe_unused]] clang::ContinueStmt *stmt) {
  std::vector<RsExpr *> parts;
  if (!curr_for_inc_.empty()) {
    parts.push_back(ConvertExpr(curr_for_inc_.back()));
    parts.push_back(Text(token::kSemiColon));
  }
  parts.push_back(Text(keyword::kContinue));
  parts.push_back(Text("'loop_"));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::ConvertExpr(clang::Expr *expr,
                               std::optional<clang::QualType> ict) {
  if (expr == nullptr) {
    return Text("");
  }
  RsExpr *node = DispatchExpr(expr);
  node->expr = expr;
  if (ict && NeedsImplicitScalarCast(expr->IgnoreImplicit()->getType(), *ict)) {
    node = Parens(
        Cat(node, Text(keyword::kAs), Text(GetUnsafeTypeAsString(*ict))));
    computed_expr_type_ = ComputedExprType::FreshValue;
  }
  return node;
}

RsExpr *Converter::DispatchExpr(clang::Expr *expr) {
  switch (expr->getStmtClass()) {
  case clang::Stmt::IntegerLiteralClass:
    return VisitIntegerLiteral(clang::cast<clang::IntegerLiteral>(expr));
  case clang::Stmt::FloatingLiteralClass:
    return VisitFloatingLiteral(clang::cast<clang::FloatingLiteral>(expr));
  case clang::Stmt::CharacterLiteralClass:
    return VisitCharacterLiteral(clang::cast<clang::CharacterLiteral>(expr));
  case clang::Stmt::StringLiteralClass:
    return VisitStringLiteral(clang::cast<clang::StringLiteral>(expr));
  case clang::Stmt::CXXBoolLiteralExprClass:
    return VisitCXXBoolLiteralExpr(
        clang::cast<clang::CXXBoolLiteralExpr>(expr));
  case clang::Stmt::ParenExprClass:
    return VisitParenExpr(clang::cast<clang::ParenExpr>(expr));
  case clang::Stmt::CXXThisExprClass:
    return VisitCXXThisExpr(clang::cast<clang::CXXThisExpr>(expr));
  case clang::Stmt::CXXNullPtrLiteralExprClass:
    return VisitCXXNullPtrLiteralExpr(
        clang::cast<clang::CXXNullPtrLiteralExpr>(expr));
  case clang::Stmt::GNUNullExprClass:
    return VisitGNUNullExpr(clang::cast<clang::GNUNullExpr>(expr));
  case clang::Stmt::CXXDefaultArgExprClass:
    return VisitCXXDefaultArgExpr(clang::cast<clang::CXXDefaultArgExpr>(expr));
  case clang::Stmt::CXXDefaultInitExprClass:
    return VisitCXXDefaultInitExpr(
        clang::cast<clang::CXXDefaultInitExpr>(expr));
  case clang::Stmt::PredefinedExprClass:
    return VisitPredefinedExpr(clang::cast<clang::PredefinedExpr>(expr));
  case clang::Stmt::DeclRefExprClass:
    return VisitDeclRefExpr(clang::cast<clang::DeclRefExpr>(expr));
  case clang::Stmt::UnaryOperatorClass:
    return VisitUnaryOperator(clang::cast<clang::UnaryOperator>(expr));
  case clang::Stmt::ImplicitCastExprClass:
    return VisitImplicitCastExpr(clang::cast<clang::ImplicitCastExpr>(expr));
  case clang::Stmt::CStyleCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass:
  case clang::Stmt::CXXReinterpretCastExprClass:
  case clang::Stmt::CXXConstCastExprClass:
  case clang::Stmt::CXXFunctionalCastExprClass:
    return VisitExplicitCastExpr(clang::cast<clang::ExplicitCastExpr>(expr));
  case clang::Stmt::StmtExprClass:
    return VisitStmtExpr(clang::cast<clang::StmtExpr>(expr));
  case clang::Stmt::ConditionalOperatorClass:
    return VisitConditionalOperator(
        clang::cast<clang::ConditionalOperator>(expr));
  case clang::Stmt::BinaryOperatorClass:
  case clang::Stmt::CompoundAssignOperatorClass:
    return VisitBinaryOperator(clang::cast<clang::BinaryOperator>(expr));
  case clang::Stmt::MemberExprClass:
    return VisitMemberExpr(clang::cast<clang::MemberExpr>(expr));
  case clang::Stmt::ArraySubscriptExprClass:
    return VisitArraySubscriptExpr(
        clang::cast<clang::ArraySubscriptExpr>(expr));
  case clang::Stmt::CallExprClass:
  case clang::Stmt::CXXMemberCallExprClass:
  case clang::Stmt::CXXOperatorCallExprClass:
    return VisitCallExpr(clang::cast<clang::CallExpr>(expr));
  case clang::Stmt::InitListExprClass:
    return VisitInitListExpr(clang::cast<clang::InitListExpr>(expr));
  case clang::Stmt::CompoundLiteralExprClass:
    return VisitCompoundLiteralExpr(
        clang::cast<clang::CompoundLiteralExpr>(expr));
  case clang::Stmt::ImplicitValueInitExprClass:
    return VisitImplicitValueInitExpr(
        clang::cast<clang::ImplicitValueInitExpr>(expr));
  case clang::Stmt::CXXStdInitializerListExprClass:
    return VisitCXXStdInitializerListExpr(
        clang::cast<clang::CXXStdInitializerListExpr>(expr));
  case clang::Stmt::VAArgExprClass:
    return VisitVAArgExpr(clang::cast<clang::VAArgExpr>(expr));
  case clang::Stmt::CXXNewExprClass:
    return VisitCXXNewExpr(clang::cast<clang::CXXNewExpr>(expr));
  case clang::Stmt::CXXDeleteExprClass:
    return VisitCXXDeleteExpr(clang::cast<clang::CXXDeleteExpr>(expr));
  case clang::Stmt::CXXConstructExprClass:
  case clang::Stmt::CXXTemporaryObjectExprClass:
    return VisitCXXConstructExpr(clang::cast<clang::CXXConstructExpr>(expr));
  case clang::Stmt::UnaryExprOrTypeTraitExprClass:
    return VisitUnaryExprOrTypeTraitExpr(
        clang::cast<clang::UnaryExprOrTypeTraitExpr>(expr));
  case clang::Stmt::TypeTraitExprClass:
    return VisitTypeTraitExpr(clang::cast<clang::TypeTraitExpr>(expr));
  case clang::Stmt::OffsetOfExprClass:
    return VisitOffsetOfExpr(clang::cast<clang::OffsetOfExpr>(expr));
  case clang::Stmt::LambdaExprClass:
    return VisitLambdaExpr(clang::cast<clang::LambdaExpr>(expr));
  case clang::Stmt::RecoveryExprClass:
    return VisitRecoveryExpr(clang::cast<clang::RecoveryExpr>(expr));
  case clang::Stmt::ExprWithCleanupsClass:
  case clang::Stmt::MaterializeTemporaryExprClass:
  case clang::Stmt::ConstantExprClass:
  case clang::Stmt::CXXBindTemporaryExprClass:
  case clang::Stmt::SubstNonTypeTemplateParmExprClass:
    return ConvertExpr(clang::cast<clang::Expr>(*expr->child_begin()));
  case clang::Stmt::ArrayInitLoopExprClass:
    return ConvertExpr(clang::cast<clang::ArrayInitLoopExpr>(expr)
                           ->getCommonExpr()
                           ->getSourceExpr());
  case clang::Stmt::OpaqueValueExprClass:
  case clang::Stmt::ArrayInitIndexExprClass:
    return arena_.New<Verbatim>("");
  default:
    llvm::errs() << "ConvertExpr: unhandled expression class "
                 << expr->getStmtClassName() << '\n';
    assert(false && "expression class not handled by ConvertExpr dispatch");
    return arena_.New<Verbatim>("");
  }
}

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
                          std::vector<RsExpr *> &fmt_args,
                          const char *&fmt_trait, std::string &fmt_width) {
  std::string arg_str = Mapper::ToString(arg);
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
  } else if (arg_str.contains("std::endl")) {
    fmt += "\\n";
  } else if (arg_str.contains("std::hex")) {
    fmt_trait = "x";
  } else if (arg_str.contains("std::dec")) {
    fmt_trait = "";
  } else if (arg_str.contains("Setw")) {
    auto *width_call = clang::dyn_cast<clang::CallExpr>(arg->IgnoreImplicit());
    assert(width_call && "Setw expression is not a call");
    clang::Expr::EvalResult width;
    bool is_const_width = width_call->getArg(0)->EvaluateAsInt(width, ctx_);
    assert(is_const_width && "Setw width is not a constant");
    (void)is_const_width;
    fmt_width = static_cast<std::string>(GetNumAsString(width.Val.getInt()));
  } else if (!arg->getType()->isCharType() &&
             Mapper::Map(arg->getType()) !=
                 std::format("Vec<{}>", CharRustType())) {
    fmt += ("{:" + fmt_width + fmt_trait + "}");
    fmt_width.clear(); // Reset setw after first usage
    auto *arg_node = ConvertExpr(arg);
    if (arg->getType()->isBooleanType()) {
      arg_node = Parens(Cat(arg_node, Text("as u8")));
    }
    fmt_args.push_back(arg_node);
  } else {
    return false;
  }
  return true;
}

bool Converter::GetRawArg(clang::Expr *arg, std::vector<RsExpr *> &raw_args) {
  RsExpr *bytes = nullptr;
  if (arg->getType()->isCharType()) {
    bytes = Cat(Text("&["), ConvertExpr(arg), Text("as u8]"));
  } else if (Mapper::Map(arg->getType()) ==
             std::format("Vec<{}>", CharRustType())) {
    PushExprKind push(*this, ExprKind::RValue);
    auto *str = ConvertExpr(arg);
    bytes = Cat(Text("&"), Parens(str), Text(".iter().take("), Parens(str),
                Text(".len() - 1).map(|&c| c as u8).collect::<Vec<u8>>()[..]"));
  } else if (Mapper::ToString(arg).contains("std::endl")) {
    bytes = Text("&[b'\\n']");
  } else if (clang::isa<clang::StringLiteral>(arg->IgnoreImplicit())) {
    bytes = Text("b" + GetEscapedStringLiteral(arg));
  } else {
    return false;
  }
  raw_args.push_back(Parens(Cat(bytes, Text("as &[u8]"))));
  return true;
}

RsExpr *Converter::ConvertStream(clang::Expr *expr) {
  return ConvertExpr(expr);
}

RsExpr *Converter::ConvertCallToOstream(clang::CallExpr *expr) {
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
    return Text("");
  }

  std::string fmt;
  const char *fmt_trait = "";
  std::string fmt_width;
  std::vector<RsExpr *> fmt_args;
  std::vector<RsExpr *> raw_args;
  auto *stream_node = ConvertStream(stream);
  size_t arg_count = args.size();

  std::vector<RsExpr *> parts;
  auto write_raw_args = [&]() {
    if (!raw_args.empty()) {
      auto *call = Cat(stream_node, Text(".write_all(&(["));
      for (auto *raw_arg : raw_args) {
        call = Cat(call, raw_arg, Text(','));
      }
      parts.push_back(Cat(call, Text("].concat()));")));
      raw_args.clear();
    }
  };

  auto write_fmt_args = [&]() {
    if (!fmt_args.empty() || !fmt.empty()) {
      auto *call = Cat(Text("write!("), stream_node,
                       Text(",\"" + std::move(fmt) + "\","));
      for (auto *fmt_arg : fmt_args) {
        call = Cat(call, fmt_arg, Text(','));
      }
      parts.push_back(Cat(call, Text(");")));
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
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::ConvertPrintf(clang::CallExpr *expr) {
  bool is_fprintf =
      Mapper::ToString(expr->getCallee()).starts_with("int fprintf");

  std::vector<RsExpr *> parts;
  parts.push_back(Text("printf("));
  for (unsigned i = is_fprintf; i < expr->getNumArgs(); ++i) {
    auto *arg = ConvertExpr(expr->getArg(i));
    if (i == is_fprintf ? 1 : 0) {
      parts.push_back(Cat(arg, Text("as *const i8")));
    } else {
      parts.push_back(arg);
    }
    parts.push_back(Text(token::kComma));
  }
  parts.push_back(Text(')'));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::TryPluginConvert(clang::CallExpr *call) {
  if (emplace_back_plugin_match(call)) {
    return emplace_back_plugin_convert(call);
  }
  return nullptr;
}

RsExpr *Converter::ConvertVariadicArg(clang::Expr *arg) {
  if (arg->getType()->isFunctionPointerType()) {
    auto *node = ConvertExpr(arg);
    return Cat(
        node,
        Text(".map_or(::std::ptr::null_mut(), |f| f as *mut ::libc::c_void)"));
  }
  return ConvertExpr(arg);
}

RsExpr *Converter::ConvertVAArgCall(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr)) {
    auto *arg = ConvertExpr(expr->getArg(0)->IgnoreImpCasts());
    return Cat(arg, Text("= VaList::new(__args)"));
  }
  if (IsBuiltinVaEnd(expr)) {
    // va_end is a no-op
    return Text("");
  }
  if (IsBuiltinVaCopy(expr)) {
    auto *dst = ConvertExpr(expr->getArg(0)->IgnoreImpCasts());
    auto *src = ConvertExpr(expr->getArg(1)->IgnoreImpCasts());
    return Cat(dst, Text('='), src, Text(".clone()"));
  }
  return Text("");
}

RsExpr *Converter::VisitCallExpr(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr) || IsBuiltinVaEnd(expr) || IsBuiltinVaCopy(expr)) {
    return ConvertVAArgCall(expr);
  }

  if (auto *plugin_node = TryPluginConvert(expr)) {
    return plugin_node;
  }

  if (Mapper::Contains(expr->getCallee())) {
    if (Mapper::IsLibcPassthrough(GetCalleeOrExpr(expr))) {
      return ConvertGenericCallExpr(expr);
    }

    auto **args = expr->getArgs();
    auto num_args = expr->getNumArgs();
    auto ctx = CollectRefBindingTempArgs(expr);
    RsExpr *node = nullptr;
    {
      PushExprKind push(*this, ExprKind::RValue);
      node = GetMappedAsNode(expr, args, num_args, &ctx);
    };
    if (!node) {
      node = Text("");
    }

    if ((IsReferenceType(expr) ||
         GetReturnTypeOfFunction(expr)->isReferenceType()) &&
        !isAddrOf() && !isVoid()) {
      node = Cat(Text("( *"), node, Text(')'));
    }

    if (!ctx.temporary_bindings.empty()) {
      std::vector<RsExpr *> parts = ctx.temporary_bindings;
      parts.push_back(node);
      node = Braces(arena_.New<Concat>(std::move(parts)));
    }

    return node;
  }

  if (expr->isCallToStdMove()) {
    auto *node = ConvertExpr(expr->getArg(0));
    computed_expr_type_ = ComputedExprType::FreshValue;
    return node;
  }

  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
      opcall && !Mapper::Contains(expr->getCallee())) {
    return ConvertCXXOperatorCallExpr(opcall);
  }

  auto [call_node, ctx] = Converter::ConvertCallExpr(expr);

  auto ty = GetReturnTypeOfFunction(expr);
  auto ref = clang::dyn_cast<clang::ReferenceType>(ty);

  if (ref && !isAddrOf() && !isVoid()) {
    auto *node = Parens(Cat(
        Text(GetPointerDerefPrefix(ref->getPointeeType())), call_node));
    SetValueFreshness(ref->getPointeeType());
    return node;
  }

  return call_node;
}

RsExpr *Converter::EmitFnPtrCall(clang::Expr *callee) {
  return Cat(Parens(ConvertExpr(callee)), Text(".unwrap()"));
}

RsExpr *Converter::ConvertFunctionToFunctionPointer(
    const clang::FunctionDecl *fn_decl) {
  return Text(std::format("Some({})", Mapper::MapFunctionName(fn_decl)));
}

Converter::CallInfo Converter::CollectCallInfo(clang::CallExpr *expr) {
  using Kind = CallArg::Kind;

  CallInfo info;
  info.expr = expr;
  auto callee = GetCallee(expr);
  unsigned arg_begin = 0;
  if (auto op_call = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (op_call->getOperator() == clang::OO_Call) {
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
  assert((function || proto) &&
         "Either function decl or function prototype should be known");

  unsigned num_args = expr->getNumArgs() - arg_begin;
  unsigned num_named_params =
      function ? function->getNumParams() : proto->getNumParams();
  info.is_variadic = function ? function->isVariadic() : proto->isVariadic();
  info.is_fn_ptr_call = !function;
  info.is_libc_passthrough = Mapper::IsLibcPassthrough(GetCalleeOrExpr(expr));

  for (unsigned i = 0; i < num_named_params && i < num_args; ++i) {
    auto *arg = expr->getArg(i + arg_begin);
    CallArg ca{
        .param_name = function
                          ? ("_" + function->getParamDecl(i)->getNameAsString())
                          : ("_arg" + std::to_string(i)),
        .param_type = function ? function->getParamDecl(i)->getType()
                               : proto->getParamType(i),
        .expr = arg,
        .has_default = function && function->getParamDecl(i)->hasDefaultArg(),
        .kind = (IsLiteral(arg) || info.is_libc_passthrough) ? Kind::Inline
                                                             : Kind::Hoisted,
    };
    bool is_materialize = clang::isa<clang::MaterializeTemporaryExpr>(arg);
    if (is_materialize && ca.param_type->isLValueReferenceType()) {
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

RsExpr *Converter::ConvertParamTy(clang::QualType param_type,
                                  clang::Expr *expr) {
  if (param_type->isLValueReferenceType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    return ConvertVarInit(param_type, expr);
  }
  return ConvertVarInit(param_type, expr);
}

RsExpr *Converter::EmitHoistedArgs(CallInfo &info) {
  using Kind = CallArg::Kind;
  std::vector<RsExpr *> parts;
  for (auto &ca : info.args) {
    switch (ca.kind) {
    case Kind::Hoisted: {
      parts.push_back(
          Text(std::format("let {}:", ca.param_name)));
      parts.push_back(Convert(ca.param_type));
      parts.push_back(Text("="));
      parts.push_back(ConvertParamTy(ca.param_type, ca.expr));
      parts.push_back(Text(";"));
      break;
    }
    case Kind::Materialized: {
      auto [binding, ref] =
          MaterializeTemp(ca.param_name, ca.param_type, ca.expr);
      parts.push_back(binding);
      ca.ref_temp_name = ref;
      break;
    }
    case Kind::Inline:
      break;
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::EmitArgList(const CallInfo &info) {
  using Kind = CallArg::Kind;
  std::vector<RsExpr *> parts;

  for (unsigned i = 0; i < info.args.size(); i++) {
    const auto &ca = info.args[i];

    RsExpr *arg_node = nullptr;
    switch (ca.kind) {
    case Kind::Hoisted:
      arg_node = Text(ca.param_name);
      break;
    case Kind::Materialized:
      arg_node = ca.ref_temp_name;
      break;
    case Kind::Inline:
      arg_node = ConvertParamTy(ca.param_type, ca.expr);
      if (info.is_libc_passthrough) {
        arg_node = Cat(arg_node,
                       Text(std::format("as {}",
                                        Mapper::GetParamType(
                                            GetCalleeOrExpr(info.expr), i))));
      }
      break;
    }

    if (ca.has_default) {
      arg_node = Cat(Text("Some"), Parens(arg_node));
    }
    parts.push_back(arg_node);
    parts.push_back(Text(token::kComma));
  }

  if (info.is_variadic) {
    std::vector<RsExpr *> va_parts;
    for (auto *arg : info.variadic_args) {
      auto *node = Parens(ConvertVariadicArg(arg));
      if (!info.is_libc_passthrough) {
        node = Cat(node, Text(".into()"));
      }
      va_parts.push_back(node);
      va_parts.push_back(Text(token::kComma));
    }
    auto *va_node = arena_.New<Concat>(std::move(va_parts));
    if (!info.is_libc_passthrough) {
      parts.push_back(Cat(Text(token::kRef), Brackets(va_node)));
    } else {
      parts.push_back(va_node);
    }
  }

  return Parens(arena_.New<Concat>(std::move(parts)));
}

RsExpr *Converter::EmitCall(CallInfo &&info) {
  auto *hoisted = EmitHoistedArgs(info);

  RsExpr *callee_node = nullptr;
  if (info.is_fn_ptr_call) {
    callee_node = EmitFnPtrCall(GetCallee(info.expr));
  } else if (info.is_libc_passthrough) {
    auto *direct_callee = info.expr->getDirectCallee();
    assert(direct_callee);
    callee_node = Cat(Text("libc::"), Text(direct_callee->getName().str()));
  } else {
    PushExprKind push(*this, ExprKind::Callee);
    callee_node = ConvertExpr(GetCallee(info.expr));
  }

  auto *args_node = EmitArgList(info);
  return Cat(hoisted, callee_node, args_node);
}

RsExpr *Converter::ConvertGenericCallExpr(clang::CallExpr *expr) {
  auto *call = EmitCall(CollectCallInfo(expr));
  return Parens(Cat(Text(keyword_unsafe_), Braces(call)));
}

std::pair<RsExpr *, std::optional<Converter::TempMaterializationCtx>>
Converter::ConvertCallExpr(clang::CallExpr *expr) {
  auto *callee = expr->getCallee();

  if (auto fn = Mapper::ToString(callee);
      fn.starts_with("int printf") || fn.starts_with("int fprintf")) {
    return {ConvertPrintf(expr), std::nullopt};
  }
  if (expr->isCallToStdMove()) {
    return {ConvertExpr(expr->getArg(0)), std::nullopt};
  }
  if (IsBuiltinConstantP(callee)) {
    return {Text(expr->getArg(0)->isCXX11ConstantExpr(ctx_) ? token::kOne
                                                            : token::kZero),
            std::nullopt};
  }
  if (Mapper::Contains(callee)) {
    auto **args = expr->getArgs();
    auto num_args = expr->getNumArgs();
    auto ctx = CollectRefBindingTempArgs(expr);
    auto *node = GetMappedAsNode(expr, args, num_args, &ctx);
    if (!node) {
      node = Text("");
    }
    return {node, std::move(ctx)};
  }
  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    return {ConvertCXXOperatorCallExpr(opcall), std::nullopt};
  }
  return {ConvertGenericCallExpr(expr), std::nullopt};
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
    if (expr->getValue().isZero()) {
      if (auto init = Mapper::MapInitializer(ty); !init.empty()) {
        return init;
      }
    }
    return getTypedLiteral(num_as_string.c_str(), type_as_string);
  }

  return static_cast<std::string>(num_as_string);
}

RsExpr *Converter::VisitIntegerLiteral(clang::IntegerLiteral *expr) {
  computed_expr_type_ = ComputedExprType::FreshValue;
  if (auto *mapped = GetMappedAsNode(expr)) {
    return mapped;
  }
  return Text(getIntegerLiteral(expr, Mapper::Map(expr->getType()) != "i32"));
}

RsExpr *Converter::VisitFloatingLiteral(clang::FloatingLiteral *expr) {
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Text(static_cast<std::string>(GetNumAsString(expr->getValue())));
}

RsExpr *Converter::VisitCharacterLiteral(clang::CharacterLiteral *expr) {
  auto uc = static_cast<unsigned char>(expr->getValue());
  std::string ch = GetEscapedCharLiteral(expr->getValue());
  ch = (uc > 0x7F ? "b'" : "'") + std::move(ch) + '\'';
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Parens(Cat(Text(std::move(ch)), Text(keyword::kAs),
                    Converter::Convert(expr->getType())));
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
  if (uc < 0x20 || uc >= 0x7F) {
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

RsExpr *Converter::VisitStringLiteral(clang::StringLiteral *expr) {
  computed_expr_type_ = ComputedExprType::FreshValue;
  if (!curr_init_type_.empty() && curr_init_type_.back()->isArrayType()) {
    if (auto *arr_ty = ctx_.getAsConstantArrayType(curr_init_type_.back())) {
      uint64_t arr_size = arr_ty->getSize().getZExtValue();
      if (expr->getString().empty()) {
        return Text(std::format("[0 as libc::c_char; {}]", arr_size));
      }
      uint64_t pad = arr_size > expr->getString().size()
                         ? arr_size - expr->getString().size()
                         : 0;
      return Text(std::format("std::mem::transmute(*b{})",
                              GetEscapedStringLiteral(expr, pad)));
    }
    return Text(std::format("std::mem::transmute(*b{})",
                            GetEscapedStringLiteral(expr, 1)));
  }
  if (expr->getString().contains('\0')) {
    std::string out = "(&[";
    for (unsigned char c : expr->getString()) {
      out += getTypedLiteral(std::to_string(c).c_str(), CharRustType()) + ", ";
    }
    out += getTypedLiteral("0", CharRustType()) + "])";
    return Text(std::move(out));
  }
  return Text(std::format("c{}", GetEscapedStringLiteral(expr, 0)));
}

RsExpr *Converter::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr *expr) {
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Text(expr->getValue() ? keyword::kTrue : keyword::kFalse);
}

RsExpr *Converter::ConvertIntegerToEnumeralCast(clang::Expr *to,
                                                clang::Expr *from) {
  // Short circuit `Enum::from(X as i32)` to `X`
  if (auto ref =
          clang::dyn_cast<clang::DeclRefExpr>(from->IgnoreParenImpCasts())) {
    if (auto ec = clang::dyn_cast<clang::EnumConstantDecl>(ref->getDecl())) {
      auto src_enum = clang::dyn_cast<clang::EnumDecl>(ec->getDeclContext());
      auto dst_enum = to->getType()->getAs<clang::EnumType>();
      if (src_enum && dst_enum && dst_enum->getDecl() == src_enum) {
        return Text(std::format("{}::{}", GetRecordName(src_enum),
                                std::string_view(ec->getName())));
      }
    }
  }
  auto *from_node = ConvertExpr(from);
  RsExpr *inner = from_node;
  if (!from->getType()->isSpecificBuiltinType(clang::BuiltinType::Int)) {
    inner = Cat(from_node, Text(keyword::kAs), Text("i32"));
  }
  return Cat(Text(GetUnsafeTypeAsString(to->getType()) + "::from"),
             Parens(inner));
}

RsExpr *Converter::ConvertIntegralToBooleanCast(clang::ImplicitCastExpr *expr) {
  auto sub_expr = expr->getSubExpr();
  auto *stripped = sub_expr->IgnoreParenImpCasts();

  if (auto binop = clang::dyn_cast<clang::BinaryOperator>(stripped)) {
    // Comparisons and logical ops already produces bool, no wrap needed.
    if ((binop->isComparisonOp() || binop->isLogicalOp()) &&
        binop->getType()->isBooleanType()) {
      return ConvertExpr(sub_expr);
    }
  }

  RsExpr *zero = nullptr;
  if (sub_expr->getType()->isEnumeralType()) {
    zero = Text(GetUnsafeTypeAsString(sub_expr->getType()) + "::from(0)");
  } else /* sub_expr->getType()->isIntegerType() */ {
    zero = Text(token::kZero);
  }
  return Parens(Cat(ConvertExpr(sub_expr), Text(token::kDiff), zero));
}

bool Converter::IsCastRedundantInRust(clang::Expr *expr,
                                      clang::QualType target_type) {
  auto target = GetUnsafeTypeAsString(target_type);
  if (const auto *rule = Mapper::GetExprRule(expr)) {
    return rule->return_type.type == target;
  }
  return GetUnsafeTypeAsString(expr->getType()) == target;
}

RsExpr *Converter::VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
  auto *sub_expr = expr->getSubExpr();
  auto type = expr->getType();
  switch (expr->getCastKind()) {
  case clang::CastKind::CK_LValueToRValue: {
    PushExprKind push(*this, ExprKind::RValue);
    auto *node = ConvertExpr(sub_expr);
    SetValueFreshness(type);
    return node;
  }
  case clang::CastKind::CK_ArrayToPointerDecay: {
    // __va_list_tag [1] decays to __va_list_tag *. Just pass through by value
    if (IsVaListType(sub_expr->getType())) {
      return ConvertExpr(sub_expr);
    }
    bool dest_pointee_const =
        expr->getType()->getPointeeType().isConstQualified();
    auto *node = ConvertExpr(sub_expr);
    if (IsStringLiteralExpr(sub_expr)) {
      node = Cat(node, Text(".as_ptr()"));
      if (!dest_pointee_const) {
        node = Cat(node, Text(".cast_mut()"));
      }
      return node;
    }
    return Cat(node,
               Text(dest_pointee_const ? ".as_ptr()" : ".as_mut_ptr()"));
  }
  case clang::CastKind::CK_BitCast: {
    auto *inner = ConvertExpr(sub_expr);
    if (type->isVoidPointerType()) {
      inner = Cat(
          inner, Text(keyword::kAs),
          Text(type->getPointeeType().isConstQualified() ? "*const" : "*mut"),
          ConvertPointeeType(sub_expr->getType()));
    }
    return Parens(Cat(inner, CastTo(type)));
  }
  case clang::CastKind::CK_NoOp: {
    const char *suffix = nullptr;
    if (expr->getType()->isPointerType() &&
        sub_expr->getType()->isPointerType() &&
        !clang::isa<clang::CXXThisExpr>(expr->IgnoreImplicit())) {
      switch (GetConstCastType(expr->getType()->getPointeeType(),
                               sub_expr->getType()->getPointeeType())) {
      case ConstCastType::MutableToConst:
        suffix = ".cast_const()";
        break;
      case ConstCastType::ConstToMutable:
        suffix = ".cast_mut()";
        break;
      default:
        break;
      }
    }
    auto *node = Parens(ConvertExpr(sub_expr), suffix != nullptr);
    if (suffix) {
      return Cat(node, Text(suffix));
    }
    return node;
  }
  case clang::CastKind::CK_FunctionToPointerDecay:
  case clang::CastKind::CK_BuiltinFnToFnPtr: {
    if (isCallee()) {
      return ConvertExpr(sub_expr);
    }
    PushExprKind push(*this, ExprKind::AddrOf);
    return ConvertExpr(sub_expr);
  }
  case clang::CastKind::CK_ConstructorConversion:
  case clang::CastKind::CK_DerivedToBase:
    return ConvertExpr(sub_expr);
  case clang::CastKind::CK_IntegralToBoolean:
    return ConvertIntegralToBooleanCast(expr);
  case clang::CastKind::CK_PointerToBoolean:
    return Cat(Text(token::kNot), ConvertEqualsNullPtr(sub_expr));
  case clang::CastKind::CK_NullToPointer:
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return GetDefaultAsString(type);
  default:
    if (auto *literal = clang::dyn_cast<clang::IntegerLiteral>(sub_expr)) {
      auto type = expr->getType();
      computed_expr_type_ = ComputedExprType::FreshValue;
      return Text(getIntegerLiteral(literal, true, &type));
    }
    // Skip cast if source and target map to the same Rust type.
    if (IsCastRedundantInRust(sub_expr, type)) {
      return ConvertExpr(sub_expr);
    }
    if (type->isEnumeralType() && !sub_expr->getType()->isEnumeralType()) {
      return ConvertIntegerToEnumeralCast(expr, sub_expr);
    }
    if (clang::isa<clang::BinaryOperator>(sub_expr)) {
      return Parens(Cat(Parens(ConvertExpr(sub_expr)), CastTo(type)));
    }
    return Parens(Parens(Cat(ConvertExpr(sub_expr), CastTo(type))));
  }
}

RsExpr *Converter::VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
  auto type = expr->getTypeAsWritten();
  auto *sub_expr = expr->getSubExpr();
  if (type->isVoidType()) {
    PushExprKind push(*this, ExprKind::Void);
    return Cat(Text(token::kRef), Parens(ConvertExpr(expr->getSubExpr())));
  }
  switch (expr->getStmtClass()) {
  case clang::Stmt::CXXReinterpretCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass:
  case clang::Stmt::CStyleCastExprClass: {
    if (expr->getType() == sub_expr->getType()) {
      return ConvertExpr(sub_expr);
    }
    if (type->isFunctionPointerType() ||
        sub_expr->getType()->isFunctionPointerType()) {
      auto *src_type = Convert(sub_expr->getType());
      auto *dst_type = Convert(type);
      auto *sub_node = ConvertExpr(sub_expr);
      return Cat(Text("std::mem::transmute::<"), src_type, Text(','),
                 dst_type, Text(">("), sub_node, Text(')'));
    }
    if (type->isEnumeralType() && !sub_expr->getType()->isEnumeralType()) {
      return ConvertIntegerToEnumeralCast(expr, sub_expr);
    }
    if (type->isBooleanType() && sub_expr->getType()->isIntegerType() &&
        !sub_expr->getType()->isBooleanType()) {
      return Parens(
          Cat(ConvertExpr(sub_expr), Text(token::kDiff), Text(token::kZero)));
    }
    auto *inner = ConvertExpr(sub_expr);
    if (auto *unary_oper = clang::dyn_cast<clang::UnaryOperator>(sub_expr);
        unary_oper && unary_oper->getOpcode() == clang::UO_AddrOf &&
        (clang::isa<clang::ArraySubscriptExpr>(unary_oper->getSubExpr()) ||
         clang::isa<clang::CXXOperatorCallExpr>(unary_oper->getSubExpr()))) {
      inner = Cat(inner, CastTo(sub_expr->getType()));
    }
    return Parens(Cat(inner, CastTo(type)));
  }
  default:
    return ConvertExpr(sub_expr);
  }
}

RsExpr *Converter::VisitBinaryOperator(clang::BinaryOperator *expr) {
  bool needs_cast = (expr->isComparisonOp() || expr->isLogicalOp()) &&
                    expr->getType()->isIntegerType() &&
                    !expr->getType()->isBooleanType();
  auto *node = ConvertBinaryOperator(expr);
  if (needs_cast) {
    return Parens(Cat(Parens(node), CastTo(expr->getType())));
  }
  return node;
}

RsExpr *Converter::ConvertBinaryOperator(clang::BinaryOperator *expr) {
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
    auto computation_result_type = cmpd_assign_op->getComputationResultType();
    RsExpr *node = nullptr;
    if (IsUnsignedArithOp(cmpd_assign_op)) {
      auto *lhs_node = ConvertExpr(lhs);
      auto *lhs_again = ConvertExpr(lhs);
      auto *arith = ConvertUnsignedArithBinaryOperator(expr, rhs);
      node = Cat(lhs_node, Text(token::kAssign),
                 Parens(Cat(Parens(Cat(lhs_again,
                                       CastTo(computation_result_type))),
                            arith)));
    } else {
      auto *lhs_node = ConvertExpr(lhs);
      auto *lhs_again = ConvertExpr(lhs);
      auto op = opcode_as_string;
      op.remove_suffix(1); // remove '=' from operator
      auto *rhs_node = ConvertExpr(rhs, computation_result_type);
      node = Cat(lhs_node, Text(token::kAssign),
                 Parens(Cat(Parens(Cat(lhs_again,
                                       CastTo(computation_result_type))),
                            Text(std::string(op)), rhs_node)));
    }
    if (lhs_type->isBooleanType()) {
      return Cat(node, Text(token::kDiff), Text(token::kZero));
    }
    return Cat(node, CastTo(lhs_type));
  }
  if (expr->isCommaOp()) {
    RsExpr *lhs_node = nullptr;
    {
      PushExprKind push(*this, ExprKind::Void);
      lhs_node = ConvertExpr(lhs);
    }
    auto *rhs_node = ConvertExpr(rhs);
    return Cat(lhs_node, Text(token::kSemiColon), rhs_node);
  }
  if (IsUnsignedArithOp(expr)) {
    RsExpr *prefix = nullptr;
    if (expr->isCompoundAssignmentOp()) {
      prefix = Cat(ConvertExpr(lhs), Text(token::kAssign));
    }
    auto *operand = Parens(ConvertUnsignedArithOperand(lhs, type));
    auto *arith = ConvertUnsignedArithBinaryOperator(expr, rhs);
    if (!expr->isCompoundAssignmentOp()) {
      computed_expr_type_ = ComputedExprType::FreshValue;
    }
    if (prefix) {
      return Cat(prefix, operand, arith);
    }
    return Cat(operand, arith);
  }
  if (expr->isAssignmentOp()) {
    if (expr->isCompoundAssignmentOp() &&
        expr->getLHS()->getType()->isPointerType() &&
        expr->getRHS()->getType()->isIntegralOrEnumerationType()) {
      auto *lhs_node = ConvertExpr(lhs);
      auto *operand = Parens(ConvertUnsignedArithOperand(lhs, type));
      auto *arith = ConvertUnsignedArithBinaryOperator(expr, rhs);
      auto *node = Cat(lhs_node, Text(token::kAssign), operand, arith);
      if (!isVoid()) {
        node = Cat(node, Text(token::kSemiColon), ConvertRValue(lhs));
      }
      return Braces(node, !isVoid());
    }
    return ConvertAssignment(lhs, rhs, opcode_as_string);
  }
  if (IsComparisonWithNullOp(expr)) {
    if (expr->getOpcode() == clang::BO_EQ) {
      return ConvertEqualsNullPtr(lhs);
    }
    return Cat(Text(token::kNot), Parens(ConvertEqualsNullPtr(lhs)));
  }
  if (expr->isAdditiveOp() && expr->getType()->isPointerType()) {
    auto [base, idx] = lhs_type->isPointerType() ? std::make_tuple(lhs, rhs)
                                                 : std::make_tuple(rhs, lhs);
    return ConvertPointerOffset(base, idx,
                                expr->getOpcode() == clang::BO_Add);
  }
  if (expr->isAdditiveOp() && lhs_type->isPointerType() &&
      rhs_type->isPointerType()) {
    auto *lhs_node = ConvertExpr(lhs);
    auto *rhs_node = ConvertExpr(rhs);
    auto *pointee_type_node = ConvertPointeeType(lhs_type);
    auto *size_of_node = Cat(Text("::std::mem::size_of::<"), pointee_type_node,
                             Text(">()"));
    auto *diff = Parens(Cat(lhs_node, Text(keyword::kAs), Text("usize"),
                            Text(token::kMinus), rhs_node, Text(keyword::kAs),
                            Text("usize")));
    auto *node = Parens(Cat(diff, Text(token::kDiv), size_of_node));
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Cat(node, CastTo(expr->getType()));
  }
  if (expr->isLogicalOp()) {
    auto *lhs_node = Parens(ConvertCondition(expr->getLHS()));
    auto *rhs_node = Parens(ConvertCondition(expr->getRHS()));
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Cat(lhs_node, Text(std::string(expr->getOpcodeStr())), rhs_node);
  }
  return ConvertGenericBinaryOperator(expr);
}

RsExpr *Converter::ConvertGenericBinaryOperator(clang::BinaryOperator *expr) {
  auto *lhs = expr->getLHS();
  auto *rhs = expr->getRHS();

  auto *lhs_node =
      ConvertExpr(lhs, GetOperandImplicitConversionTarget(expr, lhs, rhs));
  auto *rhs_node =
      ConvertExpr(rhs, GetOperandImplicitConversionTarget(expr, rhs, lhs));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Parens(Cat(Parens(lhs_node), Text(std::string(expr->getOpcodeStr())),
                    Parens(rhs_node)));
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

RsExpr *Converter::ConvertIncAndDec(clang::UnaryOperator *expr) {
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
    return nullptr;
  }
  RsExpr *node = nullptr;
  {
    PushExprKind push(*this, ExprKind::RValue);
    node = ConvertExpr(sub_expr);
  }
  SetFresh();
  return Cat(node, Text(std::format(".{}()", method)));
}

RsExpr *Converter::VisitUnaryOperator(clang::UnaryOperator *expr) {
  if (auto *mapped = GetMappedAsNode(expr)) {
    return mapped;
  }

  auto opcode = expr->getOpcode();
  auto *sub_expr = expr->getSubExpr();
  if (auto *node = ConvertIncAndDec(expr)) {
    return node;
  }
  switch (opcode) {
  case clang::UO_Extension:
    return ConvertExpr(sub_expr);
  case clang::UO_AddrOf:
    return Parens(ConvertAddrOf(sub_expr, expr->getType()));
  case clang::UO_Deref:
    return ConvertDeref(sub_expr);
  case clang::UO_Not: {
    auto *node = ConvertExpr(sub_expr);
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Cat(Text(token::kNot), node);
  }
  case clang::UO_LNot: {
    bool needs_int_cast =
        expr->getType()->isIntegerType() && !expr->getType()->isBooleanType();
    auto *cond = ConvertCondition(sub_expr);
    computed_expr_type_ = ComputedExprType::FreshValue;
    auto *node = Cat(Text(token::kNot), cond);
    if (needs_int_cast) {
      return Parens(Cat(node, CastTo(expr->getType())));
    }
    return node;
  }
  case clang::UO_Minus:
    if (auto *literal = clang::dyn_cast<clang::IntegerLiteral>(sub_expr)) {
      computed_expr_type_ = ComputedExprType::FreshValue;
      if (sub_expr->getType()->isUnsignedIntegerType()) {
        return Text(std::format(
            "(-{}_i{} as {})", getIntegerLiteral(literal, false),
            ctx_.getTypeSize(expr->getType()),
            GetUnsafeTypeAsString(expr->getType())));
      }
      return Cat(Text(token::kMinus), Text(getIntegerLiteral(literal, true)));
    }
    [[fallthrough]];
  default: {
    auto *node = ConvertExpr(sub_expr);
    return Cat(Text(std::string(expr->getOpcodeStr(opcode))), node);
  }
  }
}

RsExpr *Converter::VisitStmtExpr(clang::StmtExpr *expr) {
  auto *body = expr->getSubStmt();
  std::vector<RsExpr *> parts;
  auto stmts = body->body();
  size_t n = static_cast<size_t>(stmts.end() - stmts.begin());
  size_t i = 0;
  for (auto *s : stmts) {
    ++i;
    if (i == n) {
      if (auto *tail = clang::dyn_cast<clang::Expr>(s)) {
        parts.push_back(EmitStmtExprTail(tail));
        continue;
      }
    }
    parts.push_back(ConvertFullStmt(s));
  }
  return Braces(arena_.New<Concat>(std::move(parts)));
}

RsExpr *Converter::EmitStmtExprTail(clang::Expr *tail) {
  return ConvertExpr(tail);
}

RsExpr *Converter::VisitConditionalOperator(clang::ConditionalOperator *expr) {
  auto *cond = ConvertCondition(expr->getCond());
  bool branch_is_addr =
      expr->isLValue() && !isRValue() && !expr->getType()->isFunctionType();

  RsExpr *then_node = nullptr;
  {
    PushExplicitAutoref no_autoref(*this, branch_is_addr ? std::nullopt
                                                         : autoref_mut_);
    then_node = ConvertExpr(expr->getTrueExpr(),
                            branch_is_addr
                                ? std::nullopt
                                : std::make_optional(expr->getType()));
  }
  if (branch_is_addr) {
    then_node = Cat(Text(token::kRef), Text(keyword_mut_), then_node);
  }

  RsExpr *else_node = nullptr;
  {
    PushExplicitAutoref no_autoref(*this, branch_is_addr ? std::nullopt
                                                         : autoref_mut_);
    else_node = ConvertExpr(expr->getFalseExpr(),
                            branch_is_addr
                                ? std::nullopt
                                : std::make_optional(expr->getType()));
  }
  if (branch_is_addr) {
    else_node = Cat(Text(token::kRef), Text(keyword_mut_), else_node);
  }

  return Cat(Text(keyword::kIf), cond, Braces(then_node), Text(keyword::kElse),
             Braces(else_node));
}

RsExpr *Converter::ConvertDeclRefExpr(clang::DeclRefExpr *expr) {
  if (isAddrOf()) {
    clang::Expr *addrof_op = ToAddrOf(ctx_, expr);
    if (auto *mapped = GetMappedAsNode(addrof_op)) {
      return mapped;
    }
  }

  auto *decl = expr->getDecl();
  if (ShouldReplaceWithMappedBody(expr)) {
    if (auto *mapped = GetMappedAsNode(expr)) {
      return mapped;
    }
  }

  if (auto *function = decl->getAsFunction()) {
    if (auto method = clang::dyn_cast<clang::CXXMethodDecl>(function)) {
      if (method->isStatic()) {
        return Text(std::format("{}::{}", GetRecordName(method->getParent()),
                                GetNamedDeclAsString(method)));
      }
    }
    return Text(GetNamedDeclAsString(function->getCanonicalDecl()));
  }

  if (auto enum_constant = clang::dyn_cast<clang::EnumConstantDecl>(decl)) {
    auto qualified = std::format("{}::{}",
                                 GetRecordName(clang::dyn_cast<clang::EnumDecl>(
                                     enum_constant->getDeclContext())),
                                 std::string_view(enum_constant->getName()));
    if (!expr->getType()->isEnumeralType()) {
      return Text(std::format("({} as i32)", qualified));
    }
    return Text(std::move(qualified));
  }

  if (IsGlobalVar(expr)) {
    return Text(GetNamedDeclAsString(expr->getDecl()));
  }

  return Text(GetNamedDeclAsString(decl));
}

RsExpr *Converter::VisitDeclRefExpr(clang::DeclRefExpr *expr) {
  auto *name = ConvertDeclRefExpr(expr);
  auto decl = expr->getDecl();

  if (decl->getType()->getAs<clang::ReferenceType>() && !isAddrOf() &&
      !map_iter_decls_.contains(clang::dyn_cast<clang::VarDecl>(decl))) {
    auto *node = EmitDeref(name, decl->getType().getNonReferenceType());
    SetValueFreshness(expr->getType());
    return node;
  }

  if (auto *fn_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (isAddrOf()) {
      return ConvertFunctionToFunctionPointer(fn_decl);
    }
  }

  if (auto var_decl = clang::dyn_cast<clang::VarDecl>(decl)) {
    if (!var_decl->getType()->isFunctionPointerType()) {
      if (auto init = var_decl->getInit()) {
        if (auto lambda = clang::dyn_cast<clang::LambdaExpr>(
                init->IgnoreUnlessSpelledInSource())) {
          return Parens(VisitLambdaExpr(lambda));
        }
      }
    }
  }

  if (!decl->getType()->getAs<clang::ReferenceType>() && isAddrOf()) {
    return Cat(Text(token::kRef),
               Text(decl->getType().isConstQualified() ? "" : keyword_mut_),
               name);
  }

  return name;
}

RsExpr *Converter::VisitParenExpr(clang::ParenExpr *expr) {
  // Comma operator becomes (A, B, C) -> { A; B; C }
  if (auto *bin = clang::dyn_cast<clang::BinaryOperator>(expr->getSubExpr())) {
    if (bin->isCommaOp()) {
      return Braces(ConvertExpr(expr->getSubExpr()));
    }
  }

  return Parens(ConvertExpr(expr->getSubExpr()));
}

RsExpr *Converter::ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) {
  switch (expr->getOperator()) {
  case clang::OverloadedOperatorKind::OO_Equal:
    return ConvertAssignment(expr->getArg(0), expr->getArg(1), "=");
  case clang::OverloadedOperatorKind::OO_Star:
  case clang::OverloadedOperatorKind::OO_Arrow:
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      return ConvertUniquePtrDeref(expr);
    }
    if (GetStrongestIteratorCategory(expr->getArg(0)->getType()) ==
        IteratorCategory::Bidirectional) {
      return ConvertExpr(expr->getArg(0));
    }
    if (expr->getOperator() == clang::OverloadedOperatorKind::OO_Star) {
      return Parens(
          Cat(Text(token::kStar), ConvertExpr(expr->getArg(0))));
    }
    return ConvertExpr(expr->getArg(0));
  case clang::OverloadedOperatorKind::OO_Subscript: {
    PushExplicitAutoref autoref(*this, IsMutatingCall(expr));
    return ConvertArraySubscript(expr->getArg(0), expr->getArg(1),
                                 expr->getType());
  }
  case clang::OverloadedOperatorKind::OO_LessLess:
    if (IsCallToOstream(expr)) {
      return ConvertCallToOstream(expr);
    }
    return Text("");
  case clang::OverloadedOperatorKind::OO_Call:
    return ConvertGenericCallExpr(expr);
  case clang::OverloadedOperatorKind::OO_Less: {
    RsExpr *node = Text("");
    if (auto callee = expr->getDirectCallee()) {
      if (clang::isa<clang::CXXMethodDecl>(callee)) {
        auto *lhs = ConvertExpr(expr->getArg(0));
        if (callee->isUserProvided()) {
          auto *rhs = ConvertPointer(expr->getArg(1));
          node = Cat(lhs, Text(token::kDot),
                     Text(GetOverloadedOperator(callee)), Parens(rhs));
        } else {
          auto *rhs = ConvertExpr(expr->getArg(1));
          node = Cat(lhs, Text(token::kLt), rhs);
        }
      } else {
        auto *lhs = ConvertFreshPointer(expr->getArg(0));
        auto *rhs = ConvertFreshPointer(expr->getArg(1));
        node = Cat(Text(GetOverloadedOperator(callee)),
                   Parens(Cat(lhs, Text(token::kComma), rhs)));
      }
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
    return node;
  }
  default:
    // FIXME: improve error handling
    llvm::errs() << "unsupported CXXOperatorCallExpr: "
                 << clang::getOperatorSpelling(expr->getOperator()) << '\n';
    assert(0);
    return Text("");
  }
}

RsExpr *Converter::VisitMemberExpr(clang::MemberExpr *expr) {
  auto *member = expr->getMemberDecl();
  auto *node = Converter::ConvertMemberExpr(expr);

  if (isAddrOf()) {
    bool is_reference_type = member->getType()->isReferenceType();
    if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member)) {
      is_reference_type |= method->getReturnType()->isReferenceType();
    }

    if (is_reference_type) {
      computed_expr_type_ = ComputedExprType::Pointer;
      return node;
    }
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Cat(Text(token::kRef), node);
  }

  if (!isAddrOf() && member->getType()->isReferenceType()) {
    return EmitDeref(node, member->getType().getNonReferenceType());
  }

  if (!isAddrOf() && member->getType()->isFunctionPointerType()) {
    return Parens(node);
  }

  return node;
}

// Returns the inner member and the replacement string.
static std::pair<clang::MemberExpr *, std::string>
replaceNonUniformLibcField(clang::MemberExpr *expr) {
  // Example: ::struct stat::st_mtim::tv_sec -> ::libc::stat::st_mtime
  struct Mapping {
    const char *record;
    const char *inner_field;
    const char *leaf_field;
    const char *replacement;
  };
  static constexpr Mapping kFields[] = {
      {"stat", "st_mtim", "tv_sec", "st_mtime"},      // Linux
      {"stat", "st_mtimespec", "tv_sec", "st_mtime"}, // macOS
      {"in6_addr", "__in6_u", "__u6_addr8", "s6_addr"},
  };

  auto getNamedIdentifierOrNull = [](auto *decl) {
    return decl && decl->getDeclName().isIdentifier() ? decl : nullptr;
  };

  if (auto leaf = getNamedIdentifierOrNull(expr->getMemberDecl())) {
    if (auto inner = clang::dyn_cast<clang::MemberExpr>(
            expr->getBase()->IgnoreParenImpCasts())) {
      if (auto field = getNamedIdentifierOrNull(
              clang::dyn_cast<clang::FieldDecl>(inner->getMemberDecl()))) {
        if (getNamedIdentifierOrNull(field->getParent())) {
          for (const auto &m : kFields) {
            if (field->getParent()->getName() == m.record &&
                field->getName() == m.inner_field &&
                leaf->getName() == m.leaf_field) {
              return {inner, m.replacement};
            }
          }
        }
      }
    }
  }
  return {nullptr, ""};
}

RsExpr *Converter::ConvertMemberExpr(clang::MemberExpr *expr) {
  if (auto *mapped = GetMappedAsNode(expr)) {
    if (Mapper::ReturnsPointer(expr)) {
      return Cat(Text(token::kStar), mapped);
    }
    return mapped;
  }

  auto *member = expr->getMemberDecl();
  auto [inner, name_override] = replaceNonUniformLibcField(expr);
  if (inner) {
    expr = inner;
  }

  auto *base = expr->getBase();
  bool base_is_this = clang::isa<clang::CXXThisExpr>(base->IgnoreCasts());
  PushExprKind push(*this, isLValue() ? ExprKind::LValue : ExprKind::RValue);
  RsExpr *base_node = nullptr;
  if (expr->isArrow() && !base_is_this) {
    base_node = ConvertArrow(base);
  } else {
    base_node = ConvertExpr(base);
  }

  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && IsOverloadedMethod(method)) {
    return Cat(base_node, Text(token::kDot),
               Text(GetOverloadedFunctionName(method)));
  }
  if (!name_override.empty()) {
    return Cat(base_node, Text(token::kDot), Text(std::move(name_override)));
  }
  if (member->getDeclName().isIdentifier()) {
    return Cat(base_node, Text(token::kDot),
               Text(GetNamedDeclAsString(member)));
  }
  return base_node;
}

RsExpr *Converter::VisitCXXThisExpr([[maybe_unused]] clang::CXXThisExpr *expr) {
  if (clang::isa<clang::CXXConstructorDecl>(curr_function_)) {
    return Text("this");
  }
  return Text(keyword::kSelfValue);
}

RsExpr *Converter::VisitInitListExpr(clang::InitListExpr *expr) {
  if (auto form = expr->getSemanticForm())
    expr = form;

  auto qual_type = expr->getType();
  if (qual_type->isScalarType()) {
    assert(expr->getNumInits() < 2 && "Excess elements in scalar initializer");
    if (expr->getNumInits() > 0) {
      auto init = expr->getInit(0);
      return ConvertVarInit(init->getType(), init);
    }
    return GetDefaultAsString(qual_type);
  }
  if (qual_type->isRecordType()) {
    const auto *record = qual_type->getAsRecordDecl();
    if (record->getQualifiedNameAsString() == "std::array") {
      if (auto init = clang::dyn_cast<clang::InitListExpr>(expr->getInit(0))) {
        return Cat(Text("vec!"), VisitInitListExpr(init));
      }
      return Cat(Text("vec!"), Text("[]"));
    }

    std::vector<RsExpr *> fields;
    int i = 0;
    for (const auto *field : record->fields()) {
      fields.push_back(Text(GetNamedDeclAsString(field)));
      fields.push_back(Text(token::kColon));
      fields.push_back(ConvertVarInit(field->getType(), expr->getInit(i++)));
      fields.push_back(Text(token::kComma));
    }
    return Cat(Text(GetUnsafeTypeAsString(qual_type)),
               Braces(arena_.New<Concat>(std::move(fields))));
  }
  if (IsInitExprOfStringLiteral(expr)) {
    return ConvertExpr(expr->getInit(0)->IgnoreParenImpCasts());
  }
  std::vector<RsExpr *> elems;
  for (auto *init : expr->inits()) {
    elems.push_back(ConvertVarInit(init->getType(), init));
    elems.push_back(Text(token::kComma));
  }
  if (expr->hasArrayFiller()) {
    if (auto arr_ty = ctx_.getAsConstantArrayType(expr->getType())) {
      assert((arr_ty->getSize().getZExtValue() - expr->getNumInits()) &&
             "Number of initializers should be less than total size of array");
      for (unsigned i = 0;
           i < arr_ty->getSize().getZExtValue() - expr->getNumInits(); ++i) {
        elems.push_back(ConvertVarInit(expr->getArrayFiller()->getType(),
                                       expr->getArrayFiller()));
        elems.push_back(Text(token::kComma));
      }
    }
  }
  return Brackets(arena_.New<Concat>(std::move(elems)));
}

RsExpr *Converter::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr *expr) {
  auto record = expr->getType()->getAsRecordDecl();
  if (!record || !record->hasAttr<clang::TransparentUnionAttr>()) {
    return ConvertExpr(expr->getInitializer());
  }
  auto init = clang::cast<clang::InitListExpr>(expr->getInitializer());
  assert(init->getNumInits() == 1);
  PushExprKind push(*this, ExprKind::RValue);
  return ConvertExpr(init->getInit(0));
}

RsExpr *Converter::VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  if (base->IgnoreCasts()->getType()->isPointerType() ||
      clang::isa<clang::StringLiteral>(base->IgnoreCasts())) {
    return ConvertPointerSubscript(expr);
  }
  return ConvertArraySubscript(base, expr->getIdx(), expr->getType());
}

RsExpr *
Converter::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr *expr) {
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return Text(token::kDefault);
}

RsExpr *Converter::VisitVAArgExpr(clang::VAArgExpr *expr) {
  auto va_list_expr = expr->getSubExpr();
  if (auto *cast = clang::dyn_cast<clang::ImplicitCastExpr>(va_list_expr)) {
    va_list_expr = cast->getSubExpr();
  }
  if (expr->getType()->isFunctionPointerType()) {
    auto *type_node = Convert(expr->getType());
    RsExpr *va_list = nullptr;
    {
      PushExprKind push(*this, ExprKind::RValue);
      va_list = ConvertExpr(va_list_expr);
    }
    SetFreshType(expr->getType());
    return Cat(Text("std::mem::transmute::<*mut ::libc::c_void"),
               Text(token::kComma), type_node, Text('>'),
               Parens(Cat(va_list, Text(".arg::<*mut ::libc::c_void>()"))));
  }
  auto *va_list = ConvertExpr(va_list_expr);
  auto *type_node = Convert(expr->getType());
  SetFreshType(expr->getType());
  return Cat(va_list, Text(".arg::<"), type_node, Text(">()"));
}

RsExpr *Converter::VisitGNUNullExpr(clang::GNUNullExpr *expr) {
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return Text(token::kDefault);
}

RsExpr *Converter::VisitCXXNewExpr(clang::CXXNewExpr *expr) {
  if (expr->isArray()) {
    RsExpr *node = nullptr;
    if (auto *init = llvm::dyn_cast_or_null<clang::InitListExpr>(
            expr->getInitializer())) {
      node = Cat(Text("Box::leak(Box::new("), ConvertExpr(init), Text("))"));
    } else {
      assert(expr->getArraySize().has_value());
      auto *array_size = ConvertExpr(*expr->getArraySize());
      auto *alloc_type = Convert(expr->getAllocatedType());
      auto *default_value = GetDefaultAsString(expr->getAllocatedType());
      node = Cat(Text("Box::leak((0.."), array_size, Text(").map(|_|"),
                 default_value, Text(").collect::<Box<["), alloc_type,
                 Text("]>>())"));
    }
    if (!curr_init_type_.empty() && curr_init_type_.back()->isPointerType()) {
      node = Cat(node, Text(".as_mut_ptr()"));
    }
    return node;
  }
  auto *initializer = ConvertExpr(expr->getInitializer());
  auto *type_node = Convert(expr->getType());
  return Cat(Text("(Box::leak(Box::new("), initializer, Text(")) as"),
             type_node, Text(')'));
}

RsExpr *Converter::VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) {
  auto *argument = ConvertExpr(expr->getArgument());
  if (expr->isArrayForm()) {
    auto destroyed_type = expr->getDestroyedType();
    auto *type_node = Convert(destroyed_type);
    if (destroyed_type.isConstQualified()) {
      return Cat(
          Text("::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts("),
          argument, Text(", libcc2rs::malloc_usable_size("), argument,
          Text(" as *mut ::libc::c_void) / ::std::mem::size_of::<"),
          type_node, Text(">()) as *const ["), type_node, Text("] as *mut ["),
          type_node, Text("]))"));
    }
    return Cat(
        Text(
            "::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut("),
        argument, Text(", libcc2rs::malloc_usable_size("), argument,
        Text(" as *mut ::libc::c_void) / ::std::mem::size_of::<"), type_node,
        Text(">())))"));
  }
  return Cat(Text("::std::mem::drop(Box::from_raw("), argument, Text("))"));
}

RsExpr *
Converter::ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) {
  auto *args = ConvertCXXConstructExprArgs(expr);
  return Cat(Text(std::format("std::array::from_fn::<_, {}, _>",
                              GetArraySize(expr->getType()))),
             Parens(Cat(Text("|_|"), args)));
}

RsExpr *Converter::ConvertCXXConstructExprArgs(clang::CXXConstructExpr *expr) {
  auto ctor = expr->getConstructor();
  auto ctor_name = GetRecordName(ctor->getParent());

  std::vector<RsExpr *> parts;
  unsigned arg_idx = 0;
  for (unsigned param_idx = 0; param_idx < ctor->getNumParams(); ++param_idx) {
    auto param = ctor->getParamDecl(param_idx);
    auto param_type = param->getType();
    bool has_default = param->hasDefaultArg();

    if (arg_idx < expr->getNumArgs()) {
      clang::Expr *arg = expr->getArg(arg_idx++);

      if (has_default) {
        auto *init = ConvertVarInit(param_type, arg);
        parts.push_back(Cat(Text("Some("), init, Text(')')));
      } else {
        parts.push_back(ConvertVarInit(param_type, arg));
      }
    } else {
      assert(has_default);
      parts.push_back(Text("None"));
    }
    parts.push_back(Text(token::kComma));
  }

  return Cat(Text(ctor_name), Text(token::kDoubleColon),
             Text(ctor_name +
                  (GetNumberOfConvertingCtors(ctor->getParent()) != 1
                       ? std::to_string(GetCtorIndex(ctor))
                       : "")),
             Parens(arena_.New<Concat>(std::move(parts))));
}

RsExpr *Converter::VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
  PushSuppressIteratorClone push(*this, expr);

  if (auto *mapped =
          GetMappedAsNode(expr, expr->getArgs(), expr->getNumArgs())) {
    return mapped;
  }

  auto *ctor = expr->getConstructor();
  if (ctor->isCopyOrMoveConstructor() ||
      (ctor->isConvertingConstructor(false) && ctor->getNumParams() == 1 &&
       ctor->getParamDecl(0)->getType()->isRValueReferenceType())) {
    // Take suppress before recursing into the child.
    bool suppress = PushSuppressIteratorClone::take(*this);
    auto *node = ConvertExpr(expr->getArg(0));
    if (ctor->isCopyConstructor() && !suppress) {
      return Cat(node, Text(".clone()"));
    }
    return node;
  }

  if (ctor->isDefaultConstructor() && !ctor->isUserProvided()) {
    auto ty = expr->getType();
    return GetDefaultAsString(ty);
  }

  assert(ctor->isUserProvided());
  if (expr->getType()->isArrayType()) {
    return ConvertArrayCXXConstructExpr(expr);
  }
  return ConvertCXXConstructExprArgs(expr);
}

RsExpr *Converter::VisitUnaryExprOrTypeTraitExpr(
    clang::UnaryExprOrTypeTraitExpr *expr) {
  switch (expr->getKind()) {
  case clang::UnaryExprOrTypeTrait::UETT_SizeOf:
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Text(std::format(
        "::std::mem::size_of::<{}>()",
        GetUnsafeTypeAsString(expr->isArgumentType()
                                  ? expr->getArgumentType()
                                  : expr->getArgumentExpr()->getType())));
  default:
    // FIXME: improve error handling
    log() << "unsupported unary expr or type trait expr\n";
    return Text("");
  }
}

RsExpr *Converter::VisitTypeTraitExpr(clang::TypeTraitExpr *expr) {
  clang::Expr::EvalResult result;
  ENSURE(expr->EvaluateAsInt(result, ctx_));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Text(std::to_string(result.Val.getInt().getExtValue()));
}

RsExpr *Converter::VisitOffsetOfExpr(clang::OffsetOfExpr *expr) {
  std::string member_path;
  for (unsigned i = 0; i < expr->getNumComponents(); ++i) {
    const clang::OffsetOfNode &node = expr->getComponent(i);
    ENSURE(node.getKind() == clang::OffsetOfNode::Field);
    if (!member_path.empty()) {
      member_path += '.';
    }
    member_path += GetNamedDeclAsString(node.getField());
  }
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Text(
      std::format("::std::mem::offset_of!({}, {})",
                  GetUnsafeTypeAsString(expr->getTypeSourceInfo()->getType()),
                  member_path));
}

RsExpr *Converter::VisitEnumDecl(clang::EnumDecl *decl) {
  ENSURE(decl_ids_.insert(GetID(decl)).second);
  if (Mapper::Contains(ctx_.getCanonicalTagType(decl))) {
    return arena_.New<Verbatim>("");
  }
  Mapper::AddRuleForUserDefinedType(decl);
  Mapper::SetDerives(ctx_.getCanonicalTagType(decl),
                     {"Clone", "Copy", "PartialEq", "Debug", "Default"});
  std::vector<RsExpr *> parts;
  parts.push_back(Text("#[derive(Clone, Copy, PartialEq, Debug, Default)]"));
  parts.push_back(Text(std::format("enum {}", GetRecordName(decl))));
  std::vector<RsExpr *> enumerators;
  bool first_enumerator = true;
  for (auto e : decl->enumerators()) {
    llvm::SmallVector<char, 32> init;
    e->getInitVal().toString(init, 10);
    if (first_enumerator) {
      enumerators.push_back(Text("#[default]"));
      first_enumerator = false;
    }
    enumerators.push_back(
        Text(std::format("{} = {},", std::string_view(e->getName()),
                         std::string_view(init.data(), init.size()))));
  }
  parts.push_back(Braces(arena_.New<Concat>(std::move(enumerators))));

  parts.push_back(AddFromImpl(decl));
  parts.push_back(AddIncDecImpls(decl));
  parts.push_back(AddByteReprTrait(decl));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::AddFromImpl(clang::EnumDecl *decl) {
  auto name = GetRecordName(decl);
  std::vector<RsExpr *> arms;
  for (auto e : decl->enumerators()) {
    llvm::SmallVector<char, 32> init;
    e->getInitVal().toString(init, 10);
    arms.push_back(
        Text(std::format("{} => {}::{},",
                         std::string_view(init.data(), init.size()), name,
                         std::string_view(e->getName()))));
  }
  arms.push_back(
      Text(std::format("_ => panic!(\"invalid {} value: {{}}\", n),", name)));

  return Cat(
      Text(std::format("impl From<i32> for {}", name)),
      Braces(Cat(Text(std::format("fn from(n: i32) -> {}", name)),
                 Braces(Cat(Text("match n"),
                            Braces(arena_.New<Concat>(std::move(arms))))))));
}

RsExpr *Converter::AddIncDecImpls(clang::EnumDecl *decl) {
  return Text(
      std::format("libcc2rs::impl_enum_inc_dec!({});", GetRecordName(decl)));
}

RsExpr *Converter::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) {
  if (expr->getType()->isPointerType()) {
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Text(token::kDefault);
  }
  return Text("");
}

RsExpr *Converter::VisitLambdaExpr(clang::LambdaExpr *expr) {
  bool some_wrap = isAddrOf() && expr->capture_size() == 0;

  std::vector<RsExpr *> parts;
  parts.push_back(Text('|'));
  for (auto p :
       expr->getLambdaClass()->getLambdaCallOperator()->parameters()) {
    parts.push_back(Text(GetNamedDeclAsString(p)));
    parts.push_back(Text(token::kColon));
    parts.push_back(Convert(p->getType()));
    parts.push_back(Text(token::kComma));
  }
  parts.push_back(Text("| {"));
  {
    parts.push_back(
        EmitFunctionPreamble(expr->getLambdaClass()->getLambdaCallOperator()));
    // TODO: replace with a stack
    auto old_function = curr_function_;
    curr_function_ = expr->getLambdaClass()->getLambdaCallOperator();
    parts.push_back(ConvertFunctionBody(curr_function_));
    curr_function_ = old_function;
  }
  parts.push_back(Text('}'));

  auto *node = Parens(arena_.New<Concat>(std::move(parts)));
  if (some_wrap) {
    return Cat(Text("Some"), node);
  }
  return node;
}

RsExpr *
Converter::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) {
  if (auto arr_ty = clang::dyn_cast<clang::ArrayType>(
          expr->getType()->getCanonicalTypeInternal().getTypePtr())) {
    if (auto const_arr_ty = clang::dyn_cast<clang::ConstantArrayType>(arr_ty)) {
      auto elem_ty = const_arr_ty->getElementType();
      computed_expr_type_ = ComputedExprType::FreshValue;
      if (elem_ty->isIntegerType() && !elem_ty->isEnumeralType()) {
        return Text(
            std::format("[0; {}]", const_arr_ty->getSize().getZExtValue()));
      }
      return Text(
          std::format("std::array::from_fn::<_, {}, _>(|_| Default::default())",
                      const_arr_ty->getSize().getZExtValue()));
    }
  }

  return GetDefaultAsString(expr->getType());
}

RsExpr *Converter::ConvertSwitchCaseCondition(clang::SwitchCase *stmt) {
  clang::Stmt *cur = stmt;
  clang::SwitchCase *last = nullptr;
  bool first = true;

  std::vector<RsExpr *> parts;
  while (auto *sc = clang::dyn_cast<clang::SwitchCase>(cur)) {
    if (auto *case_stmt = clang::dyn_cast<clang::CaseStmt>(sc)) {
      if (!first) {
        parts.push_back(Text("|| __v == "));
      }
      parts.push_back(ConvertExpr(case_stmt->getLHS()));
    }
    last = sc;
    first = false;
    cur = sc->getSubStmt();
  }

  if (clang::isa<clang::CaseStmt>(last)) {
    parts.push_back(Text(" => "));
  } else /* DefaultStmt */ {
    parts.push_back(Text("_ => "));
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::EmitSwitchArm(const SwitchArm &arm, bool is_default) {
  std::vector<RsExpr *> parts;
  if (is_default) {
    parts.push_back(Text("_ => "));
  } else {
    parts.push_back(Text("__v if __v == "));
    parts.push_back(ConvertSwitchCaseCondition(arm.head));
  }
  if (!arm.label.empty()) {
    parts.push_back(Text(std::format("'{}: ", arm.label.str())));
  }
  std::vector<RsExpr *> body;
  for (auto *t : arm.body) {
    body.push_back(ConvertFullStmt(t));
  }
  parts.push_back(Braces(arena_.New<Concat>(std::move(body))));
  parts.push_back(Text(token::kComma));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitSwitchStmt(clang::SwitchStmt *stmt) {
  auto *body = clang::dyn_cast<clang::CompoundStmt>(stmt->getBody());
  assert(body);
  auto arms = AnalyzeSwitchArms(body);

  bool needs_switch_macro = std::ranges::any_of(arms, [](const SwitchArm &arm) {
    return !arm.label.empty() || arm.has_fallthrough;
  });

  PushBreakTarget push(break_target_, needs_switch_macro
                                          ? BreakTarget::FallthroughSwitch
                                          : BreakTarget::Switch);

  std::vector<RsExpr *> parts;
  if (needs_switch_macro) {
    auto *cond = ConvertExpr(stmt->getCond());
    parts.push_back(Text("match"));
    parts.push_back(cond);
  } else {
    auto *cond = ConvertRValue(stmt->getCond());
    parts.push_back(Text("let __match_cond ="));
    parts.push_back(cond);
    parts.push_back(Text(token::kSemiColon));
    parts.push_back(Text("match __match_cond"));
  }

  std::vector<RsExpr *> match_arms;
  const SwitchArm *default_arm = nullptr;
  for (const auto &arm : arms) {
    if (arm.is_default_case) {
      default_arm = &arm;
      continue;
    }
    match_arms.push_back(EmitSwitchArm(arm, /*is_default=*/false));
  }

  if (default_arm) {
    match_arms.push_back(EmitSwitchArm(*default_arm, /*is_default=*/true));
  } else {
    match_arms.push_back(Text(R"( _ => {})"));
  }

  parts.push_back(Braces(arena_.New<Concat>(std::move(match_arms))));
  auto *node = arena_.New<Concat>(std::move(parts));
  if (needs_switch_macro) {
    return Cat(Text("switch!"), Parens(node));
  }
  return Cat(Text("'switch:"), Braces(node));
}

// TODO: right now defaults go into the constructor, but they should also be
// placed in the Default trait impl.
RsExpr *Converter::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr *expr) {
  return ConvertExpr(expr->getExpr());
}

RsExpr *Converter::VisitPredefinedExpr(clang::PredefinedExpr *expr) {
  return ConvertExpr(expr->getFunctionName());
}

RsExpr *Converter::VisitClassTemplateDecl(clang::ClassTemplateDecl *decl) {
  std::vector<RsExpr *> parts;
  for (auto decl : decl->specializations()) {
    parts.push_back(VisitCXXRecordDecl(decl));
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::VisitCXXStdInitializerListExpr(
    clang::CXXStdInitializerListExpr *expr) {
  auto *sub = ConvertExpr(expr->getSubExpr());
  if (expr->getSubExpr()->getType()->isArrayType()) {
    // Arrays become Vec's
    return Cat(Text("vec!"), sub);
  }
  return sub;
}

RsExpr *Converter::GetArrayDefaultAsString(clang::QualType qual_type) {
  if (auto *array_type = clang::dyn_cast<clang::ConstantArrayType>(qual_type)) {
    auto size_as_string = GetNumAsString(array_type->getSize());
    auto element_type = array_type->getElementType();
    auto *element_default = GetDefaultAsString(element_type);
    if (auto *rec = element_type->getAsRecordDecl()) {
      if (!RecordDerivesCopy(rec)) {
        return Cat(Text(std::format("std::array::from_fn::<_, {}, _>(|_|",
                                    size_as_string.c_str())),
                   element_default, Text(')'));
      }
    }
    return Cat(Text('['), element_default,
               Text(std::format("; {}]", size_as_string.c_str())));
  }
  if (auto *array_type =
          clang::dyn_cast<clang::IncompleteArrayType>(qual_type)) {
    return GetDefaultAsString(array_type->getElementType());
  }
  if (Mapper::ToString(qual_type).contains("std::array")) {
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
    return Text(std::format(
        "std::array::from_fn::<_, {}, _>(|_| Default::default()).to_vec()",
        size));
  }
  return nullptr;
}

RsExpr *Converter::GetDefaultAsString(clang::QualType qual_type) {
  if (IsVaListType(qual_type)) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Text("VaList::default()");
  }

  if (auto *arr = GetArrayDefaultAsString(qual_type)) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return arr;
  }

  if (auto init = Mapper::MapInitializer(qual_type); !init.empty()) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Text(std::move(init));
  }

  if (qual_type->isPointerType()) {
    auto pointee = qual_type->getPointeeType();
    if (pointee->isFunctionType()) {
      return Text("None");
    }
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Text(pointee.isConstQualified() ? "std::ptr::null()"
                                           : "std::ptr::null_mut()");
  }

  computed_expr_type_ = ComputedExprType::FreshValue;
  return GetDefaultAsStringFallback(qual_type);
}

RsExpr *Converter::GetDefaultAsStringFallback(clang::QualType qual_type) {
  qual_type = qual_type.getUnqualifiedType().getCanonicalType();

  if (qual_type->isBooleanType()) {
    return Text("false");
  }

  if (qual_type->isIntegerType() && !qual_type->isEnumeralType()) {
    return Text(getTypedLiteral("0", GetUnsafeTypeAsString(qual_type)));
  }

  if (qual_type->isFloatingType()) {
    return Text(getTypedLiteral("0.0", GetUnsafeTypeAsString(qual_type)));
  }

  if (auto record = qual_type->getAsRecordDecl();
      record && in_const_initializer_) {
    if (auto cxx = clang::dyn_cast<clang::CXXRecordDecl>(record)) {
      ENSURE(GetUserDefinedDefaultConstructor(cxx) == nullptr &&
             "Default initializing globals using default constructor is not "
             "supported");
    }
    return EmitDefaultStructLiteral(record);
  }

  if (auto record = qual_type->getAsRecordDecl()) {
    if (ctx_.getSourceManager().isInSystemHeader(record->getLocation()) &&
        qual_type.isPODType(ctx_)) {
      return Cat(Text("unsafe { std::mem::zeroed::<"), Convert(qual_type),
                 Text(">() }"));
    }
  }

  if (qual_type->isEnumeralType()) {
    auto enum_decl = qual_type->castAs<clang::EnumType>()->getDecl();
    return Text(std::format(
        "{}::{}", GetRecordName(enum_decl),
        std::string_view(enum_decl->enumerator_begin()->getName())));
  }

  return Cat(Text('<'), Convert(qual_type), Text(">::default()"));
}

RsExpr *Converter::ConvertVarDefaultInit(clang::QualType qual_type) {
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
  return Mapper::ToRustName(Mapper::ToString(Mapper::GetTypeForDecl(decl)));
}

std::vector<const char *>
Converter::GetStructAttributes(const clang::RecordDecl *decl) {
  if (decl->isUnion()) {
    return {"Copy", "Clone"};
  }

  std::vector<const char *> struct_attrs;

  if (RecordHasCopyableFields(decl)) {
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
  return std::string(Trim(converter.Convert(qual_type)->print()));
}

RsExpr *Converter::ConvertVarInit(clang::QualType qual_type,
                                  clang::Expr *expr) {
  std::vector<RsExpr *> parts;
  if (qual_type->isReferenceType() && !IsReferenceType(expr)) {
    parts.push_back(Text(token::kRef));
    if (IsMut(qual_type)) {
      parts.push_back(Text(keyword_mut_));
    }
  }
  if (qual_type->isFunctionPointerType()) {
    if (auto *lambda = clang::dyn_cast<clang::LambdaExpr>(
            expr->IgnoreUnlessSpelledInSource())) {
      PushExprKind push(*this, ExprKind::AddrOf);
      PushInitType init_type(*this, qual_type);
      parts.push_back(VisitLambdaExpr(lambda));
      return arena_.New<Concat>(std::move(parts));
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
    RsExpr *inner = nullptr;
    {
      PushInitType init_type(*this, qual_type);
      inner = ConvertExpr(expr);
    }
    parts.push_back(Cat(Parens(Cat(Text(token::kStar), inner)),
                        Text(".clone()")));
  } else if (IsReferenceType(expr) || qual_type->isFunctionPointerType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    PushInitType init_type(*this, qual_type);
    parts.push_back(ConvertExpr(expr, qual_type));
  } else {
    PushExprKind push(*this, ExprKind::RValue);
    PushInitType init_type(*this, qual_type);
    parts.push_back(ConvertExpr(expr, qual_type));
  }
  if (qual_type->isReferenceType() && !IsReferenceType(expr)) {
    parts.push_back(Text(keyword::kAs));
    parts.push_back(Convert(qual_type));
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::ConvertUnsignedArithOperand(clang::Expr *expr,
                                               clang::QualType type) {
  bool needs_cast = (expr->isIntegerConstantExpr(ctx_) &&
                     !clang::isa<clang::ImplicitCastExpr>(expr)) ||
                    Mapper::Map(expr->getType()) != Mapper::Map(type);
  auto *node = ConvertExpr(expr);
  if (needs_cast) {
    return Parens(Cat(node, CastTo(type)));
  }
  return node;
}

RsExpr *Converter::ConvertEqualsNullPtr(clang::Expr *expr) {
  auto *node = ConvertExpr(expr);
  const char *suffix = IsUniquePtr(expr->getType()) ||
                               expr->getType()->isFunctionPointerType()
                           ? ").is_none()"
                           : ").is_null()";
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Cat(Text('('), node, Text(suffix));
}

RsExpr *Converter::ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  auto *idx = expr->getIdx();
  if (isAddrOf()) {
    return ConvertPointerOffset(base, idx);
  }
  return Parens(Cat(Text(token::kStar), ConvertPointerOffset(base, idx)));
}

RsExpr *Converter::ConvertPointerOffset(clang::Expr *base, clang::Expr *idx,
                                        bool is_addition) {
  auto *base_node = ConvertExpr(base);
  RsExpr *idx_node = nullptr;
  {
    PushExprKind push(*this, ExprKind::RValue);
    idx_node = ConvertExpr(idx);
  }
  RsExpr *offset = Cat(Parens(idx_node), Text(keyword::kAs), Text("isize"));
  if (!is_addition) {
    offset = Cat(Text(token::kMinus), Parens(offset));
  }
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return Cat(base_node, Text(token::kDot), Text("offset"), Parens(offset));
}

static bool IsFlexibleArrayMemberAccess(clang::ASTContext &ctx,
                                        clang::Expr *array) {
  return array->isFlexibleArrayMemberLike(
      ctx, clang::LangOptions::StrictFlexArraysLevelKind::OneZeroOrIncomplete,
      /*IgnoreTemplateOrMacroSubstitution=*/true);
}

RsExpr *Converter::EmitFlexibleArrayElementPtr(clang::Expr *array,
                                               clang::Expr *idx, bool is_mut) {
  RsExpr *array_node = nullptr;
  {
    PushExplicitAutoref no_autoref(*this, std::nullopt);
    array_node = ConvertExpr(array);
  }
  auto *idx_node = ConvertExpr(idx);
  return Cat(array_node, Text(is_mut ? ".as_mut_ptr()" : ".as_ptr()"),
             Text(".add"),
             Parens(Cat(Parens(idx_node), Text(keyword::kAs), Text("usize"))));
}

RsExpr *Converter::ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                                         clang::QualType type) {
  if (auto inner = base->IgnoreImplicit()) {
    if (inner->getType()->isArrayType() &&
        IsFlexibleArrayMemberAccess(ctx_, inner)) {
      auto *elem = EmitFlexibleArrayElementPtr(
          inner, idx, !inner->getType().isConstQualified());
      return Parens(Cat(Text(token::kStar), elem));
    }
  }
  RsExpr *base_node = nullptr;
  if (IsUniquePtr(base->getType())) {
    PushExplicitAutoref no_autoref(*this, std::nullopt);
    base_node =
        Cat(ConvertExpr(base->IgnoreImplicit()), Text(".as_mut().unwrap()"));
  } else {
    base_node = ConvertExpr(base->IgnoreImplicit());
  }
  PushExplicitAutoref no_autoref(*this, std::nullopt);
  auto *idx_node = Parens(ConvertExpr(idx));
  if (Mapper::Map(idx->getType()) != "usize") {
    idx_node = Cat(idx_node, Text(keyword::kAs), Text("usize"));
  }
  return Cat(base_node, Brackets(idx_node));
}

RsExpr *Converter::ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                     std::string_view assign_operator) {
  RsExpr *lhs_node = nullptr;
  {
    PushInitType init_type(*this, lhs->getType());
    lhs_node = ConvertLValue(lhs);
  }
  auto *rhs_node = ConvertFreshRValue(rhs, lhs->getType());

  auto *node = Cat(lhs_node, Text(std::string(assign_operator)), rhs_node);
  if (!isVoid()) {
    node = Cat(node, Text(token::kSemiColon), ConvertRValue(lhs));
  }
  return Braces(node, !isVoid());
}

RsExpr *Converter::ConvertFunctionParameters(clang::FunctionDecl *decl) {
  in_function_formals_ = true;
  auto *definition =
      decl->getDefinition() != nullptr ? decl->getDefinition() : decl;
  std::vector<RsExpr *> parts;
  for (auto *parameter : definition->parameters()) {
    parts.push_back(ConvertVarDeclSkipInit(parameter).first);
    parts.push_back(Text(token::kComma));
  }
  if (decl->isVariadic()) {
    parts.push_back(Text("__args: &[VaArg]"));
    parts.push_back(Text(token::kComma));
  }
  in_function_formals_ = false;
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::ConvertFunctionQualifiers(clang::FunctionDecl *decl) {
  return Text(AccessSpecifierAsString(decl->getAccess()));
}

RsExpr *Converter::ConvertFunctionReturnType(clang::FunctionDecl *decl) {
  auto return_type = decl->getReturnType();
  if (!return_type->isVoidType()) {
    return Cat(Text(token::kArrow), Convert(return_type));
  }
  return Text("");
}

RsExpr *
Converter::ConvertFunctionMain(const clang::FunctionDecl *decl,
                               const std::string_view main_function_name) {
  if (decl->getNumParams() != 0U) {
    return Text(std::format(R"(
pub fn main() {{
    let mut args: Vec<Vec<u8>> = std::env::args().map(|arg| arg.as_bytes().to_vec()).collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args.iter().map(|arg| arg.as_ptr() as *mut libc::c_char).collect();
    argv.push(::std::ptr::null_mut());
    unsafe {{
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }}
}})",
                            main_function_name));
  }
  return Text(std::format(
      "pub fn main() {{ unsafe {{ std::process::exit({}() as i32); }} }}",
      main_function_name));
}

RsExpr *Converter::ConvertAbstractClass(clang::CXXRecordDecl *decl) {
  ENSURE(abstract_structs_.insert(GetID(decl)).second);
  auto trait_name = GetRecordName(decl);
  auto access_specifier_as_string = AccessSpecifierAsString(decl->getAccess());
  auto signature = std::format("{} {} trait {}", access_specifier_as_string,
                               keyword_unsafe_, trait_name);
  auto predicate = [](auto *method) {
    return !method->isImplicit() &&
           !clang::isa<clang::CXXDestructorDecl>(method);
  };
  return ConvertCXXMethodDecls(decl, signature, predicate);
}

RsExpr *Converter::ConvertCXXMethodDecls(
    const clang::CXXRecordDecl *decl, const std::string_view signature,
    bool (*predicate)(clang::CXXMethodDecl *)) {
  std::vector<RsExpr *> methods;
  for (auto *method : decl->methods()) {
    if (predicate(method)) {
      methods.push_back(VisitCXXMethodDecl(method));
    }
  }
  if (methods.empty()) {
    return Text("");
  }
  return Cat(Text(std::string(signature)),
             Braces(arena_.New<Concat>(std::move(methods))));
}

RsExpr *Converter::ConvertOrdAndPartialOrdTraitsBase(
    std::string_view first_branch, std::string_view second_branch,
    std::string_view first_return, std::string_view second_return,
    std::string_view record_name) {
  std::vector<RsExpr *> parts;
  parts.push_back(Text(keyword::kImpl));
  parts.push_back(Text("Ord for "));
  parts.push_back(Text(std::string(record_name)));
  parts.push_back(Text('{'));
  parts.push_back(Text("fn cmp(&self, other: &Self) -> std::cmp::Ordering {"));
  parts.push_back(Text(std::format("{} {{", keyword_unsafe_)));
  parts.push_back(Text(std::format(
      "if {} {{ {} }} else if {} {{ {} }} else {{ std::cmp::Ordering::Equal }}",
      first_branch, first_return, second_branch, second_return)));
  parts.push_back(Text("}}}"));

  parts.push_back(Text(keyword::kImpl));
  parts.push_back(Text("PartialOrd for"));
  parts.push_back(Text(std::string(record_name)));
  parts.push_back(Text('{'));
  parts.push_back(Text(R"(
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
    }
  })"));

  parts.push_back(Text(keyword::kImpl));
  parts.push_back(Text("PartialEq for"));
  parts.push_back(Text(std::string(record_name)));
  parts.push_back(Text('{'));
  parts.push_back(Text("fn eq(&self, other: &Self) -> bool {"));
  parts.push_back(Text(std::format("{} {{", keyword_unsafe_)));
  parts.push_back(Text(std::format("!( {} ) && !( {} )", first_branch,
                                   second_branch)));
  parts.push_back(Text("}}}"));

  parts.push_back(Text(keyword::kImpl));
  parts.push_back(Text("Eq for"));
  parts.push_back(Text(std::string(record_name)));
  parts.push_back(Text("{}"));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *
Converter::ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                         const clang::FunctionDecl *op) {
  std::string first_branch, second_branch, first_return, second_return;

  switch (op->getOverloadedOperator()) {
  case clang::OO_Less:
    if (clang::isa<clang::CXXMethodDecl>(op)) {
      first_branch = std::format("self.{}(other)", GetOverloadedOperator(op));
      second_branch = std::format("other.{}(self)", GetOverloadedOperator(op));
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

  return ConvertOrdAndPartialOrdTraitsBase(first_branch, second_branch,
                                           first_return, second_return,
                                           GetRecordName(decl));
}

RsExpr *Converter::AddOrdTrait(const clang::CXXRecordDecl *decl) {
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
    return Text("");
  }

  if (methods.size() > 1) {
    llvm::errs()
        << "Currently supporting only one overloaded comparison operator\n";
    abort();
  }

  return ConvertOrdAndPartialOrdTraits(decl, methods[0]);
}

RsExpr *Converter::AddCloneTrait(const clang::RecordDecl *decl) {
  return Text("");
}

RsExpr *Converter::AddDropTrait(const clang::CXXRecordDecl *decl) {
  return Text("");
}

RsExpr *Converter::AddDefaultTraitForUnion(const clang::RecordDecl *decl) {
  return Cat(Text(std::format("impl Default for {}", GetRecordName(decl))),
             Braces(Cat(Text("fn default() -> Self"),
                        Braces(Cat(Text("unsafe"),
                                   Braces(Text("std::mem::zeroed()")))))));
}

RsExpr *Converter::AddDefaultTrait(const clang::RecordDecl *decl) {
  if (decl->isUnion()) {
    return AddDefaultTraitForUnion(decl);
  }
  if (RecordDerivesDefault(decl)) {
    return Text("");
  }
  auto struct_name = GetRecordName(decl);

  RsExpr *body = nullptr;
  if (auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (auto *default_ctor = GetUserDefinedDefaultConstructor(cxx)) {
      auto *ctor_call = ConvertExpr(clang::CXXConstructExpr::Create(
          ctx_, ctx_.getCanonicalTagType(decl), clang::SourceLocation(),
          default_ctor,
          /*Elidable=*/false, llvm::ArrayRef<clang::Expr *>(),
          /*HadMultipleCandidates=*/false,
          /*ListInitialization=*/false,
          /*StdInitListInitialization=*/false,
          /*ZeroInitialization=*/false, clang::CXXConstructionKind::Complete,
          clang::SourceRange()));
      body = Cat(Text(keyword_unsafe_), Braces(ctor_call));
    }
  }
  if (!body) {
    body = EmitDefaultStructLiteral(decl);
  }

  return Cat(Text(std::format("impl Default for {}", struct_name)),
             Braces(Cat(Text("fn default() -> Self"), Braces(body))));
}

RsExpr *Converter::EmitDefaultStructLiteral(const clang::RecordDecl *decl) {
  std::vector<RsExpr *> fields;
  for (auto *field : decl->fields()) {
    fields.push_back(Text(GetNamedDeclAsString(field)));
    fields.push_back(Text(token::kColon));
    fields.push_back(GetDefaultAsString(field->getType()));
    fields.push_back(Text(token::kComma));
  }
  return Cat(Text(GetRecordName(decl)),
             Braces(arena_.New<Concat>(std::move(fields))));
}

RsExpr *Converter::AddByteReprTrait(const clang::RecordDecl *decl) {
  return Text("");
}

RsExpr *Converter::AddByteReprTrait(const clang::EnumDecl *decl) {
  return Text("");
}

RsExpr *Converter::ConvertUnsignedArithBinaryOperator(clang::BinaryOperator *op,
                                                      clang::Expr *expr) {
  auto opcode = op->getOpcode();
  const char *method = nullptr;
  switch (opcode) {
  case clang::BinaryOperator::Opcode::BO_Add:
  case clang::BinaryOperator::Opcode::BO_AddAssign:
    method = "wrapping_add";
    break;
  case clang::BinaryOperator::Opcode::BO_Sub:
  case clang::BinaryOperator::Opcode::BO_SubAssign:
    method = "wrapping_sub";
    break;
  case clang::BinaryOperator::Opcode::BO_Mul:
  case clang::BinaryOperator::Opcode::BO_MulAssign:
    method = "wrapping_mul";
    break;
  case clang::BinaryOperator::Opcode::BO_Div:
  case clang::BinaryOperator::Opcode::BO_DivAssign:
    method = "wrapping_div";
    break;
  case clang::BinaryOperator::Opcode::BO_Rem:
  case clang::BinaryOperator::Opcode::BO_RemAssign:
    method = "wrapping_rem";
    break;
  default:
    // FIXME: improve error handling
    llvm::errs() << "unsupported unsigned binary operator: " << opcode << '\n';
    op->dump();
    assert(0);
  }

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
  auto *operand = ConvertUnsignedArithOperand(expr, type);
  if (is_pointer_plus_integer_op) {
    operand = Cat(operand, Text("as usize"));
  }
  return Cat(Text(token::kDot), Text(method), Parens(operand));
}

RsExpr *Converter::ConvertAddrOf(clang::Expr *expr,
                                 clang::QualType pointer_type) {
  assert(pointer_type->isPointerType());
  if (auto ase =
          clang::dyn_cast<clang::ArraySubscriptExpr>(expr->IgnoreParens())) {
    auto base = ase->getBase();
    auto inner = base->IgnoreImplicit();
    if (base->IgnoreCasts()->getType()->isArrayType() &&
        IsFlexibleArrayMemberAccess(ctx_, inner)) {
      auto *node = EmitFlexibleArrayElementPtr(
          inner, ase->getIdx(),
          !pointer_type->getPointeeType().isConstQualified());
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return node;
    }
  }
  if (IsReferenceType(expr) || pointer_type->isFunctionPointerType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    return ConvertExpr(expr);
  }
  if (IsGlobalVar(expr)) {
    auto *node = ConvertExpr(expr);
    return Cat(Text("&raw"),
               Text(pointer_type->getPointeeType().isConstQualified()
                        ? keyword::kConst
                        : keyword_mut_),
               node, CastTo(pointer_type));
  }
  auto *node = ConvertExpr(expr);
  if (!pointer_type->getPointeeType().isConstQualified()) {
    return Cat(Text(token::kRef), Text(keyword_mut_), node,
               CastTo(pointer_type));
  }
  return Cat(Text(token::kRef), node, CastTo(pointer_type));
}


RsExpr *Converter::EmitDeref(RsExpr *inner, clang::QualType pointee_type) {
  auto wrap = std::exchange(autoref_mut_, std::nullopt);
  auto *node =
      Parens(Cat(Text(GetPointerDerefPrefix(pointee_type)), inner));
  if (wrap) {
    return Parens(Cat(Text(*wrap ? "&mut" : "&"), node));
  }
  return node;
}

RsExpr *Converter::ConvertDeref(clang::Expr *expr) {
  if (!isAddrOf()) {
    return EmitDeref(ConvertExpr(expr), expr->getType()->getPointeeType());
  }
  return ConvertExpr(expr);
}

RsExpr *Converter::ConvertArrow(clang::Expr *expr) {
  return ConvertDeref(expr);
}

Converter::TempMaterializationCtx
Converter::CollectRefBindingTempArgs(clang::CallExpr *expr) {
  TempMaterializationCtx ctx(expr->getNumArgs());
  if (auto *fn = expr->getCalleeDecl() ? expr->getCalleeDecl()->getAsFunction()
                                       : nullptr) {
    for (unsigned i = 0; i < expr->getNumArgs() && i < fn->getNumParams();
         ++i) {
      auto param_type = fn->getParamDecl(i)->getType();
      if (NeedsRefBindingTemp(expr->getArg(i), param_type)) {
        ctx.materialized_args[i] = param_type;
      }
    }
  }
  return ctx;
}

RsExpr *Converter::TempMaterializationCtx::GetOrMaterialize(
    unsigned argument_num,
    std::function<std::pair<RsExpr *, RsExpr *>(const std::string &,
                                                clang::QualType)>
        materialize_fn) {
  auto *&node = materialized_refs_.at(argument_num);
  if (node) {
    return node;
  }

  if (auto m = materialized_args.at(argument_num)) {
    auto [binding, ref] =
        materialize_fn(std::format("__tmp_{}", argument_num), *m);
    temporary_bindings.push_back(binding);
    node = ref;
    return node;
  }

  return nullptr;
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

RsExpr *Converter::ConvertPlaceholder(clang::Expr *expr, clang::Expr *arg,
                                      const PlaceholderCtx &ph_ctx) {
  if (arg->getType()->isFunctionPointerType()) {
    PushExprKind push(*this, ExprKind::Callee);
    return ConvertExpr(arg);
  }

  if (ph_ctx.declared_in_rule_as_rust_ptr && arg->getType()->isArrayType()) {
    auto *node = ConvertFreshPointer(arg);
    return Parens(Cat(node, Text(keyword::kAs), Text(ph_ctx.param_type)));
  }

  if (ph_ctx.needs_materialization()) {
    auto *materialized = ph_ctx.materialize_ctx->GetOrMaterialize(
        static_cast<unsigned>(ph_ctx.materialize_idx),
        [this, arg](const std::string &name, clang::QualType type) {
          return MaterializeTemp(name, type, arg);
        });
    if (materialized) {
      return materialized;
    }
  }

  if (ph_ctx.needs_pointer_receiver()) {
    auto *node = ConvertFreshObject(arg);
    return Parens(Cat(node, Text(keyword::kAs), Text(ph_ctx.param_type)));
  }

  if (ph_ctx.needs_object_receiver()) {
    PushExplicitAutoref autoref(
        *this,
        ph_ctx.is_index_base
            ? std::optional(ph_ctx.access == TranslationRule::Access::kWrite)
            : std::nullopt);
    PushExprKind push(*this, ExprKind::RValue);
    return ConvertDeref(arg);
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
    auto *node = ConvertLValue(arg);
    return Cat(Text("std::mem::take(&mut"), node, Text(')'));
  }

  return ConvertRValue(arg, ph_ctx.implicit_convert_to);
}

RsExpr *Converter::ConvertMappedMethodCall(
    clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
    clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx) {
  auto *receiver = ConvertIRFragment(mc.receiver, expr, args, num_args, ctx);
  auto *body = ConvertIRFragment(mc.body, expr, args, num_args, ctx);
  return Cat(receiver, body);
}

RsExpr *Converter::GetMappedAsNode(clang::Expr *expr, clang::Expr **args,
                                   unsigned num_args,
                                   TempMaterializationCtx *ctx) {
  auto *tgt_ir = Mapper::GetExprRule(GetCalleeOrExpr(expr));
  if (!tgt_ir)
    return nullptr;

  auto *node = ConvertIRFragment(tgt_ir->body, expr, args, num_args, ctx);
  if (tgt_ir->multi_statement) {
    return Braces(node);
  }
  return node;
}

RsExpr *Converter::ConvertIRFragment(
    const std::vector<TranslationRule::BodyFragment> &fragments,
    clang::Expr *expr, clang::Expr **args, unsigned num_args,
    TempMaterializationCtx *ctx) {
  using namespace TranslationRule;

  auto all_args = BuildUnifiedArgs(expr, args, num_args);

  std::vector<RsExpr *> parts;
  for (auto &frag : fragments) {
    if (auto *t = std::get_if<TextFragment>(&frag)) {
      parts.push_back(Text(t->text));
    } else if (auto *g = std::get_if<GenericFragment>(&frag)) {
      parts.push_back(
          Text(Mapper::InstantiateTemplate(GetCalleeOrExpr(expr), g->n)));
    } else if (auto *ph = std::get_if<PlaceholderFragment>(&frag)) {
      auto arg_idx = ph->n;
      assert(arg_idx < all_args.size());
      auto *arg = all_args[arg_idx];
      bool is_receiver = HasReceiver(expr) && arg_idx == 0;

      PlaceholderCtx ph_ctx{
          .param_type = Mapper::GetParamType(GetCalleeOrExpr(expr), arg_idx),
          .implicit_convert_to = GetParamImplicitConvertTarget(expr, arg_idx),
          .materialize_ctx = ctx,
          .materialize_idx =
              is_receiver ? -1 : ((int)arg_idx - HasReceiver(expr)),
          .access = ph->access,
          .is_receiver = is_receiver,
          .is_cpp_ptr = arg->getType()->isPointerType(),
          .maps_to_rust_ptr = Mapper::MapsToPointer(arg->getType()),
          .declared_in_rule_as_rust_ptr =
              Mapper::ParamIsPointer(GetCalleeOrExpr(expr), arg_idx),
          .is_index_base = ph->is_index_base,
      };
      parts.push_back(ConvertPlaceholder(expr, arg, ph_ctx));
    } else if (std::get_if<TranslationRule::VaArgsFragment>(&frag)) {
      parts.push_back(ConvertVariadicTail(expr, all_args));
    } else if (auto *mc =
                   std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
      parts.push_back(ConvertMappedMethodCall(expr, **mc, args, num_args, ctx));
    }
  }

  if (parts.size() == 1) {
    return parts.front();
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *
Converter::ConvertVariadicTail(clang::Expr *expr,
                               const std::vector<clang::Expr *> &all_args) {
  const auto *tgt_ir = Mapper::GetExprRule(GetCalleeOrExpr(expr));
  unsigned fixed = tgt_ir ? tgt_ir->params.size() : 0;

  std::vector<RsExpr *> parts;
  parts.push_back(Text("&["));
  for (unsigned i = fixed; i < all_args.size(); ++i) {
    parts.push_back(Parens(ConvertVariadicArg(all_args[i])));
    parts.push_back(Text(".into()"));
    parts.push_back(Text(token::kComma));
  }
  parts.push_back(Text("]"));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *Converter::AccessLValueObject(clang::MemberExpr *member) {
  auto *object = member->getBase();
  auto type = object->getType();
  if (member->isArrow()) {
    auto *op =
        clang::dyn_cast<clang::CXXOperatorCallExpr>(object->IgnoreImplicit());
    if (op && GetStrongestIteratorCategory(op->getArg(0)->getType()) ==
                  IteratorCategory::Bidirectional) {
      return ConvertExpr(object);
    }
  }
  if (type->isPointerType() ||
      (IsReferenceType(object) && clang::isa<clang::CallExpr>(object))) {
    return Cat(Text('('),
               Text(GetPointerDerefPrefix(type->getPointeeType())),
               ConvertExpr(object), Text(')'));
  }
  return ConvertExpr(object);
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

void Converter::SetValueFreshness(clang::QualType type) {
  if (TypeIsCopyable(type)) {
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

RsExpr *Converter::ConvertPointer(clang::Expr *expr, int line) {
  log() << "ConvertPointer called from line " << line << '\n';
  PushExprKind push(*this, ExprKind::AddrOf);
  return ConvertExpr(expr);
}

RsExpr *Converter::ConvertFreshPointer(clang::Expr *expr) {
  auto *node = ConvertPointer(expr);
  if (isFresh()) {
    return node;
  }
  SetFresh();
  return node;
}

RsExpr *Converter::ConvertFreshObject(clang::Expr *expr) {
  return ConvertFreshPointer(expr);
}

RsExpr *Converter::ConvertLValue(clang::Expr *expr) {
  PushExprKind push(*this, ExprKind::LValue);
  return ConvertExpr(expr);
}

RsExpr *
Converter::ConvertRValue(clang::Expr *expr,
                         std::optional<clang::QualType> implicit_convert_to,
                         int line) {
  log() << "ConvertRValue called from line " << line << '\n';
  PushExprKind push(*this, ExprKind::RValue);
  return ConvertExpr(expr, implicit_convert_to);
}

RsExpr *Converter::ConvertFreshRValue(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  auto *node = ConvertRValue(expr, implicit_convert_to);
  if (!isFresh() && !expr->getType()->isVoidType() &&
      !expr->getType()->isPointerType()) {
    SetFresh();
    return Cat(Text('('), node, Text(").clone()"));
  }
  SetFresh();
  return node;
}

std::pair<RsExpr *, RsExpr *>
Converter::MaterializeTemp(const std::string &binding_name,
                           clang::QualType param_type, clang::Expr *expr) {
  auto *value = ConvertRValue(expr, param_type.getNonReferenceType());
  return {Cat(Text(std::format("let mut {} =", binding_name)), value,
              Text(token::kSemiColon)),
          Text(std::format("& mut {}", binding_name))};
}

void Converter::dump_expr_kinds() {
  log() << "isRValue: " << isRValue() << ", isXValue: " << isXValue()
        << ", isAddrOf: " << isAddrOf() << ", isObject: " << isObject()
        << ", isVoid: " << isVoid() << '\n';
}

RsExpr *Converter::emplace_back_plugin_construct_arg(
    clang::QualType elem_type, clang::CXXConstructExpr *ctor) {
  return ConvertVarInit(elem_type, ctor);
}

const char *Converter::GetPointerDerefPrefix(clang::QualType pointee_type) {
  return token::kStar;
}

} // namespace cpp2rust
