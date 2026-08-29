#include <cassert>

int main() {
  int &&r = 5;
  goto body;
body:
  assert(r == 5);
  return 0;
}
