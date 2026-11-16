// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.unary.op]/3.2: for an operand that is not a non-static member,
// the result has type "pointer to T" and points to the designated object or
// function; a static member is such an operand.
#include <cassert>

struct HasStaticMembers {
  static inline int shared = 3;
  static inline int calls = 0;
  static void run() { ++calls; }
};

int main() {
  int *object_pointer = &HasStaticMembers::shared;
  assert(*object_pointer == 3);
  *object_pointer = 4;
  assert(HasStaticMembers::shared == 4);
  void (*function_pointer)() = &HasStaticMembers::run;
  function_pointer();
  assert(HasStaticMembers::calls == 1);
  return 0;
}
