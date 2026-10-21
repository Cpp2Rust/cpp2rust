#include <cassert>
#include <cstdint>
#include <memory>
#include <vector>

static std::vector<uint8_t> g(3);

static std::vector<uint8_t> &get() { return g; }

// Elements of a vector reached through a call that returns a reference to it.
int main() {
  get()[1] = 4;
  std::unique_ptr<std::vector<uint8_t>> v(new std::vector<uint8_t>(3));
  (*v)[2] = 5;
  assert(get()[1] == 4);
  assert((*v)[2] == 5);
  return 0;
}
