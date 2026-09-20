#include <cassert>

int main() {
  int x = 0;
  int a[3] = {0, 1, 2};
  while (x < 3)
    ++a[x++];
  int out = 0;
  while (x)
    out += a[--x];
  out++;
  int x2 = --out;
  ++out;
  int x3 = out--;
  assert(out++ + x2 + x3 == 19);
  int n = +x2;
  double d = +1.5;
  assert(n == x2);
  assert(d == 1.5);
  assert(+n + +n == 2 * x2);
  assert(+a[0] == a[0]);
  return 0;
}
