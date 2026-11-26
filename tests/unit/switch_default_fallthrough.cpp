#include <cassert>

int default_chain(int x) {
  int r = 0;
  switch (x) {
  case 2:
    r = 2;
  default:
    r = r * 10 + 9;
  case 1:
    r = r * 10 + 1;
  }
  return r;
}

int default_break(int x) {
  int r = 0;
  switch (x) {
  default:
    r = 9;
    break;
  case 1:
    r = 1;
    break;
  case 2:
    r = 2;
  }
  return r;
}

int main() {
  assert(default_chain(99) == 91);
  assert(default_chain(2) == 291);
  assert(default_chain(1) == 1);
  assert(default_break(2) == 2);
  return 0;
}
