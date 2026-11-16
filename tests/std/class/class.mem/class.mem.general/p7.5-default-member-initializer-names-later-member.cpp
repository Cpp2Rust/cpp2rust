// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mem.general]/7.5: a default member initializer within the
// member-specification of a class is a complete-class context.
#include <cassert>

static int member_calls;

int base = 100;
int scaled(int amount) { return amount * 1000; }

struct HasLaterInitializerOperands {
  int first = base + 1;
  int second = scaled(first);

  static inline int base = 4;
  int scaled(int amount) const {
    ++member_calls;
    return amount * 10;
  }
};

int main() {
  HasLaterInitializerOperands object;
  assert(object.first == 5);
  assert(object.second == 50);
  assert(member_calls == 1);
  assert(base == 100);
  return 0;
}
