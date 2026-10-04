// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#define BUILTIN_TYPE(N, T)                                                     \
  typedef T CPP2RUST_TYPE_RULE(N##0);                                          \
  typedef T *CPP2RUST_TYPE_RULE(N##1);                                         \
  typedef const T *CPP2RUST_TYPE_RULE(N##2);                                   \
  typedef volatile T *CPP2RUST_TYPE_RULE(N##3);                                \
  typedef const volatile T *CPP2RUST_TYPE_RULE(N##4);

BUILTIN_TYPE(2, char)
BUILTIN_TYPE(3, signed char)
BUILTIN_TYPE(4, unsigned char)
BUILTIN_TYPE(6, short)
BUILTIN_TYPE(7, unsigned short)
BUILTIN_TYPE(9, int)
BUILTIN_TYPE(10, unsigned int)
BUILTIN_TYPE(13, float)
BUILTIN_TYPE(14, long)
BUILTIN_TYPE(15, unsigned long)
BUILTIN_TYPE(16, long long)
BUILTIN_TYPE(17, unsigned long long)
BUILTIN_TYPE(18, double)
BUILTIN_TYPE(19, long double)
BUILTIN_TYPE(20, __int128)
BUILTIN_TYPE(21, unsigned __int128)
BUILTIN_TYPE(22, void)

#undef BUILTIN_TYPE
