// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <initializer_list>
#include <utility>
#include <vector>

// N4950 [class.copy.ctor]/11: a copy/move constructor that is not
// user-provided is trivial.
namespace p11_defaulted_move_constructor_is_trivial {
struct MoveOnly {
  int value;
  MoveOnly(int value) : value(value) {}
  MoveOnly(const MoveOnly &) = delete;
  MoveOnly(MoveOnly &&) = default;
};

MoveOnly pass(MoveOnly object) { return object; }

void test() {
  MoveOnly result = pass(MoveOnly(7));
  assert(result.value == 7);
}
} // namespace p11_defaulted_move_constructor_is_trivial

// N4950 [class.copy.ctor]/14: the implicitly-defined move constructor
// performs a memberwise move, so a trivial move leaves the source unchanged.
namespace p14_trivial_move_keeps_source {
int read_by_value(std::vector<int>::iterator it) { return *it; }

void test() {
  std::initializer_list<int> source = {1, 2, 3};
  std::initializer_list<int> moved = std::move(source);
  assert(moved.size() == 3);
  assert(source.size() == 3);
  std::vector<int> values{7, 8};
  std::vector<int>::iterator it = values.begin();
  assert(read_by_value(std::move(it)) == 7);
  assert(*it == 7);
  std::vector<int>::iterator moved_it = std::move(it);
  assert(*moved_it == 7);
  assert(*it == 7);
}
} // namespace p14_trivial_move_keeps_source

int main() {
  p11_defaulted_move_constructor_is_trivial::test();
  p14_trivial_move_keeps_source::test();
  return 0;
}
