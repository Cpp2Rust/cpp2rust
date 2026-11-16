// translation-fail: refcount, unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <functional>

// N4950 [func.require]/1.1: INVOKE(f, t1, t2) is (t1.*f)(t2) when f is a
// pointer to a member function of a class T and t1 is a T or derived from T.
namespace p1_1_member_function_with_object {
struct AccumulatesCalls {
  int total = 0;
  int add(int amount) {
    total += amount;
    return total;
  }
};

struct InheritsAccumulator : AccumulatesCalls {};

void test() {
  int (AccumulatesCalls::*pmf)(int) = &AccumulatesCalls::add;
  AccumulatesCalls obj;
  assert(std::invoke(pmf, obj, 3) == 3);
  assert(obj.total == 3);
  InheritsAccumulator derived;
  assert(std::invoke(pmf, derived, 4) == 4);
  assert(derived.total == 4);
  assert(obj.total == 3);
}
} // namespace p1_1_member_function_with_object

// N4950 [func.require]/1.2: INVOKE(f, t1, t2) is (t1.get().*f)(t2) when f is a
// pointer to a member function and t1 is a reference_wrapper.
namespace p1_2_member_function_with_reference_wrapper {
struct AccumulatesCalls {
  int total = 0;
  int add(int amount) {
    total += amount;
    return total;
  }
};

struct InheritsAccumulator : AccumulatesCalls {};

void test() {
  int (AccumulatesCalls::*pmf)(int) = &AccumulatesCalls::add;
  AccumulatesCalls obj;
  assert(std::invoke(pmf, std::ref(obj), 3) == 3);
  assert(obj.total == 3);
}
} // namespace p1_2_member_function_with_reference_wrapper

// N4950 [func.require]/1.3: INVOKE(f, t1, t2) is ((*t1).*f)(t2) when f is a
// pointer to a member function and t1 is neither a T nor a reference_wrapper.
namespace p1_3_member_function_with_pointer {
struct AccumulatesCalls {
  int total = 0;
  int add(int amount) {
    total += amount;
    return total;
  }
};

struct InheritsAccumulator : AccumulatesCalls {};

void test() {
  int (AccumulatesCalls::*pmf)(int) = &AccumulatesCalls::add;
  AccumulatesCalls obj;
  assert(std::invoke(pmf, &obj, 3) == 3);
  assert(obj.total == 3);
}
} // namespace p1_3_member_function_with_pointer

// N4950 [func.require]/1.4: INVOKE(f, t1) is t1.*f when f is a pointer to data
// member of a class T and t1 is a T or derived from T.
namespace p1_4_data_member_with_object {
struct HasTwoInts {
  int first;
  int second;
};

struct InheritsTwoInts : HasTwoInts {};

void test() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  assert(std::invoke(pm, obj) == 4);
  std::invoke(pm, obj) = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
  InheritsTwoInts derived{{6, 7}};
  assert(std::invoke(pm, derived) == 7);
}
} // namespace p1_4_data_member_with_object

// N4950 [func.require]/1.5: INVOKE(f, t1) is t1.get().*f when f is a pointer
// to data member and t1 is a reference_wrapper.
namespace p1_5_data_member_with_reference_wrapper {
struct HasTwoInts {
  int first;
  int second;
};

struct InheritsTwoInts : HasTwoInts {};

void test() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  assert(std::invoke(pm, std::ref(obj)) == 4);
  std::invoke(pm, std::ref(obj)) = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
}
} // namespace p1_5_data_member_with_reference_wrapper

// N4950 [func.require]/1.6: INVOKE(f, t1) is (*t1).*f when f is a pointer to
// data member and t1 is neither a T nor a reference_wrapper.
namespace p1_6_data_member_with_pointer {
struct HasTwoInts {
  int first;
  int second;
};

struct InheritsTwoInts : HasTwoInts {};

void test() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  assert(std::invoke(pm, &obj) == 4);
  std::invoke(pm, &obj) = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
}
} // namespace p1_6_data_member_with_pointer

int main() {
  p1_1_member_function_with_object::test();
  p1_2_member_function_with_reference_wrapper::test();
  p1_3_member_function_with_pointer::test();
  p1_4_data_member_with_object::test();
  p1_5_data_member_with_reference_wrapper::test();
  p1_6_data_member_with_pointer::test();
  return 0;
}
