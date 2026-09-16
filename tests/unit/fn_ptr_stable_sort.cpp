#include <algorithm>
#include <assert.h>
#include <vector>

struct Item {
  int key;
  int value;
};

static bool Compare(const Item &a, const Item &b) { return a.key < b.key; }

int main() {
  std::vector<Item> v;
  v.push_back({3, 30});
  v.push_back({1, 10});
  v.push_back({2, 20});

  std::stable_sort(v.begin(), v.end(), Compare);

  assert(v[0].key == 1);
  assert(v[1].key == 2);
  assert(v[2].key == 3);

  return 0;
}
