#include <assert.h>
#include <cstddef>
#include <stddef.h>

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
  return 0;
}
