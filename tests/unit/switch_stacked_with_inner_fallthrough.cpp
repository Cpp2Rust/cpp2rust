// panic
#include <cassert>

int stacked_with_inner_fallthrough(int x, int flag) {
  int r = 0;
  switch (x) {
  case 1:
  case 2:
  case 3:
    if (!flag) {
      r = 50;
      break;
    }
    [[fallthrough]];
  default:
    r = 999;
    break;
  }
  return r;
}

int main() {
  assert(stacked_with_inner_fallthrough(1, 0) == 50);
  assert(stacked_with_inner_fallthrough(2, 1) == 999);
  assert(stacked_with_inner_fallthrough(99, 0) == 999);
  return 0;
}
