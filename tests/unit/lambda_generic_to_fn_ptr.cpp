// translation-fail
#include <assert.h>

int main() {
  auto negate = [](auto x) { return -x; };
  int (*fi)(int) = negate;
  double (*fd)(double) = negate;
  assert(fi(3) == -3);
  assert(fd(1.5) == -1.5);
  auto square = []<typename T>(T x) { return x * x; };
  int (*si)(int) = square;
  double (*sd)(double) = square;
  assert(si(3) == 9);
  assert(sd(1.5) == 2.25);
  return 0;
}
