#include <cassert>

enum E { A, B };

int global = E::A != E::B;

int main() {
  assert(global == 1);
  return 0;
}
