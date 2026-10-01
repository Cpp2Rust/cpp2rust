#include <assert.h>
#include <cstddef>

typedef int (*foo_t)(void *);

int my_foo(void *p) { return *static_cast<int *>(p); }

int foo(foo_t fn, int *pi) { return fn(pi); }

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

  assert(call_fn(ret_size<int>, 3) == 8);

  HashHolder<std::size_t (*)(bool)> hh(identity_hash<bool>);
  assert(hh.h(true) == 1);
  return 0;
}
