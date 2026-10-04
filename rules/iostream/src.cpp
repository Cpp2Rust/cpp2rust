// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <iostream>
#endif

using CPP2RUST_TYPE_RULE(1) = std::ostream;
using CPP2RUST_TYPE_RULE(2) = std::ostream &;
using CPP2RUST_TYPE_RULE(3) = std::ostream *;

std::ostream &CPP2RUST_EXPR_RULE(1)() { return std::cout; }

std::ostream &CPP2RUST_EXPR_RULE(2)() { return std::cerr; }

std::ostream *CPP2RUST_EXPR_RULE(3)() { return &std::cout; }

std::ostream *CPP2RUST_EXPR_RULE(4)() { return &std::cerr; }
