#include <cassert>

int main() {
  const int &r = 5;
  goto body;
body:
  assert(r == 5);
  return 0;
}
