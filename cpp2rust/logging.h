// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <llvm/Support/raw_ostream.h>

namespace cpp2rust {

void SetVerbose(bool verbose);

llvm::raw_ostream &log();

} // namespace cpp2rust
