// panic
#include <cassert>

int while_in_switch_break(int n) {
  int r = 0;
  switch (n) {
  case 0: {
    int i = 0;
    while (i < 10) {
      if (i == 4)
        break;
      r += i;
      ++i;
    }
    r += 1000;
    break;
  }
  default:
    r = -1;
    break;
  }
  return r;
}

int main() {
  assert(while_in_switch_break(0) == 1006);
  assert(while_in_switch_break(99) == -1);
  return 0;
}
