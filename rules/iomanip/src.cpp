// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <iomanip>
#endif

auto CPP2RUST_EXPR_RULE(1)(int n) { return std::setw(n); }
