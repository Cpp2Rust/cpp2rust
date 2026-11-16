// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4.2: two declarations that (re)introduce the same
// name do not correspond if one declares a type (not a typedef-name) and the
// other declares a function.
#include <cassert>

static int function_calls;

struct Reading {
  int value;
};

int Reading(int raw) {
  ++function_calls;
  return raw + 1;
}

int main() {
  struct Reading reading = {4};
  assert(reading.value == 4);
  assert(function_calls == 0);

  assert(Reading(reading.value) == 5);
  assert(function_calls == 1);
  return 0;
}
