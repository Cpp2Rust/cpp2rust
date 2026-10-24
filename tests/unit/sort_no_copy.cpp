#include <algorithm>
#include <cassert>
#include <map>
#include <vector>

static int copies = 0;

struct Tracked {
  int v;
  Tracked() : v(0) {}
  Tracked(int v) : v(v) {}
  Tracked(const Tracked &o) : v(o.v) { ++copies; }
  Tracked(Tracked &&o) noexcept : v(o.v) {}
  Tracked &operator=(const Tracked &o) {
    v = o.v;
    ++copies;
    return *this;
  }
  Tracked &operator=(Tracked &&o) noexcept {
    v = o.v;
    return *this;
  }
};

bool operator<(const Tracked &a, const Tracked &b) { return a.v < b.v; }

bool greater(const Tracked &a, const Tracked &b) { return a.v > b.v; }

struct Bag {
  std::vector<Tracked> items;
};

bool operator<(const Bag &a, const Bag &b) {
  return a.items.size() < b.items.size();
}

bool operator==(const Bag &a, const Bag &b) {
  return a.items.size() == b.items.size();
}

int main() {
  Tracked arr[5] = {Tracked(3), Tracked(1), Tracked(4), Tracked(1), Tracked(5)};
  std::sort(arr, arr + 5);
  assert(arr[0].v == 1 && arr[1].v == 1 && arr[2].v == 3 && arr[3].v == 4 &&
         arr[4].v == 5);
  assert(copies == 0);

  std::vector<Tracked> vec;
  vec.reserve(4);
  vec.emplace_back(2);
  vec.emplace_back(7);
  vec.emplace_back(1);
  vec.emplace_back(8);
  std::sort(vec.begin(), vec.end());
  assert(vec[0].v == 1 && vec[1].v == 2 && vec[2].v == 7 && vec[3].v == 8);
  assert(copies == 0);

  std::stable_sort(vec.begin(), vec.end(), greater);
  assert(vec[0].v == 8 && vec[3].v == 1);
  assert(copies == 0);

  std::stable_sort(
      vec.begin(), vec.end(),
      [](const Tracked &a, const Tracked &b) { return a.v < b.v; });
  assert(vec[0].v == 1 && vec[3].v == 8);
  assert(copies == 0);

  std::stable_sort(vec.begin() + 1, vec.end(), greater);
  assert(vec[0].v == 1 && vec[1].v == 8 && vec[3].v == 2);
  assert(copies == 0);

  Bag bags[3];
  bags[0].items.emplace_back(1);
  bags[0].items.emplace_back(2);
  bags[2].items.emplace_back(3);
  std::sort(bags, bags + 3);
  assert(bags[0].items.empty() && bags[1].items.size() == 1 &&
         bags[2].items.size() == 2);
  assert(copies == 0);

  std::vector<Bag> bag_vec(3);
  bag_vec[1].items.emplace_back(4);
  bag_vec[1].items.emplace_back(5);
  bag_vec[2].items.emplace_back(6);
  std::sort(bag_vec.begin(), bag_vec.end());
  assert(bag_vec[0].items.empty() && bag_vec[1].items.size() == 1 &&
         bag_vec[2].items[1].v == 5);
  assert(copies == 0);

  assert(std::find(bag_vec.begin(), bag_vec.end(), bags[1]) ==
         bag_vec.begin() + 1);
  assert(std::max_element(bag_vec.begin(), bag_vec.end()) ==
         bag_vec.begin() + 2);
  assert(&std::min(bags[0], bags[2]) == &bags[0]);
  assert(&std::max(bags[0], bags[2]) == &bags[2]);
  assert(copies == 0);

  std::map<Bag, int> bag_map;
  bag_map[std::move(bag_vec[2])] = 2;
  bag_map[std::move(bag_vec[0])] = 0;
  assert(bag_map.find(bags[2])->second == 2);
  assert(bag_map.find(bags[1]) == bag_map.end());
  assert(copies == 0);
  return copies;
}
