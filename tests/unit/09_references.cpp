#include <cassert>

int main() {
  int h = 15;

  int &h_ref1 = h;
  h_ref1 = 16;

  int *h_ptr = &h_ref1;
  int &h_ref2 = *h_ptr;
  h_ref2 = 17;

  assert(h_ref1 + h_ref2 == 34);

  int a = 1;
  int b = 2;
  int &r = a < b ? a : b;
  r = 10;
  assert(a == 10);

  const int &cr = a > b ? a : b;
  assert(cr == 10);

  const int x = 1;
  const int y = 2;
  const int &cx = x < y ? x : y;
  assert(cx == 1);

  const int *cp = a > b ? &a : &b;
  assert(*cp == 10);

  int *mp = a < b ? &a : &b;
  *mp = 20;
  assert(b == 20);
  return 0;
}
