#include <cassert>

int switch_in_dowhile(int n) {
  int r = 0;
  int i = 0;
  do {
    switch (i) {
    case 0:
      r += 1;
      break;
    case 1:
      r += 10;
      break;
    default:
      r += 100;
      break;
    }
    ++i;
  } while (i < n);
  return r;
}

int main() {
  assert(switch_in_dowhile(1) == 1);
  assert(switch_in_dowhile(3) == 1 + 10 + 100);
  return 0;
}
