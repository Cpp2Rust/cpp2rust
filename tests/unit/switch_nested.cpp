// no-compile
#include <cassert>

int nested(int a, int b) {
  int r = 0;
  switch (a) {
  case 1:
    switch (b) {
    case 10:
      r = 11;
      break;
    case 20:
      r = 12;
      break;
    default:
      r = 13;
      break;
    }
    r += 1;
    break;
  case 2:
    r = 2;
    break;
  default:
    r = -1;
    break;
  }
  return r;
}

int main() {
  assert(nested(1, 10) == 12);
  assert(nested(1, 99) == 14);
  assert(nested(2, 0) == 2);
  assert(nested(3, 3) == -1);
  return 0;
}
