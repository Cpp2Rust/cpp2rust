// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <map>

// N4950 [map.access]/1: operator[](const key_type& x) is equivalent to
// try_emplace(x).first->second.
namespace p1_subscript_pointer_lvalue {
void test() {
  int first = 0, second = 0;
  std::map<int *, int> counts;
  int *key = &first;
  counts[key] = 1;
  key = &second;
  counts[key] = 2;
  counts[key] += 3;
  assert(counts.size() == 2);
  assert(counts[key] == 5);
  key = &first;
  assert(counts[key] == 1);
  assert(first == 0 && second == 0);
}
} // namespace p1_subscript_pointer_lvalue

// N4950 [map.access]/2: operator[](key_type&& x) is equivalent to
// try_emplace(std::move(x)).first->second.
namespace p2_subscript_pointer_rvalue {
void test() {
  int first = 0, second = 0;
  std::map<int *, int> counts;
  counts[&first] = 1;
  counts[&second] = 2;
  counts[&second] += 3;
  assert(counts.size() == 2);
  assert(counts[&first] == 1);
  assert(counts[&second] == 5);
  assert(first == 0 && second == 0);
}
} // namespace p2_subscript_pointer_rvalue

int main() {
  p1_subscript_pointer_lvalue::test();
  p2_subscript_pointer_rvalue::test();
  return 0;
}
