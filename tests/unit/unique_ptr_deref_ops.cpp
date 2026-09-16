// Tests compound assignment through deref and multiple derefs in one
// expression.
#include <cassert>
#include <memory>

int main() {
  auto p = std::make_unique<int>(10);
  *p += 5;
  *p -= 3;
  *p *= 2;
  auto q = std::make_unique<int>(1);
  int sum = *p + *q;
  assert(sum == 25);
  return 0;
}
