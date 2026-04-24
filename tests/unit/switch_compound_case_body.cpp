#include <cassert>

int compound_case_body(int x) {
  int r = 0;
  switch (x) {
  case 1: {
    int y = 10;
    int z = 20;
    r = y + z;
    break;
  }
  case 2: {
    int y = 100;
    r = y - 1;
    break;
  }
  default:
    r = -1;
    break;
  }
  return r;
}

int main() {
  assert(compound_case_body(1) == 30);
  assert(compound_case_body(2) == 99);
  assert(compound_case_body(9) == -1);
  return 0;
}
