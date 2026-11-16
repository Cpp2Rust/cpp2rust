#include <cassert>
#include <map>
#include <memory>
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

int temporary_calls = 0;
struct Temporary {
  int n;
  int data[2];
  Temporary() : n(5), data{6, 7} { ++temporary_calls; }
};
struct Nested {
  Temporary inner;
};
Temporary make_temporary() { return Temporary(); }
Nested make_nested() { return Nested(); }
struct Owner {
  std::unique_ptr<Temporary> ptr;
  Owner() : ptr(std::make_unique<Temporary>()) {}
  const int &value() { return ptr->n; }
};
struct Noncopy {
  int n = 5;
  Noncopy() = default;
  Noncopy(const Noncopy &) = delete;
  Noncopy &operator=(const Noncopy &) = delete;
};
struct Held {
  const int *ptr;
  Held() : ptr(nullptr) {}
  Held(const int &n) : ptr(&n) {}
  Held(const int (&n)[2], int) : ptr(n) {}
};

std::map<int, int> global_values;
const int global_key = 0, conditional_key = 1, owned_key = 2;
int global_a = 0, global_b = 0;
bool global_choose = false;
auto global_temporary =
    global_values.try_emplace(global_key, make_temporary().n);
auto global_conditional = global_values.try_emplace(
    conditional_key, global_choose ? ++global_a : ++global_b);
auto global_owned =
    global_values.try_emplace(owned_key, std::make_unique<Temporary>()->n);

