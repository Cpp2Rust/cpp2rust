// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef __linux__
#include <sys/types.h>
#include <sys/xattr.h>

int f1(int fd, const char *name, const void *value, size_t size, int flags) {
  return fsetxattr(fd, name, value, size, flags);
}
#endif
