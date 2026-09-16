#include <cassert>

int main() {
  int x = 1;
  int &r = x;
  r = 5;
  assert(x == 5);
  return 0;
}
