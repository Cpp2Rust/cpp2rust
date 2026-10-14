// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "ast_consumer.h"

#include <llvm/Support/ErrorHandling.h>

#include "converter/converter.h"
#include "converter/rules/registry.h"

namespace cpp2rust {
void ASTConsumer::HandleTranslationUnit(clang::ASTContext &ctx) {
  if (CI_.getDiagnostics().hasErrorOccurred()) {
    llvm::report_fatal_error(
        "the translation unit and its loaded rules do not compile");
  }
  RuleRegistry::Load(CI_.getSema(), model_);
  auto converter = CreateConverter(rs_code_, ctx, model_, rules_dir_);
  converter->SetSema(CI_.getSema());
  if (first_) {
    converter->EmitFilePreamble();
  }
  converter->TraverseDecl(ctx.getTranslationUnitDecl());
}
} // namespace cpp2rust
