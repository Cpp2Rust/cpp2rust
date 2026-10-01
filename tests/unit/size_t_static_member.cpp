#include <cassert>
#include <cstddef>

struct S {
  static const size_t c = 5;
};

const size_t S::c;

template <class T> struct Table;

template <> struct Table<char> {
  static const size_t table_size = 256;
};

const size_t Table<char>::table_size;

int main() {
  const size_t *pa = &S::c;
  assert(*pa + 1 == 6);

  const std::size_t *G = &Table<char>::table_size;
  assert(*G >= 256);
  return 0;
}
