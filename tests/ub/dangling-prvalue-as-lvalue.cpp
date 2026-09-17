// panic-ub: refcount
// nondet-result: unsafe

#include <vector>

const int &foo(const int &a) { return a; }

int main() {
  std::vector<int> v = {1, 2};
  const int &b = foo(*v.begin());
  v.clear();
  return b;
}
