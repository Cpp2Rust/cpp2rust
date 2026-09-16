// ub: unsafe
// panic-ub: refcount

#include <algorithm>

int main() {
  const int &a = std::min(1, 2);
  const int &b = std::max(1, 2);
  return a == b;
}
