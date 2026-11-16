// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [class.mem.general]/7.1: a function body within the
// member-specification of a class is a complete-class context.
namespace p7_1_function_body_names_later_member {
int stock = 100;
int minimum() { return 50; }

struct HasLaterMembers {
  int take(int amount) {
    stock -= amount;
    return stock + minimum() + reserve;
  }

  int stock = 9;
  int minimum() const { return 2; }
  enum { reserve = 30 };
};

void test() {
  HasLaterMembers object;
  assert(object.take(3) == 38);
  assert(object.stock == 6);
  assert(stock == 100);
}
} // namespace p7_1_function_body_names_later_member

// N4950 [class.mem.general]/7.2: a default argument within the
// member-specification of a class is a complete-class context.
namespace p7_2_default_argument_names_later_member {
int step = 100;

struct HasLaterStaticStep {
  int total = 1;

  int advance(int amount = step) {
    total += amount;
    return total;
  }

  static inline int step = 4;
};

void test() {
  HasLaterStaticStep object;
  assert(object.advance() == 5);
  assert(object.advance(2) == 7);
  assert(step == 100);
}
} // namespace p7_2_default_argument_names_later_member

int main() {
  p7_1_function_body_names_later_member::test();
  p7_2_default_argument_names_later_member::test();
  return 0;
}
