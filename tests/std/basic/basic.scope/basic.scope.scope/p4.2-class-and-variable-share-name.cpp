// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4.2: two declarations that (re)introduce the same
// name do not correspond if one declares a type (not a typedef-name) and the
// other declares a variable.
#include <cassert>

struct Limit {
  int value;
};

int Limit = 12;

int main() {
  struct Limit limit = {3};
  assert(limit.value == 3);
  assert(Limit == 12);

  Limit += limit.value;
  assert(Limit == 15);
  assert(limit.value == 3);
  return 0;
}
