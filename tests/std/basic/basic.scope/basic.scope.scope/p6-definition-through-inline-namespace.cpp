// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/6: a declaration is nominable in a namespace E if
// its target scope is the scope of E or of any element of the inline namespace
// set of E.
#include <cassert>

static int version_calls;

namespace Api {
inline namespace Latest {
int version();
extern int revision;
} // namespace Latest
} // namespace Api

int Api::version() {
  ++version_calls;
  return 2;
}

int Api::revision = 40;

int main() {
  assert(Api::version() == 2);
  assert(version_calls == 1);

  assert(Api::Latest::version() == 2);
  assert(version_calls == 2);

  assert(&Api::revision == &Api::Latest::revision);
  Api::revision = 41;
  assert(Api::Latest::revision == 41);
  return 0;
}
