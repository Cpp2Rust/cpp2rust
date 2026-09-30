// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/factory.h"

#include "converter/models/converter_refcount.h"
#include "converter/rules/registry.h"

namespace cpp2rust {

std::unique_ptr<Converter> CreateConverter(std::string &rs_code,
                                           clang::ASTContext &ctx, Model model,
                                           const std::string &rules_dir) {
  RuleRegistry::Load(model, rules_dir);
  switch (model) {
  case Model::kUnsafe:
    return std::make_unique<Converter>(rs_code, ctx);
  case Model::kRefCount:
    return std::make_unique<ConverterRefCount>(rs_code, ctx);
  }
  std::unreachable();
}

} // namespace cpp2rust
