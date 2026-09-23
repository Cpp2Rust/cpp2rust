#include <cassert>
#include <cstdlib>

enum E { A, B, C };

int main() {
  size_t size = 1;
  assert(size == 1);

  unsigned long ul = 5;
  size_t s1 = size_t(ul);
  assert(s1 == 5);
  ul = (unsigned long)size_t(7);
  assert(ul == 7);

  int i = 2;
  E e = E(i);
  assert(e == C);
  assert(int(e) == 2);
  return 0;
}
