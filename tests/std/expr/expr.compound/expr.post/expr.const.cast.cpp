#include <cassert>
#include <vector>

// N4950 [expr.const.cast]/4: For two object types T1 and T2, if a pointer to T1
// can be explicitly converted to the type "pointer to T2" using a const_cast,
// then the following conversions can also be made:
// (4.1) an lvalue of type T1 can be explicitly converted to an lvalue of type
// T2 using the cast const_cast<T2&>;
// (4.2) a glvalue of type T1 can be explicitly converted to an xvalue of type
// T2 using the cast const_cast<T2&&>;
namespace p4_reference_const_cast {
struct Pair {
  int x;
};

void take(int &&r) { r = 8; }

void test() {
  int v = 1;
  const int &cr = v;
  const_cast<int &>(cr) = 2;
  assert(v == 2);
  take(const_cast<int &&>(cr));
  assert(v == 8);

  Pair pair{1};
  const Pair &cpair = pair;
  const_cast<Pair &>(cpair).x = 2;
  assert(pair.x == 2);

  std::vector<int> vec{1};
  const std::vector<int> &cvec = vec;
  const_cast<std::vector<int> &>(cvec).push_back(2);
  assert(vec.size() == 2);
}
} // namespace p4_reference_const_cast

int main() {
  p4_reference_const_cast::test();
  return 0;
}
