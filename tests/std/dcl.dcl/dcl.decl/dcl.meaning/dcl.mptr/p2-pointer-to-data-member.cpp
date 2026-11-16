// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.mptr]/2: "T C::*" declares a pointer to member of class C of
// type T.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
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
  return 0;
}
