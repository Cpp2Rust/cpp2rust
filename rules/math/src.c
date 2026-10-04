// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <math.h>
#endif

double CPP2RUST_EXPR_RULE(1)(double x) { return cos(x); }

double CPP2RUST_EXPR_RULE(2)(double x) { return round(x); }

double CPP2RUST_EXPR_RULE(3)(double x) { return sin(x); }
