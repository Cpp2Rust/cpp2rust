// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <pwd.h>

struct passwd *f1(uid_t uid) { return getpwuid(uid); }
