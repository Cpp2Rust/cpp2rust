#include <cassert>

int &&g = 5;

int main() {
  assert(g == 5);
  return 0;
}
