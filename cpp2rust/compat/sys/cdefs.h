// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <sys/cdefs.h>

// glibc >= 2.43 wraps strchr, memchr, strstr, etc. in a _Generic selection
// to preserve the constness of the argument in C23. Disable it so we
// translate the plain function call.
#undef __glibc_const_generic
