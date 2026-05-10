#include <cassert>
#include <cstddef>

typedef int (*FnPtr)(int);

struct Inner {
  int v;
  const char *name;
};

struct Outer {
  int *p1;
  const int *p2;
  int *arr[3];
  const char *cp;
  int **pp;
  Inner inner;
  int x;
  FnPtr fn;
};

struct Foo {
  const char *s1;
  const char *s2;
  FnPtr fn1;
  FnPtr fn2;
  int n;
};

static FnPtr static_fn;
static Outer static_outer;
static Inner static_inner_array[2];

static Foo static_foo = {"hello", 0, 0, 0, 42};

static Foo static_foo_array[2] = {
    {"first", 0, 0, 0, 1},
    {"second", 0, 0, 0, 2},
};

void check_local_static() {
  static Outer local_outer;
  static FnPtr local_fn;
  static int *local_p;
  assert(local_outer.p1 == nullptr);
  assert(local_outer.fn == nullptr);
  assert(local_fn == nullptr);
  assert(local_p == nullptr);
}

int main() {
  assert(static_fn == nullptr);

  assert(static_outer.p1 == nullptr);
  assert(static_outer.p2 == nullptr);
  assert(static_outer.cp == nullptr);
  assert(static_outer.pp == nullptr);
  assert(static_outer.fn == nullptr);
  for (int i = 0; i < 3; ++i) {
    assert(static_outer.arr[i] == nullptr);
  }
  assert(static_outer.inner.name == nullptr);

  for (int i = 0; i < 2; ++i) {
    assert(static_inner_array[i].name == nullptr);
  }

  assert(static_foo.s2 == nullptr);
  assert(static_foo.fn1 == nullptr);
  assert(static_foo.fn2 == nullptr);
  assert(static_foo.n == 42);

  for (int i = 0; i < 2; ++i) {
    assert(static_foo_array[i].s2 == nullptr);
    assert(static_foo_array[i].fn1 == nullptr);
    assert(static_foo_array[i].fn2 == nullptr);
  }

  check_local_static();

  return 0;
}
