#include <algorithm>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <sys/types.h>

static size_t add_sizes(size_t a, size_t b) { return a + b; }

static unsigned long take_ulong(unsigned long x) { return x; }

static ssize_t sub_signed(ssize_t a, ssize_t b) { return a - b; }

int main() {
  size_t n = sizeof(int) + 4;
  assert(n == sizeof(int) + 4);

  unsigned long ul = 10;
  size_t sz = 20;
  size_t mixed = sz + ul;
  assert(mixed == 30);

  assert(sz > ul);
  assert(ul < sz);
  assert(!(sz == ul));

  size_t chain = sz + ul + 5u + sizeof(long);
  assert(chain == 20 + 10 + 5 + sizeof(long));

  size_t acc = 100;
  acc += sizeof(double);
  acc *= 2;
  acc -= ul;
  assert(acc == (100 + sizeof(double)) * 2 - 10);

  sz = sz + 1;
  assert(sz == 21);

  size_t fr = add_sizes(sizeof(int) + sz, ul);
  assert(fr == sizeof(int) + 21 + 10);

  unsigned long fr2 = take_ulong(sz);
  assert(fr2 == 21);

  size_t lo = std::min(sz, sizeof(long) + ul);
  size_t hi = std::max(sizeof(int) + sz, ul);
  assert(lo == sizeof(long) + 10);
  assert(hi == sizeof(int) + 21);
  size_t bound = std::min(sz, (size_t)4);
  assert(bound == 4);

  int data[8];
  size_t count = sizeof(data) / sizeof(data[0]);
  for (size_t i = 0; i < count; i++) {
    data[i] = (int)(i * 2);
  }
  size_t total = 0;
  for (size_t i = 0; i < count; i++) {
    total += (size_t)data[i];
  }
  assert(total == 56);

  size_t cond = (sz > ul) ? sz + sizeof(int) : ul;
  assert(cond == 21 + sizeof(int));

  size_t arr[4] = {0, 1, 2, 3};
  size_t idx = (sizeof(int) > 2) ? 2 : 0;
  assert(arr[idx] == 2);

  ssize_t s1 = 5;
  ssize_t s2 = 12;
  ssize_t sd = sub_signed(s1, s2);
  assert(sd == -7);
  assert(sd < 0);

  long l = 3;
  ssize_t sm = s2 + l;
  assert(sm == 15);
  assert(sm > l);

  ssize_t smin = std::min(sd, sm);
  ssize_t smax = std::max(sd, sm);
  assert(smin == -7);
  assert(smax == 15);

  ssize_t delta = (ssize_t)sz - (ssize_t)ul;
  assert(delta == 11);

  typedef long long ll_t;
  ll_t a64 = 100;
  ssize_t b = 30;
  a64 -= b;
  assert(a64 == 70);
  a64 += b;
  assert(a64 == 100);
  ssize_t c = -20;
  a64 -= c;
  assert(a64 == 120);

  return (int)(n % 7);
}
