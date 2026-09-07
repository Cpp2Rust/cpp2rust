#include <cassert>

enum E {
  A = 1 << 1,
  B = 1 << 2,
  C = A,
  D = 1 << 2,
};

int main() {
  assert(E::A == 1 << 1);
  assert(E::B == 1 << 2);
  assert(E::C == 1 << 1);
  assert(E::D == 1 << 2);
  return 0;
}
