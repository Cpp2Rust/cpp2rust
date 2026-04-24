// panic
#include <cassert>

int for_in_switch_break(int n) {
  int r = 0;
  switch (n) {
  case 0:
    for (int i = 0; i < 10; ++i) {
      if (i == 3)
        break;
      r += i;
    }
    r += 100;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}

int main() {
  assert(for_in_switch_break(0) == 103);
  assert(for_in_switch_break(99) == -1);
  return 0;
}
