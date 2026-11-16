// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/2: the underlying bytes of an object of
// trivially copyable type can be copied into an array of unsigned char; if
// they are copied back, the object holds its original value.
#include <cassert>
#include <cstring>

static int base_calls;
static int override_calls;

struct DeclaresVirtual {
  virtual void run() { ++base_calls; }
  void other() {}
};

struct OverridesVirtual : DeclaresVirtual {
  void run() override { ++override_calls; }
};

int main() {
  void (DeclaresVirtual::*pmf)() = &DeclaresVirtual::run;
  unsigned char bytes[sizeof(pmf)];
  std::memcpy(bytes, &pmf, sizeof(pmf));
  pmf = &DeclaresVirtual::other;
  std::memcpy(&pmf, bytes, sizeof(pmf));
  OverridesVirtual overrider;
  DeclaresVirtual &ref = overrider;
  (ref.*pmf)();
  assert(base_calls == 0);
  assert(override_calls == 1);
  return 0;
}
