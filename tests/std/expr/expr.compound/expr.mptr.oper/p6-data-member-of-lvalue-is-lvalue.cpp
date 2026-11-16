// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6: the result of .* with a pointer to data member is
// an lvalue if the first operand is an lvalue.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  assert(&(obj.*pm) == &obj.second);
  int &ref = obj.*pm;
  ref = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
  return 0;
}
