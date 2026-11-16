// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/3: E1->*E2 is converted into (*(E1)).*E2.
#include <cassert>

struct HasTwoInts {
  int first;
  int second;
};

int main() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  HasTwoInts *p = &obj;
  assert(p->*pm == 4);
  p->*pm = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
  assert(&(p->*pm) == &((*p).*pm));
  return 0;
}
