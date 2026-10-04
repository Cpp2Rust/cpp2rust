// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <dirent.h>
#endif

typedef DIR *CPP2RUST_TYPE_RULE(1);
typedef struct dirent CPP2RUST_TYPE_RULE(2);

DIR *CPP2RUST_EXPR_RULE(1)(const char *name) { return opendir(name); }

struct dirent *CPP2RUST_EXPR_RULE(2)(DIR *dirp) {
  return readdir(dirp);
}

int CPP2RUST_EXPR_RULE(3)(DIR *dirp) { return closedir(dirp); }
