// panic: refcount
#include <assert.h>
#include <stdint.h>

#define UNCONST(p) ((void *)(uintptr_t)(const void *)(p))

int main() {
  const int a = 1;
  const int *p = &a;
  auto q = static_cast<int *>(UNCONST(p));
  assert(p == q);
  return 0;
}
