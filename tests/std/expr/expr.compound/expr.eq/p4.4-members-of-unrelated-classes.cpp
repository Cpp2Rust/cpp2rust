// nondet-result
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.4: if one operand refers to a member of class C1 and the
// other to a member of a different class C2, neither a base class of the
// other, the result is unspecified.

struct CommonBase {};

struct FirstDerived : CommonBase {
  int value;
};

struct SecondDerived : CommonBase {
  int value;
};

int main() {
  int CommonBase::*first_pm =
      static_cast<int CommonBase::*>(&FirstDerived::value);
  int CommonBase::*second_pm =
      static_cast<int CommonBase::*>(&SecondDerived::value);
  return first_pm == second_pm;
}
