// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/5.2: the exemption requires a variable with no mutable
// subobjects. k has one, so reading k.value odr-uses k and [=] captures it.
#include <cassert>

static int copies;

struct HasMutableMember {
  int value;
  mutable int scratch;

  constexpr HasMutableMember(int v) : value(v), scratch(0) {}
  HasMutableMember(const HasMutableMember &o)
      : value(o.value), scratch(o.scratch) {
    ++copies;
  }
};

int main() {
  constexpr HasMutableMember k{5};
  auto f = [=] { return k.value; };
  assert(copies == 1);
  assert(f() == 5);
  assert(copies == 1);
  return 0;
}
