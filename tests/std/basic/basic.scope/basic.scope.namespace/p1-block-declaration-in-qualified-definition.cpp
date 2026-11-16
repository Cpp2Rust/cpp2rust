// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.namespace]/1: for each non-friend redeclaration whose
// target scope is the namespace scope, the portion after the declarator-id is
// also included in the scope.
#include <cassert>

static int global_calls;
static int inner_calls;

int helper() {
  ++global_calls;
  return 100;
}

namespace Outer {
namespace Inner {
int run();
}

int Inner::run() {
  int helper();
  return helper();
}

namespace Inner {
int helper() {
  ++inner_calls;
  return 9;
}
} // namespace Inner
} // namespace Outer

int main() {
  assert(Outer::Inner::run() == 9);
  assert(inner_calls == 1);
  assert(global_calls == 0);
  return 0;
}
