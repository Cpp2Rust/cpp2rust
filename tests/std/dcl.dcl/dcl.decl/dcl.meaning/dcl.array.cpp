// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [dcl.array]/6: an object of type "array of N U" consists of a
// contiguously allocated set of N subobjects of type U, numbered 0 to N-1.
namespace p6_array_of_pointers_to_data_members {
struct HasThreeInts {
  int first;
  int second;
  int third;
};

void test() {
  int HasThreeInts::*members[3] = {&HasThreeInts::third, &HasThreeInts::first,
                                   &HasThreeInts::second};
  HasThreeInts obj{4, 5, 6};
  assert(obj.*members[0] == 6);
  assert(obj.*members[1] == 4);
  assert(obj.*members[2] == 5);
  int HasThreeInts::**element = members;
  assert(element + 1 == &members[1]);
  assert(obj.*element[1] == 4);
  ++element;
  ++element;
  assert(element == &members[2]);
  assert(obj.**element == 5);
  members[2] = &HasThreeInts::third;
  assert(obj.**element == 6);
}
} // namespace p6_array_of_pointers_to_data_members

int main() {
  p6_array_of_pointers_to_data_members::test();
  return 0;
}
