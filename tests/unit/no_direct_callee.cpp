#include <cassert>

bool test1() { return false; }

int test(decltype(test1) fn) {
  if (!fn()) {
    return 1;
  }
  return 0;
}

int main() {
  assert(test(test1) == 1);
  return 0;
}
