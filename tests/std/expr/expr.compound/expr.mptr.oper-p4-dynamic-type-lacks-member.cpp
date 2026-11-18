// translation-fail: refcount, unsafe
// panic-ub: refcount
// nondet-result: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/4: the behavior is undefined if the most derived
// object of E1 does not contain the member to which E2 refers.

struct LacksValue {
  int own;
};

struct AddsValue : LacksValue {
  int value;
};

int main() {
  int LacksValue::*pm = static_cast<int LacksValue::*>(&AddsValue::value);
  LacksValue obj{1};
  return obj.*pm;
}
