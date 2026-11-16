// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6: if the result of .* or ->* is a function, it can
// be used only as the operand of the function call operator ().
#include <cassert>

struct AccumulatesCalls {
  int total = 0;
  int add(int amount) {
    total += amount;
    return total;
  }
};

int main() {
  int (AccumulatesCalls::*pmf)(int) = &AccumulatesCalls::add;
  AccumulatesCalls first;
  AccumulatesCalls second;
  assert((first.*pmf)(3) == 3);
  assert(first.total == 3);
  assert(second.total == 0);
  AccumulatesCalls *p = &second;
  assert((p->*pmf)(4) == 4);
  assert(first.total == 3);
  assert(second.total == 4);
  return 0;
}
