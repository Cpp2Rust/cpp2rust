// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mem.general]/7.1: a function body within the
// member-specification of a class is a complete-class context.
#include <cassert>

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

int main() {
  HasLaterMembers object;
  assert(object.take(3) == 38);
  assert(object.stock == 6);
  assert(stock == 100);
  return 0;
}
