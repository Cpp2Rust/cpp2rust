// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.fct.default]/5: the names in the default argument are looked up
// at the point where the default argument appears.
#include <cassert>

static int scaled_calls;

int rate = 1;

int scaled(int amount) {
  ++scaled_calls;
  return amount * 10;
}

int total(int base = scaled(rate)) { return base; }

int main() {
  rate = 2;
  {
    int rate = 3;
    assert(total() == 20);
    assert(scaled_calls == 1);
    assert(rate == 3);
  }
  assert(total(5) == 5);
  assert(scaled_calls == 1);
  return 0;
}
