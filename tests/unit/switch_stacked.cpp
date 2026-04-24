#include <cassert>

int stacked(int x) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  case 3:
    r = 100;
    break;
  case 4:
  case 5:
    r = 200;
    break;
  default:
    r = 300;
    break;
  }
  return r;
}

int main() {
  assert(stacked(1) == 100);
  assert(stacked(3) == 100);
  assert(stacked(5) == 200);
  assert(stacked(9) == 300);
  return 0;
}
