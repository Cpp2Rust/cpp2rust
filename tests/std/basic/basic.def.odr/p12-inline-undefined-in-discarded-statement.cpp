// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/12: an inline function needs a reachable definition
// only where it is odr-used outside of a discarded statement. f is used only in
// a discarded statement, so it is never defined and never runs.
#include <cassert>

static int g_calls;

inline void f();

void g() {
  if constexpr (false) {
    f();
  }
  ++g_calls;
}

int main() {
  g();
  assert(g_calls == 1);
  return 0;
}
