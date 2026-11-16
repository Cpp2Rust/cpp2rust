// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.temp]/2: each template-declaration introduces a template
// parameter scope that extends from the beginning of its
// template-parameter-list to the end of the template-declaration.
namespace p2_template_parameter_hides_outer_name {
const int width = 3;

template <int width> int area(int height) { return width * height; }

int plain_area(int height) { return width * height; }

void test() {
  assert(area<5>(2) == 10);
  assert(area<7>(2) == 14);
  assert(plain_area(2) == 6);
}
} // namespace p2_template_parameter_hides_outer_name

int main() {
  p2_template_parameter_hides_outer_name::test();
  return 0;
}
