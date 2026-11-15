// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.2: the exemption requires the lvalue-to-rvalue
// conversion. Taking the address of the constant n odr-uses it, so [=] captures
// a copy, and the address seen inside the lambda is not that of n.
#include <cassert>

int main() {
  const int n = 5;
  auto f = [=] { return &n; };
  assert(f() != &n);
  assert(*f() == 5);
  return 0;
}
