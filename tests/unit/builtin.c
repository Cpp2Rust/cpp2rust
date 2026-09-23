#include <assert.h>
#include <limits.h>
#if defined(__linux__)
#include <byteswap.h>
#endif

static void test_expect(void) {
  int x = 42;
  assert(__builtin_expect(x == 42, 1));
}

static void test_ctz(void) {
  assert(__builtin_ctz(8U) == 3);
  assert(__builtin_ctz(1U) == 0);
}

static void test_clz(void) {
  assert(__builtin_clz(1U) == 31);
  assert(__builtin_clz(0x80000000U) == 0);
}

static void test_bswap16(void) {
#if defined(__linux__)
  assert(bswap_16((unsigned short)0x1234) == 0x3412);
#else
  assert(__builtin_bswap16((unsigned short)0x1234) == 0x3412);
#endif
}

static void test_bswap32(void) {
#if defined(__linux__)
  assert(bswap_32(0x12345678U) == 0x78563412U);
#else
  assert(__builtin_bswap32(0x12345678U) == 0x78563412U);
#endif
}

static void test_bswap64(void) {
#if defined(__linux__)
  assert(bswap_64(0x0123456789ABCDEFULL) == 0xEFCDAB8967452301ULL);
#else
  assert(__builtin_bswap64(0x0123456789ABCDEFULL) == 0xEFCDAB8967452301ULL);
#endif
}

static void test_ctzl(void) {
  assert(__builtin_ctzl(8UL) == 3);
  assert(__builtin_ctzl(1UL) == 0);
}

static void test_popcountl(void) {
  assert(__builtin_popcountl(0UL) == 0);
  assert(__builtin_popcountl(0xFFUL) == 8);
}

static void test_clzl(void) {
  assert(__builtin_clzl(1UL) == 63);
  assert(__builtin_clzl(0x8000000000000000UL) == 0);
}

static void test_inff(void) {
  float inf = __builtin_inff();
  assert(inf > 0.0f);
  assert(inf == inf * 2.0f);
  assert(1.0f / inf == 0.0f);
}

static void test_nanf(void) {
  float nan = __builtin_nanf("");
  assert(nan != nan);
  assert(!(nan < 0.0f));
  assert(!(nan > 0.0f));
}

static void test_mul_overflow_long(void) {
  long r = 0;
  assert(!__builtin_mul_overflow(3L, 7L, &r));
  assert(r == 21);
  assert(__builtin_mul_overflow(LONG_MAX, 2L, &r));
}

static void test_mul_overflow_long_long(void) {
  long long r = 0;
  assert(!__builtin_mul_overflow(1000LL, 1000LL, &r));
  assert(r == 1000000);
  assert(__builtin_mul_overflow(LLONG_MAX, 2LL, &r));
}

int main(void) {
  test_expect();
  test_ctz();
  test_clz();
  test_bswap16();
  test_bswap32();
  test_bswap64();
  test_ctzl();
  test_popcountl();
  test_clzl();
  test_inff();
  test_nanf();
  test_mul_overflow_long();
  test_mul_overflow_long_long();
  return 0;
}
