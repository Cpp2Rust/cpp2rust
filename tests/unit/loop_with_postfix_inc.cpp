#include <cassert>

int main() {
  int x = 0;
  while (x++ < 100 && x != 50) {
    ++x;
  }
  assert(x == 101);
  return 0;
}
