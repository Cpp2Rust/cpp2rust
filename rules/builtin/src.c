// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <stddef.h>

#if defined(__linux__)
#include <byteswap.h>
#elif !defined(__APPLE__)
#error "Unsupported platform"
#endif
#endif

long CPP2RUST_EXPR_RULE(1)(long exp, long c) {
  return __builtin_expect(exp, c);
}
int CPP2RUST_EXPR_RULE(2)(unsigned int x) { return __builtin_ctz(x); }
int CPP2RUST_EXPR_RULE(3)(unsigned int x) { return __builtin_clz(x); }
#if defined(__linux__)
unsigned short CPP2RUST_EXPR_RULE(4)(unsigned short x) { return bswap_16(x); }
unsigned int CPP2RUST_EXPR_RULE(5)(unsigned int x) { return bswap_32(x); }
unsigned long long CPP2RUST_EXPR_RULE(6)(unsigned long long x) {
  return bswap_64(x);
}
#elif defined(__APPLE__)
unsigned short CPP2RUST_EXPR_RULE(4)(unsigned short x) {
  return __builtin_bswap16(x);
}
unsigned int CPP2RUST_EXPR_RULE(5)(unsigned int x) {
  return __builtin_bswap32(x);
}
unsigned long long CPP2RUST_EXPR_RULE(6)(unsigned long long x) {
  return __builtin_bswap64(x);
}
#endif
int CPP2RUST_EXPR_RULE(7)(unsigned long x) { return __builtin_ctzl(x); }
int CPP2RUST_EXPR_RULE(8)(unsigned long x) { return __builtin_popcountl(x); }
int CPP2RUST_EXPR_RULE(12)(long a, long b, long *r) {
  return __builtin_mul_overflow(a, b, r);
}
int CPP2RUST_EXPR_RULE(13)(long long a, long long b, long long *r) {
  return __builtin_mul_overflow(a, b, r);
}
#if defined(__x86_64__) || defined(__i386__)
void CPP2RUST_EXPR_RULE(11)(void) { return __builtin_ia32_pause(); }
#endif

void *CPP2RUST_EXPR_RULE(14)(void *dst, const void *src, size_t n) {
  return __builtin_memcpy(dst, src, n);
}

void CPP2RUST_EXPR_RULE(15)() { return __builtin_abort(); }

int CPP2RUST_EXPR_RULE(16)(unsigned long x) { return __builtin_clzl(x); }

float CPP2RUST_EXPR_RULE(17)() { return __builtin_inff(); }

float CPP2RUST_EXPR_RULE(18)(const char *tagp) { return __builtin_nanf(tagp); }
