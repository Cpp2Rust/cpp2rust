// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#if defined(__linux__)
#include <byteswap.h>
#elif !defined(__APPLE__)
#error "Unsupported platform"
#endif

long f1(long exp, long c) { return __builtin_expect(exp, c); }
int f2(unsigned int x) { return __builtin_ctz(x); }
int f3(unsigned int x) { return __builtin_clz(x); }
#if defined(__linux__)
unsigned short f4(unsigned short x) { return bswap_16(x); }
unsigned int f5(unsigned int x) { return bswap_32(x); }
unsigned long long f6(unsigned long long x) { return bswap_64(x); }
#elif defined(__APPLE__)
unsigned short f4(unsigned short x) { return __builtin_bswap16(x); }
unsigned int f5(unsigned int x) { return __builtin_bswap32(x); }
unsigned long long f6(unsigned long long x) { return __builtin_bswap64(x); }
#endif
int f7(unsigned long x) { return __builtin_ctzl(x); }
int f8(unsigned long x) { return __builtin_popcountl(x); }
bool f9(long a, long b, long *r) { return __builtin_mul_overflow(a, b, r); }
bool f10(long long a, long long b, long long *r) { return __builtin_mul_overflow(a, b, r); }
#if defined(__x86_64__) || defined(__i386__)
void f11(void) { return __builtin_ia32_pause(); }
#endif
