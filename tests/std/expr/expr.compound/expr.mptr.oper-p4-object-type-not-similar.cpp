// translation-fail: refcount
// no-compile: unsafe
// panic-ub: refcount
// nondet-result: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/4: the behavior is undefined if the result of E1 is
// an object whose type is not similar to the type of E1.

struct HasTwoInts {
  int first;
  int second;
};

struct HasTwoShorts {
  short first;
  short second;
};

int main() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoShorts obj{3, 4};
  return reinterpret_cast<HasTwoInts &>(obj).*pm;
}
