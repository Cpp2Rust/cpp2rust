#include <cassert>
#include <cstdint>
#include <memory>
#include <vector>

// The receiver of data() is unique_ptr::operator->, whose type is the
// unique_ptr::pointer typedef.
static uint8_t *first(const std::unique_ptr<std::vector<uint8_t>> &u) {
  return u->data();
}

int main() {
  std::unique_ptr<std::vector<uint8_t>> v(new std::vector<uint8_t>(3));
  uint8_t *p = v->data();
  p[2] = 1;
  first(v)[0] = 5;
  assert(p[0] == 5);
  assert(first(v)[2] == 1);
  return 0;
}
