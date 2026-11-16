// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.unary.op]/4: a pointer to member is only formed when an explicit
// & is used and its operand is a qualified-id not enclosed in parentheses.
#include <cassert>

struct TakesAddressOfOwnMember {
  int value;
  int *parenthesized() { return &(TakesAddressOfOwnMember::value); }
  int *unqualified() { return &value; }
};

int main() {
  TakesAddressOfOwnMember first{3};
  TakesAddressOfOwnMember second{4};
  int *p = first.parenthesized();
  assert(p == &first.value);
  *p = 5;
  assert(first.value == 5);
  assert(second.value == 4);
  int *q = second.unqualified();
  assert(q == &second.value);
  *q = 6;
  assert(first.value == 5);
  assert(second.value == 6);
  return 0;
}
