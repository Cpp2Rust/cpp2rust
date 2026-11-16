// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.fctptr]/1: "pointer to member of type noexcept function"
// converts to "pointer to member of type function"; the result designates the
// member function.
#include <cassert>

struct HasNoexceptMember {
  int calls = 0;
  void run() noexcept { ++calls; }
};

int main() {
  void (HasNoexceptMember::*noexcept_pmf)() noexcept = &HasNoexceptMember::run;
  void (HasNoexceptMember::*pmf)() = noexcept_pmf;
  HasNoexceptMember obj;
  (obj.*pmf)();
  assert(obj.calls == 1);
  return 0;
}
