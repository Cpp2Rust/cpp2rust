// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef int (*foo_t)(void *);

int my_foo(void *p) { return *static_cast<int *>(p); }

int foo(foo_t fn, int *pi) { return fn(pi); }

int main() {
  foo_t fn = nullptr;
  assert(fn == nullptr);
  assert(fn != my_foo);

  fn = my_foo;
  assert(fn != nullptr);
  assert(fn == my_foo);

  int a = 10;
  assert(foo(fn, &a) == a);
  return 0;
}
