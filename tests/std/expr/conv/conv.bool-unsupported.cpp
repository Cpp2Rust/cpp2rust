// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [conv.bool]/1: a prvalue of pointer-to-member type can be converted to
// bool; the null member pointer value becomes false, any other value true.
namespace p1_pointer_to_member {
struct HasTwoInts {
  int first;
  int second;
  void run() {}
};

void test() {
  int HasTwoInts::*null_pm = nullptr;
  int HasTwoInts::*first_pm = &HasTwoInts::first;
  int HasTwoInts::*second_pm = &HasTwoInts::second;
  void (HasTwoInts::*null_pmf)() = nullptr;
  void (HasTwoInts::*run_pmf)() = &HasTwoInts::run;
  bool null_as_bool = static_cast<bool>(null_pm);
  assert(!null_as_bool);
  bool first_as_bool = static_cast<bool>(first_pm);
  assert(first_as_bool);
  bool second_as_bool = static_cast<bool>(second_pm);
  assert(second_as_bool);
  bool null_function_as_bool = static_cast<bool>(null_pmf);
  assert(!null_function_as_bool);
  bool run_as_bool = static_cast<bool>(run_pmf);
  assert(run_as_bool);
}
} // namespace p1_pointer_to_member

int main() {
  p1_pointer_to_member::test();
  return 0;
}
