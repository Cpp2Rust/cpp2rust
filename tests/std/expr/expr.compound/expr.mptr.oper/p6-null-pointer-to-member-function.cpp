// panic-ub: refcount
// nondet-result: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6: if the second operand is the null member pointer
// value, the behavior is undefined.

struct HasMemberFunction {
  int value;
  int read() { return value; }
};

int main() {
  int (HasMemberFunction::*pmf)() = nullptr;
  HasMemberFunction obj{3};
  return (obj.*pmf)();
}
