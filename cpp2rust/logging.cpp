// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "logging.h"

namespace cpp2rust {
namespace {
llvm::raw_ostream *log_ = &llvm::nulls();
} // namespace

void SetVerbose(bool verbose) {
  log_ = verbose ? &llvm::errs() : &llvm::nulls();
}

llvm::raw_ostream &log() { return *log_; }

} // namespace cpp2rust
