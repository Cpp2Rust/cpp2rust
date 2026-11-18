#include <cassert>
#include <vector>

// N4950 [expr.cast]/4: The conversions performed by
// (4.1) a const_cast (7.6.1.11),
namespace p4_cast_notation_as_const_cast {
void set(int &r) { r = 7; }

void take(int &&r) { r = 8; }

void test() {
  int v = 1;
  const int &cr = v;
  set((int &)cr);
  assert(v == 7);
  take((int &&)cr);
  assert(v == 8);

  std::vector<int> vec{1};
  const std::vector<int> &cvec = vec;
  ((std::vector<int> &)cvec).push_back(2);
  assert(vec.size() == 2);
}
} // namespace p4_cast_notation_as_const_cast

int main() {
  p4_cast_notation_as_const_cast::test();
  return 0;
}
