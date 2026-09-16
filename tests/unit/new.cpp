#include <cassert>

int main() {
  int *x = new int(5);
  int out = *x;
  delete x;
  assert(out == 5);
  return 0;
}
