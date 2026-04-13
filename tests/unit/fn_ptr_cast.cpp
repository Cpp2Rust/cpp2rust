#include <assert.h>

typedef void (*generic_fn)(void);
typedef int (*int_fn)(int);

int double_it(int x) { return x * 2; }

void test_roundtrip() {
  int_fn fn = double_it;
  assert(fn(5) == 10);

  generic_fn gfn = (generic_fn)fn;
  assert(gfn != nullptr);

  int_fn fn2 = (int_fn)gfn;
  assert(fn2(5) == 10);
  assert(fn2 == fn);
}

void test_double_cast() {
  int_fn fn = double_it;
  int_fn fn2 = (int_fn)(generic_fn)fn;
  assert(fn2(5) == 10);
  assert(fn2 == fn);
}

struct Command {
  void *data;
};

void test_void_ptr_to_fn() {
  Command cmd;
  cmd.data = (void *)double_it;

  int_fn fn = (int_fn)cmd.data;
  assert(fn(5) == 10);
}

int main() {
  test_roundtrip();
  test_double_cast();
  test_void_ptr_to_fn();
  return 0;
}
