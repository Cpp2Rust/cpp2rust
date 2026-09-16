// panic: refcount
#include <cassert>
#include <cstdint>

struct Point {
  uint32_t x;
  uint32_t y;
};

struct Pair {
  uint32_t first;
  uint32_t second;
};

// Reinterpret between two structs with identical layout.
// Fails because custom structs don't implement ByteRepr (yet!).
int main() {
  Point pt = {10, 20};
  Pair *pair = reinterpret_cast<Pair *>(&pt);

  assert(pair->first == 10);
  assert(pair->second == 20);

  pair->first = 42;
  assert(pt.x == 42);

  return 0;
}
