// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.reinterpret.cast]/10.1: converting a pointer to member function
// to a different pointer-to-member-function type and back yields the original
// value.
namespace p10_1_member_function_round_trip {
struct CountsCalls {
  int calls = 0;
  void run() { ++calls; }
};

struct Unrelated {};

void test() {
  void (Unrelated::*converted)(int) =
      reinterpret_cast<void (Unrelated::*)(int)>(&CountsCalls::run);
  void (CountsCalls::*back)() =
      reinterpret_cast<void (CountsCalls::*)()>(converted);
  assert(back == &CountsCalls::run);
  CountsCalls obj;
  (obj.*back)();
  assert(obj.calls == 1);
}
} // namespace p10_1_member_function_round_trip

int main() {
  p10_1_member_function_round_trip::test();
  return 0;
}
