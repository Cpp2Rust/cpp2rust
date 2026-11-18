#include <cassert>

// N4950 [class.conv.ctor]/1: A constructor that is not explicit (9.2.3)
// specifies a conversion from the types of its parameters (if any) to the type
// of its class.
namespace p1_template_constructor_from_own_type {
struct Probe {
  int value;
  Probe(int v) : value(v) {}
  template <class U> Probe(U &u) : value(u.value + 1) {}
};

void test() {
  Probe probe(22);
  Probe converted(probe);
  assert(converted.value == 23);
}
} // namespace p1_template_constructor_from_own_type

int main() {
  p1_template_constructor_from_own_type::test();
  return 0;
}
