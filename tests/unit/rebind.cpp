#include <cassert>

int main() {
  int x = 1;
  int &r = x;
  int y = 10;
  r = y;
  y += 1;
  assert(x == 10);
  return 0;
}
