// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#define _GNU_SOURCE
#include <pwd.h>

typedef struct passwd t1;

struct passwd *f1(uid_t uid) { return getpwuid(uid); }

int f2(uid_t uid, struct passwd *pwd, char *buf, size_t buflen,
       struct passwd **result) {
  return getpwuid_r(uid, pwd, buf, buflen, result);
}
