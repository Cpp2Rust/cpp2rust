// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>

#define BUILTIN_TYPE(N, T)                                                     \
  typedef T t##N##0;                                                           \
  typedef T *t##N##1;                                                          \
  typedef const T *t##N##2;                                                    \
  typedef volatile T *t##N##3;                                                 \
  typedef const volatile T *t##N##4;

typedef decltype(nullptr) t0;

BUILTIN_TYPE(1, bool)
BUILTIN_TYPE(5, char8_t)
BUILTIN_TYPE(8, char16_t)
BUILTIN_TYPE(11, wchar_t)
BUILTIN_TYPE(12, char32_t)

#undef BUILTIN_TYPE
