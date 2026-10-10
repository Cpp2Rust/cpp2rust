// panic: refcount
#include <assert.h>
#include <stdint.h>

#define UNCONST(p) ((void *)(uintptr_t)(const void *)(p))

void set(int &r) { r = 7; }

template <typename T> void set_through(const T &v) { set(const_cast<T &>(v)); }

int main() {
  const int a = 1;
  const int *p = &a;
  auto q = static_cast<int *>(UNCONST(p));
  assert(p == q);

  int v = 1;
  const int &cr = v;
  set(const_cast<int &>(cr));
  assert(v == 7);
  v = 0;
  set_through(v);
  assert(v == 7);
  return 0;
}
