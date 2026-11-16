// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.namespace]/1: any namespace-definition for a namespace N
// introduces a namespace scope that includes the namespace-body for every
// namespace-definition for N.
#include <cassert>

namespace Tally {
int count = 4;
}

int count = 100;

namespace Tally {
int next() { return ++count; }
} // namespace Tally

int main() {
  assert(Tally::next() == 5);
  assert(Tally::count == 5);
  assert(count == 100);
  assert(::count == 100);
  return 0;
}
