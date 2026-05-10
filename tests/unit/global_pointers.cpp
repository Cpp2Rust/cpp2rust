#include <cassert>
#include <cstddef>

struct Entry {
  const char *name;
  int *p;
};

static const Entry single_entry = {"alone", nullptr};

static const Entry entries[2] = {
    {"first", nullptr},
    {"second", nullptr},
};

char *arr_of_pointers[3] = {};

int main() {
  assert(single_entry.p == nullptr);
  for (int i = 0; i < 2; ++i) {
    assert(entries[i].p == nullptr);
    assert(arr_of_pointers[i] == nullptr);
  }
  return 0;
}
