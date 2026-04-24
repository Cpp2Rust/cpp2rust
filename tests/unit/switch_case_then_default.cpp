#include <cassert>

int case_then_default(int x) {
  int r = 0;
  switch (x) {
  case 1:
  default:
    r = 10;
    break;
  case 2:
    r = 20;
    break;
  }
  return r;
}

int main() {
  assert(case_then_default(1) == 10);
  assert(case_then_default(2) == 20);
  assert(case_then_default(99) == 10);
  return 0;
}
