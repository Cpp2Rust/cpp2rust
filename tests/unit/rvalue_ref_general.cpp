#include <cassert>
#include <utility>

int main() {
  int i1 = 3;
  int i2 = std::move(i1);
  assert(i1 == 3);
  assert(i2 == 3);

  int &&i3 = 40;
  i3 += 2;
  assert(i3 == 42);

  int &&i4 = 2 + 3;
  assert(i4 == 5);

  const int &i5 = 40;
  int &i6 = i3;
  int &i7 = i4;
  assert(i6 == i3);
  assert(i7 == i4);

  int i8 = 3;
  int &&i9 = std::move(i8);
  assert(i9 == 3);

  int *p1 = &i1;
  int *p2 = &i3;
  int *p3 = &i6;
  assert(*p1 == i1);
  assert(*p2 == i3);
  assert(*p3 == i6);

  return 0;
}
