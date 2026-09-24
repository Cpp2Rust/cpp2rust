// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// no-compile
#include <assert.h>

template <typename Cmp> struct Sorted {
  int items[4];
  int size = 0;
  Cmp cmp;
  void insert(int v) {
    int i = size;
    while (i > 0 && cmp(v, items[i - 1])) {
      items[i] = items[i - 1];
      i--;
    }
    items[i] = v;
    size++;
  }
};

using Less = decltype([](int a, int b) { return a < b; });

int main() {
  auto greater = [](int a, int b) { return a > b; };

  Sorted<decltype(greater)> desc;
  desc.insert(2);
  desc.insert(7);
  desc.insert(4);
  assert(desc.items[0] == 7);
  assert(desc.items[1] == 4);
  assert(desc.items[2] == 2);

  Sorted<Less> asc;
  asc.insert(2);
  asc.insert(7);
  asc.insert(4);
  assert(asc.items[0] == 2);
  assert(asc.items[1] == 4);
  assert(asc.items[2] == 7);

  decltype(greater) fresh;
  assert(fresh(3, 1));
  assert(!fresh(1, 3));

  decltype(greater) assigned;
  assigned = greater;
  assert(assigned(5, 4));

  Less less;
  assert(less(1, 3));

  return 0;
}
