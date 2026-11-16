// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstring>

// N4950 [basic.types.general]/2: the underlying bytes of an object of
// trivially copyable type can be copied into an array of unsigned char; if
// they are copied back, the object holds its original value.
namespace p2_pointer_to_member_function_bytes_round_trip {
struct HasTwoMemberFunctions {
  int first_calls = 0;
  int second_calls = 0;
  void first() { ++first_calls; }
  void second() { ++second_calls; }
};

void test() {
  void (HasTwoMemberFunctions::*pmf)() = &HasTwoMemberFunctions::second;
  unsigned char bytes[sizeof(pmf)];
  std::memcpy(bytes, &pmf, sizeof(pmf));
  pmf = &HasTwoMemberFunctions::first;
  std::memcpy(&pmf, bytes, sizeof(pmf));
  assert(pmf == &HasTwoMemberFunctions::second);
  HasTwoMemberFunctions obj;
  (obj.*pmf)();
  assert(obj.first_calls == 0);
  assert(obj.second_calls == 1);

  void (HasTwoMemberFunctions::*null_pmf)() = nullptr;
  std::memcpy(bytes, &null_pmf, sizeof(null_pmf));
  null_pmf = &HasTwoMemberFunctions::first;
  std::memcpy(&null_pmf, bytes, sizeof(null_pmf));
  assert(null_pmf == nullptr);
}
} // namespace p2_pointer_to_member_function_bytes_round_trip

// N4950 [basic.types.general]/2: the underlying bytes of an object of
// trivially copyable type can be copied into an array of unsigned char; if
// they are copied back, the object holds its original value.
namespace p2_pointer_to_virtual_member_function_bytes_round_trip {
static int base_calls;
static int override_calls;

struct DeclaresVirtual {
  virtual void run() { ++base_calls; }
  void other() {}
};

struct OverridesVirtual : DeclaresVirtual {
  void run() override { ++override_calls; }
};

void test() {
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
}
} // namespace p2_pointer_to_virtual_member_function_bytes_round_trip

// N4950 [basic.types.general]/3: if the underlying bytes of obj1 of trivially
// copyable type are copied into obj2, obj2 holds the same value as obj1.
namespace p3_class_holding_pointers_to_members_bytes_copied {
struct HasValueAndIncrement {
  int padding;
  int value;
  void increment() { ++value; }
};

struct HoldsPointersToMembers {
  int HasValueAndIncrement::*data;
  void (HasValueAndIncrement::*function)();
};

void test() {
  HoldsPointersToMembers source{&HasValueAndIncrement::value,
                                &HasValueAndIncrement::increment};
  HoldsPointersToMembers destination{nullptr, nullptr};
  std::memcpy(&destination, &source, sizeof(source));
  assert(destination.data == &HasValueAndIncrement::value);
  assert(destination.function == &HasValueAndIncrement::increment);
  HasValueAndIncrement obj{1, 2};
  (obj.*destination.function)();
  assert(obj.*destination.data == 3);
  assert(obj.padding == 1);
}
} // namespace p3_class_holding_pointers_to_members_bytes_copied

// N4950 [basic.types.general]/3: if the underlying bytes of obj1 of trivially
// copyable type are copied into obj2, obj2 holds the same value as obj1.
namespace p3_pointer_to_member_function_bytes_copied_to_other_object {
struct HasTwoMemberFunctions {
  int first_calls = 0;
  int second_calls = 0;
  void first() { ++first_calls; }
  void second() { ++second_calls; }
};

void test() {
  void (HasTwoMemberFunctions::*source)() = &HasTwoMemberFunctions::second;
  void (HasTwoMemberFunctions::*destination)() = &HasTwoMemberFunctions::first;
  std::memcpy(&destination, &source, sizeof(source));
  assert(destination == source);
  HasTwoMemberFunctions obj;
  (obj.*destination)();
  assert(obj.first_calls == 0);
  assert(obj.second_calls == 1);

  void (HasTwoMemberFunctions::*null_source)() = nullptr;
  std::memcpy(&destination, &null_source, sizeof(null_source));
  assert(destination == nullptr);
}
} // namespace p3_pointer_to_member_function_bytes_copied_to_other_object

int main() {
  p2_pointer_to_member_function_bytes_round_trip::test();
  p2_pointer_to_virtual_member_function_bytes_round_trip::test();
  p3_class_holding_pointers_to_members_bytes_copied::test();
  p3_pointer_to_member_function_bytes_copied_to_other_object::test();
  return 0;
}
