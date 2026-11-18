#include <cassert>

// N4950 [class.conv.fct]/2: A conversion function shall have no non-object
// parameters and shall be a non-static member function of a class or class
// template X; it specifies a conversion from X to the type specified by the
// conversion-type-id
namespace p2_non_const_conversion_function {
struct Gauge {
  long value;
  operator long() { return ++value; }
};

void test() {
  Gauge gauge{20};
  long converted = gauge;
  assert(converted == 21);
  assert(gauge.value == 21);
}
} // namespace p2_non_const_conversion_function

int main() {
  p2_non_const_conversion_function::test();
  return 0;
}
