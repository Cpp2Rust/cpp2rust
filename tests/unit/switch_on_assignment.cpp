#include <cassert>

int switch_on_assignment(int x) {
  int y = 0;
  int r = 0;
  switch (y = x + 1) {
  case 1:
    r = 10;
    break;
  case 2:
    r = 20;
    break;
  default:
    r = y;
    break;
  }
  return r;
}

int main() {
  assert(switch_on_assignment(0) == 10);
  assert(switch_on_assignment(1) == 20);
  assert(switch_on_assignment(9) == 10);
  return 0;
}
