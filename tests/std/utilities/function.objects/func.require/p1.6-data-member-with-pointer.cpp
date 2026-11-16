// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [func.require]/1.6: INVOKE(f, t1) is (*t1).*f when f is a pointer to
// data member and t1 is neither a T nor a reference_wrapper.
#include <cassert>
#include <functional>

struct HasTwoInts {
  int first;
  int second;
};

struct InheritsTwoInts : HasTwoInts {};

int main() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  assert(std::invoke(pm, &obj) == 4);
  std::invoke(pm, &obj) = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
  return 0;
}
