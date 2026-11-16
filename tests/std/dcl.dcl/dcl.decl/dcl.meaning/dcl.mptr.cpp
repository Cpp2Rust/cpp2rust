// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [dcl.mptr]/2: the type is "derived-declarator-type-list pointer to
// member of class C of type T"; T can be an array type.
namespace p2_pointer_to_array_member {
struct HasIntArray {
  int before;
  int elements[3];
};

void test() {
  int (HasIntArray::*pm)[3] = &HasIntArray::elements;
  HasIntArray obj{1, {2, 3, 4}};
  assert((obj.*pm)[0] == 2);
  assert((obj.*pm)[2] == 4);
  (obj.*pm)[1] = 7;
  assert(obj.elements[1] == 7);
  assert(obj.before == 1);
}
} // namespace p2_pointer_to_array_member

// N4950 [dcl.mptr]/2: "T C::*" declares a pointer to member of class C of
// type T.
namespace p2_pointer_to_data_member {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*pm = &HasTwoInts::first;
  HasTwoInts obj{3, 4};
  assert(obj.*pm == 3);
  obj.*pm = 5;
  assert(obj.first == 5);
  assert(obj.second == 4);
  pm = &HasTwoInts::second;
  assert(obj.*pm == 4);
  obj.*pm = 6;
  assert(obj.first == 5);
  assert(obj.second == 6);
}
} // namespace p2_pointer_to_data_member

// N4950 [dcl.mptr]/2: the nested-name-specifier denotes a class; the class
// need not be complete.
namespace p2_pointer_to_member_of_incomplete_class {
struct CompletedLater;

static int CompletedLater::*pm = nullptr;

struct CompletedLater {
  int first;
  int second;
};

void test() {
  assert(pm == nullptr);
  pm = &CompletedLater::second;
  CompletedLater obj{3, 4};
  assert(obj.*pm == 4);
}
} // namespace p2_pointer_to_member_of_incomplete_class

// N4950 [dcl.mptr]/2: the type is "derived-declarator-type-list pointer to
// member of class C of type T"; T can itself be a pointer-to-member type.
namespace p2_pointer_to_member_of_pointer_to_member_type {
struct HasTwoInts {
  int first;
  int second;
};

struct HoldsSelector {
  int HasTwoInts::*selector;
};

void test() {
  int HasTwoInts::*HoldsSelector::*pm = &HoldsSelector::selector;
  HoldsSelector holder{&HasTwoInts::second};
  HasTwoInts obj{3, 4};
  assert(obj.*(holder.*pm) == 4);
  holder.*pm = &HasTwoInts::first;
  assert(obj.*(holder.selector) == 3);
}
} // namespace p2_pointer_to_member_of_pointer_to_member_type

int main() {
  p2_pointer_to_array_member::test();
  p2_pointer_to_data_member::test();
  p2_pointer_to_member_of_incomplete_class::test();
  p2_pointer_to_member_of_pointer_to_member_type::test();
  return 0;
}
