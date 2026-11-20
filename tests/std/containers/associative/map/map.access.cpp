// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <map>
#include <string>

// N4950 [map.access]/1: operator[](const key_type& x) is equivalent to
// return try_emplace(x).first->second, which value-initializes the mapped
// value of a key that is not in the map.
namespace p1_subscript_lvalue_key_default_string {
void test() {
  std::map<int, std::string> m;
  int key = 1;
  assert(m[key].size() == 0);
  assert(*m[key].c_str() == '\0');
  assert(m.size() == 1);
}
} // namespace p1_subscript_lvalue_key_default_string

// N4950 [map.access]/2: operator[](key_type&& x) is equivalent to
// return try_emplace(std::move(x)).first->second.
namespace p2_subscript_rvalue_key_default_string {
void test() {
  std::map<int, std::string> m;
  assert(m[1].size() == 0);
  assert(*m[1].c_str() == '\0');
  assert(m.size() == 1);
}
} // namespace p2_subscript_rvalue_key_default_string

int main() {
  p1_subscript_lvalue_key_default_string::test();
  p2_subscript_rvalue_key_default_string::test();
  return 0;
}
