// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#define BUILTIN_TYPE(T, N0, N1, N2, N3, N4)                                    \
  typedef T N0;                                                                \
  typedef T *N1;                                                               \
  typedef const T *N2;                                                         \
  typedef volatile T *N3;                                                      \
  typedef const volatile T *N4;

BUILTIN_TYPE(char, t20, t21, t22, t23, t24)
BUILTIN_TYPE(signed char, t30, t31, t32, t33, t34)
BUILTIN_TYPE(unsigned char, t40, t41, t42, t43, t44)
BUILTIN_TYPE(short, t60, t61, t62, t63, t64)
BUILTIN_TYPE(unsigned short, t70, t71, t72, t73, t74)
BUILTIN_TYPE(int, t90, t91, t92, t93, t94)
BUILTIN_TYPE(unsigned int, t100, t101, t102, t103, t104)
BUILTIN_TYPE(float, t130, t131, t132, t133, t134)
BUILTIN_TYPE(long, t140, t141, t142, t143, t144)
BUILTIN_TYPE(unsigned long, t150, t151, t152, t153, t154)
BUILTIN_TYPE(long long, t160, t161, t162, t163, t164)
BUILTIN_TYPE(unsigned long long, t170, t171, t172, t173, t174)
BUILTIN_TYPE(double, t180, t181, t182, t183, t184)
BUILTIN_TYPE(long double, t190, t191, t192, t193, t194)
BUILTIN_TYPE(__int128, t200, t201, t202, t203, t204)
BUILTIN_TYPE(unsigned __int128, t210, t211, t212, t213, t214)
BUILTIN_TYPE(void, t220, t221, t222, t223, t224)

#undef BUILTIN_TYPE
