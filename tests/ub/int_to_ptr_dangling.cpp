// panic-ub: refcount
// nondet-result: unsafe
#include <cstdint>

int main() {
  int *p = new int[2]{1, 2};
  uintptr_t i = (uintptr_t)p;
  delete[] p;
  return ((int *)i)[1];
}
