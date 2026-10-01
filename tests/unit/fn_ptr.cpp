#include <assert.h>
#include <stddef.h>

typedef int (*foo_t)(void *);

int my_foo(void *p) { return *static_cast<int *>(p); }

int foo(foo_t fn, int *pi) { return fn(pi); }

unsigned long twice(unsigned long x) { return x * 2; }

unsigned long twice_in_place(unsigned long &x) {
  x *= 2;
  return x;
}

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
  return 0;
}
