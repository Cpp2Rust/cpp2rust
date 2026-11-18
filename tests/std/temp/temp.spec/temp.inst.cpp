// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [temp.inst]/2: a class template specialization is implicitly
// instantiated only when it is referenced in a context that requires a
// completely-defined object type.
namespace p2_reference_does_not_instantiate {
template <class T> struct Wrapper {
  typename T::type value;
};

struct Member {
  int value = 1;
  Member() = default;
  explicit Member(const Wrapper<int> &) : value(2) {}
};

struct Holder {
  Member member;
};

void test() {
  Holder holder;
  assert(holder.member.value == 1);
  holder.member.value = 3;
  assert(holder.member.value == 3);
}
} // namespace p2_reference_does_not_instantiate

int main() {
  p2_reference_does_not_instantiate::test();
  return 0;
}
