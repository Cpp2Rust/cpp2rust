#include <cassert>
#include <map>
#include <string>
#include <utility>

int made = 0;
int visits = 0;

struct Item {
  int value;
  Item() : value(0) {}
  Item(int &counter, int amount) : value(counter += amount) { ++made; }
};

std::map<int, Item> &receiver(std::map<int, Item> &m) {
  ++visits;
  return m;
}

int &place(int &n) {
  ++visits;
  return n;
}

int main() {
  std::map<int, Item> m;
  int key = 0, counter = 0;
  m.try_emplace(key, counter, 3);
  made = 0;
  auto hit = receiver(m).try_emplace(key, ++place(counter), 2);
  assert(counter == 4 && made == 0 && visits == 2 && !hit.second);
  ++key;
  auto inserted = m.try_emplace(key, ++counter, 2);
  assert(counter == 7 && made == 1 && inserted.second);
  auto assigned = m.try_emplace(key, counter = 9, 2);
  assert(counter == 9 && made == 1 && !assigned.second);
  ++key;
  auto compound = m.try_emplace(key, counter += 4, 2);
  assert(counter == 15 && made == 2 && compound.second);
  assert(inserted.first->second.value == 7 &&
         compound.first->second.value == 15);
  int array[2] = {0, 0};
  int index = 0;
  ++key;
  m.try_emplace(key, ++array[index++], 3);
  assert(index == 1 && array[0] == 4 && made == 3);
  ++key;
  m.try_emplace(key, array[index++] = 5, 3);
  assert(index == 2 && array[1] == 8 && made == 4);
  m.try_emplace(key, counter = counter + 1, 3);
  assert(counter == 16 && made == 4);
  int a = 0, b = 0;
  bool choose = false;
  m.try_emplace(key, choose ? ++a : ++b, 3);
  assert(a == 0 && b == 1 && made == 4);
  choose = true;
  m.try_emplace(key, choose ? ++a : ++b, 3);
  assert(a == 1 && b == 1 && made == 4);
  m.try_emplace(key, (counter++, counter), 3);
  assert(counter == 17 && made == 4);

  // Keep the ordinary arithmetic conversions and wrapping of compound writes.
  std::map<int, unsigned> unsigned_map;
  unsigned n = ~0u;
  auto wrapped = unsigned_map.try_emplace(key, n += 1);
  assert(n == 0 && wrapped.first->second == 0);
  unsigned char bytes[2] = {255, 0};
  index = 0;
  ++key;
  auto narrowed = unsigned_map.try_emplace(key, bytes[index++] += 2);
  assert(index == 1 && bytes[0] == 1 && narrowed.first->second == 1);

  std::map<int, int *> pointers;
  int *p = array;
  auto pointer = pointers.try_emplace(key, p += 1);
  assert(p == array + 1 && pointer.first->second == p);

  std::map<int, std::string> strings;
  std::string text = "value";
  strings.try_emplace(key, "original");
  auto string_hit = strings.try_emplace(key, std::move((++visits, text)));
  assert(visits == 3 && !string_hit.second && text == "value");
  ++key;
  auto string_new = strings.try_emplace(key, std::move((++visits, text)));
  assert(visits == 4 && string_new.second &&
         string_new.first->second == "value");
  return 0;
}
