// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.unary.op]/3.2: for an operand that is not a non-static member,
// the result has type "pointer to T" and points to the designated object or
// function; a static member is such an operand.
namespace p3_2_static_member_is_ordinary_pointer {
struct HasStaticMembers {
  static inline int shared = 3;
  static inline int calls = 0;
  static void run() { ++calls; }
};

void test() {
  int *object_pointer = &HasStaticMembers::shared;
  assert(*object_pointer == 3);
  *object_pointer = 4;
  assert(HasStaticMembers::shared == 4);
  void (*function_pointer)() = &HasStaticMembers::run;
  function_pointer();
  assert(HasStaticMembers::calls == 1);
}
} // namespace p3_2_static_member_is_ordinary_pointer

// N4950 [expr.unary.op]/3.2: "Otherwise, the result has type "pointer to T" and
// points to the designated object (6.7.1) or function (6.8.4). If the operand
// names an explicit object member function (9.3.4.6), the operand shall be a
// qualified-id."
namespace p3_2_explicit_object_member_is_ordinary_pointer {
struct HasExplicitObjectMember {
  int base;
  int add(this const HasExplicitObjectMember &self, int amount) {
    return self.base + amount;
  }
};

void test() {
  int (*function_pointer)(const HasExplicitObjectMember &, int) =
      &HasExplicitObjectMember::add;
  HasExplicitObjectMember object{3};
  assert(function_pointer(object, 4) == 7);
  object.base = 5;
  assert(function_pointer(object, 1) == 6);
}
} // namespace p3_2_explicit_object_member_is_ordinary_pointer

// N4950 [expr.unary.op]/4: a pointer to member is only formed when an explicit
// & is used and its operand is a qualified-id not enclosed in parentheses.
namespace p4_parenthesized_qualified_id_is_object_pointer {
struct TakesAddressOfOwnMember {
  int value;
  int *parenthesized() { return &(TakesAddressOfOwnMember::value); }
  int *unqualified() { return &value; }
};

void test() {
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
}
} // namespace p4_parenthesized_qualified_id_is_object_pointer

int main() {
  p3_2_static_member_is_ordinary_pointer::test();
  p3_2_explicit_object_member_is_ordinary_pointer::test();
  p4_parenthesized_qualified_id_is_object_pointer::test();
  return 0;
}