int main() {
  assert(global_temporary.second && global_temporary.first->second == 5);
  assert(global_owned.second && global_owned.first->second == 5);
  assert(global_conditional.second && global_conditional.first->second == 1 &&
         global_a == 0 && global_b == 1);
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

  // Argument effects can change which branch the lookup must take.
  std::map<int, int> changed;
  changed[key] = 10;
  int effects = 0;
  auto erased = changed.try_emplace(key, (changed.erase(key), ++effects));
  assert(effects == 1 && erased.second && erased.first->second == 1);
  auto updated = changed.try_emplace(key, (++changed[key], ++effects));
  assert(effects == 2 && !updated.second && updated.first->second == 2);

  std::map<int, int *> pointers;
  int *p = array;
  auto pointer = pointers.try_emplace(key, p += 1);
  assert(p == array + 1 && pointer.first->second == p);

  // Forwarding and reference casts must keep modified arguments addressable.
  for (int hit = 0; hit < 2; ++hit) {
    std::map<int, int> values;
    const int keys[] = {0, 1, 2, 3, 4, 5};
    if (hit) {
      for (int k : keys) {
        values[k] = 42;
      }
    }
    int moved = 0, forwarded = 0, xvalue = 0, lvalue = 0, assigned = 0;
    int nested = 0;
    auto r0 = values.try_emplace(keys[0], std::move(++moved));
    auto r1 = values.try_emplace(keys[1], std::forward<int &>(++forwarded));
    auto r2 = values.try_emplace(keys[2], static_cast<int &&>(++xvalue));
    auto r3 = values.try_emplace(keys[3], static_cast<int &>(++lvalue));
    auto r4 = values.try_emplace(keys[4], std::move(assigned = 5));
    auto r5 = values.try_emplace(
        keys[5],
        std::move(static_cast<int &>(std::forward<int &>(nested += 1))));
    assert(moved == 1 && forwarded == 1 && xvalue == 1 && lvalue == 1 &&
           assigned == 5 && nested == 1);
    assert(r0.second == !hit && r1.second == !hit && r2.second == !hit &&
           r3.second == !hit && r4.second == !hit && r5.second == !hit);
    for (int k : keys) {
      assert(values[k] == (hit ? 42 : k == 4 ? 5 : 1));
    }
  }

  for (int hit = 0; hit < 2; ++hit) {
    for (int choose = 0; choose < 2; ++choose) {
      std::map<int, Item> selected;
      const int keys[] = {0, 1, 2};
      if (hit) {
        for (int k : keys) {
          selected[k].value = 42;
        }
      }
      int a = 0, b = 0, c = 0, d = 0, e = 0, f = 0;
      auto prefix = selected.try_emplace(keys[0], choose ? ++a : ++b, 2);
      auto assignment =
          selected.try_emplace(keys[1], choose ? (c = 5) : (d = 6), 2);
      auto compound =
          selected.try_emplace(keys[2], choose ? (e += 1) : (f += 2), 2);
      assert(a == (choose ? (hit ? 1 : 3) : 0) &&
             b == (choose ? 0 : (hit ? 1 : 3)));
      assert(c == (choose ? (hit ? 5 : 7) : 0) &&
             d == (choose ? 0 : (hit ? 6 : 8)));
      assert(e == (choose ? (hit ? 1 : 3) : 0) &&
             f == (choose ? 0 : (hit ? 2 : 4)));
      assert(prefix.second == !hit && assignment.second == !hit &&
             compound.second == !hit);
      assert(prefix.first->second.value == (hit ? 42 : 3));
      assert(assignment.first->second.value == (hit ? 42 : choose ? 7 : 8));
      assert(compound.first->second.value == (hit ? 42 : choose ? 3 : 4));
    }

    std::map<int, int> values;
    const int keys[] = {0, 1, 2};
    if (hit) {
      for (int k : keys) {
        values[k] = 42;
      }
    }
    int before = temporary_calls, index = 0;
    auto member = values.try_emplace(keys[0], make_temporary().n);
    auto nested = values.try_emplace(keys[1], make_nested().inner.n);
    auto element = values.try_emplace(keys[2], make_temporary().data[index++]);
    assert(temporary_calls == before + 3 && index == 1);
    assert(member.second == !hit && nested.second == !hit &&
           element.second == !hit);
    assert(member.first->second == (hit ? 42 : 5) &&
           nested.first->second == (hit ? 42 : 5) &&
           element.first->second == (hit ? 42 : 6));

    for (int choose = 0; choose < 2; ++choose) {
      std::map<int, Held> references;
      int ordinary = 9;
      if (hit) {
        references.try_emplace(key, ordinary);
      }
      before = temporary_calls;
      index = 0;
      int value = 0;
      (references.try_emplace(key, choose ? make_nested().inner.n
                                          : make_temporary().data[index++]),
       value = *references[key].ptr);
      assert(temporary_calls == before + 1 && index == (choose ? 0 : 1));
      assert(value == (hit ? 9 : choose ? 5 : 6));

      std::map<int, Held> arrays;
      const int ordinary_array[2] = {9, 10};
      if (hit) {
        arrays.try_emplace(key, ordinary_array, 0);
      }
      before = temporary_calls;
      (arrays.try_emplace(
           key, choose ? make_nested().inner.data : make_temporary().data, 0),
       value = arrays[key].ptr[1]);
      assert(temporary_calls == before + 1 && value == (hit ? 10 : 7));
    }
  }

  for (int hit = 0; hit < 2; ++hit) {
    std::map<int, int> values;
    const int keys[] = {0, 1, 2, 3, 4, 5};
    if (hit) {
      for (int k : keys) {
        values[k] = 42;
      }
    }
    int before = temporary_calls, index = 0;
    auto arrow = values.try_emplace(keys[0], std::make_unique<Temporary>()->n);
    auto nested =
        values.try_emplace(keys[1], std::make_unique<Nested>()->inner.n);
    auto element = values.try_emplace(
        keys[2], std::make_unique<Temporary>()->data[index++]);
    auto get =
        values.try_emplace(keys[3], std::make_unique<Temporary>().get()->n);
    auto deref =
        values.try_emplace(keys[4], (*std::make_unique<Temporary>()).n);
    auto indirect = values.try_emplace(
        keys[5], (**std::make_unique<std::unique_ptr<Temporary>>(
                      std::make_unique<Temporary>()))
                     .n);
    assert(temporary_calls == before + 6 && index == 1);
    assert(arrow.second == !hit && nested.second == !hit &&
           element.second == !hit && get.second == !hit &&
           deref.second == !hit && indirect.second == !hit);
    for (int k : keys) {
      assert(values[k] == (hit ? 42 : k == 2 ? 6 : 5));
    }
    for (int choose = 0; choose < 2; ++choose) {
      std::map<int, Held> references;
      int ordinary = 9;
      if (hit) {
        references.try_emplace(key, ordinary);
      }
      before = temporary_calls;
      index = 0;
      int value = 0;
      (references.try_emplace(
           key, choose ? std::make_unique<Temporary>()->n
                       : std::make_unique<Temporary>()->data[index++]),
       value = *references[key].ptr);
      assert(temporary_calls == before + 1 && index == (choose ? 0 : 1));
      assert(value == (hit ? 9 : choose ? 5 : 6));

      std::map<int, Held> owners;
      (owners.try_emplace(key, Owner().value()), value = *owners[key].ptr);
      assert(value == 5);
    }
  }

  std::map<int, std::pair<std::unique_ptr<Noncopy>, int>> move_only;
  auto move_new = move_only.try_emplace(key, std::make_unique<Noncopy>(), 7);
  auto move_hit = move_only.try_emplace(key, std::make_unique<Noncopy>(), 9);
  assert(move_new.second && !move_hit.second);
  assert(move_new.first->second.first->n == 5 &&
         move_hit.first->second.second == 7);

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
