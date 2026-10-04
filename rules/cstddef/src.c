// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <stddef.h>
#include <sys/types.h>
#endif

typedef size_t CPP2RUST_TYPE_RULE(2);
typedef size_t *CPP2RUST_TYPE_RULE(3);
typedef const size_t *CPP2RUST_TYPE_RULE(4);
typedef __typeof__(sizeof(0)) CPP2RUST_TYPE_RULE(5);
typedef ssize_t CPP2RUST_TYPE_RULE(6);
typedef ssize_t *CPP2RUST_TYPE_RULE(7);
typedef const ssize_t *CPP2RUST_TYPE_RULE(8);
