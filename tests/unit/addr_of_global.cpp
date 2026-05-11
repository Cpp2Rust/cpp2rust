#include <cassert>

struct Inner {
  int value;
};

struct Outer {
  Inner *p;
};

Inner alpha = {1};
Inner beta = {2};
Inner shared = {42};

Inner *items[2] = {&alpha, &beta};
Outer obj = {&shared};

int main() {
  assert(items[0]->value == 1);
  assert(items[1]->value == 2);
  assert(obj.p->value == 42);

  static Inner *cache[2] = {&alpha, &beta};
  assert(cache[0]->value == 1);
  assert(cache[1]->value == 2);
  return 0;
}
