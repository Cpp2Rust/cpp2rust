// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <pwd.h>
#endif

typedef struct passwd CPP2RUST_TYPE_RULE(1);

struct passwd *CPP2RUST_EXPR_RULE(1)(uid_t uid) {
  return getpwuid(uid);
}

int CPP2RUST_EXPR_RULE(2)(uid_t uid, struct passwd *pwd, char *buf,
                          size_t buflen, struct passwd **result) {
  return getpwuid_r(uid, pwd, buf, buflen, result);
}
