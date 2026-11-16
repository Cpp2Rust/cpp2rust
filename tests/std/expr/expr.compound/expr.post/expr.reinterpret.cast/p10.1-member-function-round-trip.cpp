// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.reinterpret.cast]/10.1: converting a pointer to member function
// to a different pointer-to-member-function type and back yields the original
// value.
#include <cassert>

struct CountsCalls {
  int calls = 0;
  void run() { ++calls; }
};

struct Unrelated {};

int main() {
  void (Unrelated::*converted)(int) =
      reinterpret_cast<void (Unrelated::*)(int)>(&CountsCalls::run);
  void (CountsCalls::*back)() =
      reinterpret_cast<void (CountsCalls::*)()>(converted);
  assert(back == &CountsCalls::run);
  CountsCalls obj;
  (obj.*back)();
  assert(obj.calls == 1);
  return 0;
}
