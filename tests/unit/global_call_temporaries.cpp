#include <algorithm>
#include <cassert>
#include <cstddef>

int read(const int &value) { return value; }

// The operands must survive until the returned reference has been consumed.
int selected = read(std::min(0, 0));

int x = 0, y = 0;
int assigned = (x = std::min(5, 6));
int accumulated = (y += std::max(1, 2));
int call_assigned = (x = read(5));

struct Item {
  int value;
};

int read_array(const int (&value)[1]) { return value[0]; }
int read_record(const Item &value) { return value.value; }

// Enclosing argument owners share the full expression, not the parameter name.
int aggregate_selected =
    read_array({std::min(3, 4)}) + read_record({std::max(5, 6)});

// These C++ types agree, but their Rust mappings require a cast temporary.
unsigned long left = 3;
std::size_t right = 7;
std::size_t mapped_selected = std::min<std::size_t>(left, right);

std::size_t get() {
  static std::size_t value = std::max<std::size_t>(left, right);
  return value;
}

int main() {
  assert(selected == 0);
  assert(assigned == 5);
  assert(accumulated == 2);
  assert(call_assigned == 5);
  assert(aggregate_selected == 9);
  static int aggregate_static =
      read_array({std::max(7, 8)}) + read_record({std::min(9, 10)});
  assert(aggregate_static == 17);
  assert(mapped_selected == 3);
  assert(get() == 7);
  return 0;
}
