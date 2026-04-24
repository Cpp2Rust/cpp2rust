#include <cassert>

int no_default(int x) {
  int r = -1;
  switch (x) {
  case 7:
    r = 1;
    break;
  case 8:
    r = 2;
    break;
  }
  return r;
}

int main() {
  assert(no_default(7) == 1);
  assert(no_default(42) == -1);
  return 0;
}
