#include <cassert>

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
  Box b{4};
  assert(b.v == 4);
  return 0;
}
