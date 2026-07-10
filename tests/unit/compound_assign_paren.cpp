#include <assert.h>
#include <stdint.h>

#define SET_FLAG(s, f) ((s)->flags |= (f))
#define CLEAR_FLAG(s, f) ((s)->flags &= (uint8_t)~(f))
#define MARK_USED(bits, i) ((bits)[(i) / 8] |= (unsigned char)(1 << ((i) & 7)))

struct Item {
  uint8_t flags;
};

int main() {
  Item item = {0};
  Item *ptr = &item;
  SET_FLAG(ptr, 1 << 0);
  SET_FLAG(ptr, 1 << 1);
  assert(ptr->flags == 3);
  CLEAR_FLAG(ptr, 1 << 0);
  assert(ptr->flags == 2);

  unsigned char bits[4] = {0};
  MARK_USED(bits, 5);
  MARK_USED(bits, 13);
  assert(bits[0] == 32);
  assert(bits[1] == 32);
  assert(bits[2] == 0);

  if (ptr->flags != 0) {
    CLEAR_FLAG(ptr, 1 << 1);
  }
  assert(ptr->flags == 0);
  return 0;
}
