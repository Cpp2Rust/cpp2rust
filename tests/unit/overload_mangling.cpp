#include <cassert>
#include <utility>

static void inc(int *p) { *p += 1; }
static void add(int *p, int n) { *p += n; }
static int twice(int n) { return n * 2; }

struct S {
  int base;
  template <typename T> int width(int x) const {
    return base + x * (int)sizeof(T);
  }
  template <int N> int scale(int x) const { return base + x * N; }
  template <typename... Ts> int count(int x) const {
    return base + x + (int)sizeof...(Ts);
  }
  int plain(int x) const { return base + x; }
  int plain(long x) const { return base + (int)x + 1; }
  int take(int &x) const { return base + x + 1; }
  int take(int &&x) const { return base + x + 2; }
  int pick(std::pair<int, int> p) const { return base + p.first; }
  int pick(std::pair<int, long> p) const { return base + (int)p.second; }
  int apply(void (*f)(int *), int x) const {
    f(&x);
    return base + x;
  }
  int apply(void (*f)(int *, int), int x) const {
    f(&x, 10);
    return base + x;
  }
  int apply(int (*f)(int), int x) const { return base + f(x); }
};

struct Box {
  int v;
};

int main() {
  S s{100};
  assert(s.width<char>(3) == 103);
  assert(s.width<int>(3) == 112);
  assert(s.scale<2>(5) == 110);
  assert(s.scale<3>(5) == 115);
  assert(s.count<>(1) == 101);
  assert((s.count<int, long>(1) == 103));
  assert(s.plain(1) == 101);
  assert(s.plain(1L) == 102);
  int y = 1;
  assert(s.take(y) == 102);
  assert(s.take(5) == 107);
  assert(s.pick(std::pair<int, int>(1, 2)) == 101);
  assert(s.pick(std::pair<int, long>(1, 2L)) == 102);
  assert(s.apply(inc, 1) == 102);
  assert(s.apply(add, 1) == 111);
  assert(s.apply(twice, 3) == 106);
  Box b{4};
  assert(b.v == 4);
  return 0;
}
