// nondet-result
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.reinterpret.cast]/10: the result of converting a pointer to
// member to a different pointer-to-member type is unspecified.

struct HasTwoInts {
  int first;
  int second;
};

struct AlsoHasTwoInts {
  int first;
  int second;
};

int main() {
  int AlsoHasTwoInts::*converted =
      reinterpret_cast<int AlsoHasTwoInts::*>(&HasTwoInts::second);
  return converted == &AlsoHasTwoInts::second;
}
