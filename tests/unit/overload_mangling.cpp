#include <cassert>
#include <utility>

static void inc(int *p) { *p += 1; }
static void add(int *p, int n) { *p += n; }
static int twice(int n) { return n * 2; }

template <typename T> struct Access {
  int get(T *p) { return p->base; }
  int get(const T *p) { return p->base + 1; }
  int ref(T &r) { return r.base + 2; }
  int ref(const T &r) { return r.base + 3; }
};

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
  int combine(const std::pair<int, long> &p, int (*f)(int), const int *q,
              unsigned long n) const {
    return base + (int)p.second + f(*q) + (int)n;
  }
  int combine(const std::pair<int, int> &p, void (*f)(int *, int), int *q,
              unsigned long n) const {
    f(q, (int)n);
    return base + p.first + *q;
  }
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
  const int c = 3;
  assert(s.combine(std::pair<int, long>(1, 2L), twice, &c, 4UL) == 112);
  int z = 1;
  assert(s.combine(std::pair<int, int>(1, 2), add, &z, 5UL) == 107);
  assert(z == 6);
  Access<S> a;
  const S *cs = &s;
  const S &cr = s;
  assert(a.get(&s) == 100);
  assert(a.get(cs) == 101);
  assert(a.ref(s) == 102);
  assert(a.ref(cr) == 103);
  Box b{4};
  assert(b.v == 4);
  return 0;
}
