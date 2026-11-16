// panic-ub: refcount
// nondet-result: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6: if the second operand is the null member pointer
// value, the behavior is undefined.

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*pm = nullptr;
  HasTwoInts obj{3, 4};
  return obj.*pm;
}
