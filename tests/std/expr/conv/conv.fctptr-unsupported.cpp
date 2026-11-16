// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [conv.fctptr]/1: "pointer to member of type noexcept function"
// converts to "pointer to member of type function"; the result designates the
// member function.
namespace p1_pointer_to_noexcept_member_function {
struct HasNoexceptMember {
  int calls = 0;
  void run() noexcept { ++calls; }
};

void test() {
  void (HasNoexceptMember::*noexcept_pmf)() noexcept = &HasNoexceptMember::run;
  void (HasNoexceptMember::*pmf)() = noexcept_pmf;
  HasNoexceptMember obj;
  (obj.*pmf)();
  assert(obj.calls == 1);
}
} // namespace p1_pointer_to_noexcept_member_function

int main() {
  p1_pointer_to_noexcept_member_function::test();
  return 0;
}
