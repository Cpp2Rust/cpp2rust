// translation-fail: refcount, unsafe
// panic-ub: refcount
// nondet-result: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/13: if class B neither contains the original member
// nor is a base or derived class of the class containing it, the behavior is
// undefined.

struct DeclaresValue {
  int value;
};

struct Unrelated {
  int other;
};

struct InheritsBoth : DeclaresValue, Unrelated {};

int main() {
  int InheritsBoth::*derived_pm = &DeclaresValue::value;
  int Unrelated::*unrelated_pm = static_cast<int Unrelated::*>(derived_pm);
  Unrelated obj{1};
  return obj.*unrelated_pm;
}
