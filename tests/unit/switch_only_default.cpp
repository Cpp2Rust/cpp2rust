#include <cassert>

int only_default(int x) {
  int r = 0;
  switch (x) {
  default:
    r = 42;
    break;
  }
  return r;
}

int main() {
  assert(only_default(1) == 42);
  return 0;
}
