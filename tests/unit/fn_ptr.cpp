// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <assert.h>
#include <compare>
#include <cstddef>
#include <signal.h>
#include <stddef.h>
#include <string>
#include <utility>
#include <vector>

typedef int (*foo_t)(void *);

int my_foo(void *p) { return *static_cast<int *>(p); }

int foo(foo_t fn, int *pi) { return fn(pi); }

unsigned long twice(unsigned long x) { return x * 2; }

unsigned long twice_in_place(unsigned long &x) {
  x *= 2;
  return x;
}

template <class T> static size_t ret_size(T v) { return v + 1; }

template <class T> static size_t call_fn(size_t (*f)(T), T v) {
  return f(v) * 2;
}

template <class T> static std::size_t identity_hash(T v) { return v; }

template <class H> struct HashHolder {
  H h;
  HashHolder(const H &h) : h(h) {}
};

static int total = 0;

void add_to_total(int x) { total += x; }

int pair_sum(std::pair<int, int> p) { return p.first + p.second; }

size_t string_size(std::string s) { return s.size(); }

int vector_sum(std::vector<int> v) { return v[0] + v[1]; }

bool is_less(std::strong_ordering o) { return o < 0; }

int main() {
  foo_t fn = nullptr;
  assert(fn == nullptr);
  assert(fn != my_foo);

  fn = my_foo;
  assert(fn != nullptr);
  assert(fn == my_foo);

  int a = 10;
  assert(foo(fn, &a) == a);

  unsigned long (*ul_fn)(unsigned long) = &twice;
  size_t n = 21;
  size_t r = ul_fn(n);
  assert(r == 42);

  unsigned long (*ul_ref_fn)(unsigned long &) = &twice_in_place;
  size_t m = 21;
  size_t q = ul_ref_fn(m);
  assert(q == 42);
  assert(m == 42);

  assert(call_fn(ret_size<int>, 3) == 8);

  HashHolder<std::size_t (*)(bool)> hh(identity_hash<bool>);
  assert(hh.h(true) == 1);

  sig_t handler = add_to_total;
  handler(2);
  handler(3);
  assert(total == 5);

  int (*pf)(std::pair<int, int>) = pair_sum;
  assert(pf({1, 2}) == 3);

  size_t (*sf)(std::string) = string_size;
  assert(sf(std::string("abc")) == 3);

  int (*vf)(std::vector<int>) = vector_sum;
  assert(vf({1, 2}) == 3);

  bool (*of)(std::strong_ordering) = is_less;
  assert(of(1 <=> 2));
  assert(!of(2 <=> 1));

  std::strong_ordering o = 1 <=> 2;
  std::strong_ordering *op = &o;
  assert(*op == std::strong_ordering::less);
  return 0;
}
