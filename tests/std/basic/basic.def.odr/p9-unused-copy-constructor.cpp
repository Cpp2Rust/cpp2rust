// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/9: a constructor is odr-used only where an
// initialization selects it. No HasUndefinedCopyCtor is ever copied, so its
// copy constructor is never defined.
#include <cassert>

static int ctor_calls;

struct HasUndefinedCopyCtor {
  int value = 3;

  HasUndefinedCopyCtor() { ++ctor_calls; }
  HasUndefinedCopyCtor(const HasUndefinedCopyCtor &);
};

int read(const HasUndefinedCopyCtor &object) { return object.value; }

int main() {
  HasUndefinedCopyCtor a;
  assert(read(a) == 3);
  assert(ctor_calls == 1);
  return 0;
}
