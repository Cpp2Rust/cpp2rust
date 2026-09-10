// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

struct Inner {
  int x;
};

struct Table {
  static int table[3];
  static int &operator[](int i) { return table[i]; }
};
int Table::table[3] = {7, 8, 9};

struct S {
  int data[3];
  Inner inner;
  int &operator[](int i) { return data[i]; }
  const int &operator[](int i) const { return data[i]; }
  Inner &operator*() { return inner; }
  Inner *operator->() { return &inner; }
  int *operator&() { return &data[0]; }
};

int main() {
  S s{{1, 2, 3}, {9}};
  assert(s[1] == 2);
  s[1] = 20;
  assert(s[1] == 20);
  const S &cs = s;
  assert(cs[2] == 3);
  assert((*s).x == 9);
  (*s).x = 10;
  assert(s->x == 10);
  s->x = 11;
  assert(s.inner.x == 11);
  int *p = &s;
  assert(*p == 1);
  *p = 5;
  assert(s.data[0] == 5);
  Table t;
  assert(t[1] == 8);
  t[1] = 80;
  assert(Table::table[1] == 80);
  return 0;
}
