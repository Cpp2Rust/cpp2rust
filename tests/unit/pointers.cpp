// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct Test {
  int x;

  void inc() { this->x++; }

  void dec() { this->x--; }

  int *as_ptr() { return &this->x; }

  void update(int x, int y) { this->x = x + y; }
};

Test *Update(Test *t) {
  int x = 1;
  int y = 2;
  ++x;
  t->update(x, y);
  x = (*t).x;
  y = t->x;
  (*t).update(x, y);
  return t;
}

int main() {
  Test t1 = {100};
  Test *t2 = Update(&t1);
  Test *t3 = nullptr;
  t3 = t2;
  t3->x = 15;
  *t3->as_ptr() += 10;
  assert(t3->x + t2->x + t1.x == 75);
  return 0;
}
