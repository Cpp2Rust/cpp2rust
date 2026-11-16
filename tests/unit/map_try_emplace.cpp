#include <cstdio>
#include <map>
#include <utility>

int calls = 0;
int made = 0;
int copies = 0;
int moves = 0;

struct Item {
  int value;
  Item() : value(0) {}
  Item(int value) : value(value) { ++made; }
  Item(const Item &other) : value(other.value) { ++copies; }
  Item(Item &&other) : value(other.value) {
    ++moves;
    other.value = -1;
  }
};

Item &argument(Item &item) {
  ++calls;
  return item;
}

struct RefItem {
  int value;
  RefItem() : value(0) {}
  RefItem(int &counter, int amount) : value(counter += amount) { ++made; }
};

int &reference(int &counter) {
  ++calls;
  return counter;
}

std::map<int, int> global_map;
int global_key = 0;
int global_counter = 0;
int seed = (global_map[global_key] = 10, 0);
auto global_hit = global_map.try_emplace(global_key, ++global_counter);

int main() {
  bool ok = global_counter == 1 && !global_hit.second &&
            global_hit.first->second == 10;
  int rows = 0;
  for (int subset = 0; subset < 8; ++subset) {
    for (int key = 0; key < 3; ++key) {
      for (int initial = 0; initial < 3; ++initial) {
        for (int post = 0; post < 2; ++post) {
          std::map<int, int> m;
          int size = 0;
          for (int k = 0; k < 3; ++k) {
            if (subset & (1 << k)) {
              m[k] = 10 + k;
              ++size;
            }
          }
          int counter = initial;
          auto result = post ? m.try_emplace(key, counter++)
                             : m.try_emplace(key, ++counter);
          bool hit = (subset & (1 << key)) != 0;
          ok &= counter == initial + 1;
          ok &= result.second == !hit;
          bool found = result.first == m.find(key);
          ok &= found;
          ok &= result.first->first == key;
          ok &= result.first->second == (hit ? 10 + key : initial + !post);
          ok &= m.size() == size + !hit;
          ++rows;
        }
      }
    }
  }

  std::map<int, Item> items;
  int key = 0;
  items.try_emplace(key, 10);
  Item source(20);
  made = copies = moves = 0;
  auto hit = items.try_emplace(key, argument(source));
  ok &= calls == 1 && made == 0 && copies == 0 && moves == 0;
  ok &= !hit.second && hit.first->second.value == 10;
  auto move_hit = items.try_emplace(key, std::move(argument(source)));
  ok &= calls == 2 && made == 0 && copies == 0 && moves == 0;
  ok &= !move_hit.second && source.value == 20;
  ++key;
  auto copied = items.try_emplace(key, argument(source));
  ok &= calls == 3 && made == 0 && copies == 1 && moves == 0;
  ok &= copied.second && copied.first->second.value == 20;
  ++key;
  auto moved = items.try_emplace(key, std::move(argument(source)));
  ok &= calls == 4 && made == 0 && copies == 1 && moves == 1;
  ok &= moved.second && moved.first->second.value == 20 && source.value == -1;

  std::map<int, RefItem> refs;
  key = 0;
  int counter = 0;
  refs.try_emplace(key, counter, 5);
  made = 0;
  auto ref_hit = refs.try_emplace(key, reference(counter), 2);
  ok &= calls == 5 && counter == 5 && made == 0 && !ref_hit.second;
  ++key;
  auto ref_new = refs.try_emplace(key, reference(counter), 2);
  ok &= calls == 6 && counter == 7 && made == 1 && ref_new.second;
  ok &= ref_hit.first->second.value == 5 && ref_new.first->second.value == 7;

  // A temporary argument is constructed on a hit, but is not moved from.
  key = 0;
  made = copies = moves = 0;
  auto temporary_hit = items.try_emplace(key, Item(30));
  ok &= made == 1 && copies == 0 && moves == 0 && !temporary_hit.second;
  ++key;
  ++key;
  ++key;
  auto temporary_new = items.try_emplace(key, Item(40));
  ok &= made == 2 && copies == 0 && moves == 1 && temporary_new.second;
  ok &= temporary_new.first->second.value == 40;

  printf("%d %d %d %d %d\n", rows, calls, made, copies, moves);
  return ok ? 0 : 1;
}
