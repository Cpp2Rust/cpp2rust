// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.namespace]/1: for each non-friend redeclaration whose
// target scope is the namespace scope, the portion after the declarator-id is
// also included in the scope.
#include <cassert>

static int global_calls;
static int inner_calls;

int base() {
  ++global_calls;
  return 100;
}

namespace Outer {
namespace Inner {
int base() {
  ++inner_calls;
  return 7;
}
int twice();
extern int stored;
} // namespace Inner

int Inner::twice() { return base() * 2; }
int Inner::stored = base() + 1;
} // namespace Outer

int main() {
  assert(Outer::Inner::stored == 8);
  assert(inner_calls == 1);

  assert(Outer::Inner::twice() == 14);
  assert(inner_calls == 2);
  assert(global_calls == 0);
  return 0;
}
