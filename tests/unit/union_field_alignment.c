// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>

struct chunk {
  struct chunk *next;
  union {
    uint8_t data[1];
    void *dummy;
  } x;
};

int main(void) {
  struct chunk c;
  c.next = 0;
  c.x.data[0] = 0xAB;

  assert(c.x.data[0] == 0xAB);
  assert(sizeof(c.x) >= sizeof(void *));
  assert(((uintptr_t)&c.x % _Alignof(void *)) == 0);
  return 0;
}
