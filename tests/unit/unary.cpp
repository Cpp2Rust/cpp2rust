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
  return 0;
}
