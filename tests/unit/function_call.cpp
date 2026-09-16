#include <cassert>

int function(int y, int z) {
  int x = 5;
  return x + y + z;
}

int main() {
  int y = function(10, 1);
  assert(y == 16);
  return 0;
}
