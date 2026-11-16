// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.mptr]/2: the type is "derived-declarator-type-list pointer to
// member of class C of type T"; T can itself be a pointer-to-member type.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

struct HoldsSelector {
  int HasTwoInts::*selector;
};

int main() {
  int HasTwoInts::*HoldsSelector::*pm = &HoldsSelector::selector;
  HoldsSelector holder{&HasTwoInts::second};
  HasTwoInts obj{3, 4};
  assert(obj.*(holder.*pm) == 4);
  holder.*pm = &HasTwoInts::first;
  assert(obj.*(holder.selector) == 3);
  return 0;
}
