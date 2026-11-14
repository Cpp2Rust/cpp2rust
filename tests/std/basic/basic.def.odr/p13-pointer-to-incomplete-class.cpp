// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/13, Example 4: X is never defined. Forming, copying and
// comparing pointers to X does not require X to be complete.
#include <cassert>

struct X;
struct X *x1;
X *x2;

X *identity(X *p) { return p; }

int main() {
  assert(x1 == nullptr);
  assert(identity(x2) == x1);
  X **pp = &x1;
  assert(*pp == nullptr);
  return 0;
}
