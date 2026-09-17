#include <cassert>

static int total = 0;

struct S {
  S(int x) { total += x; }
};

static S a(1);
static S b(10);

int main() {
  assert(total == 11);
  return 0;
}
