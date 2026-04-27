// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>

struct node {
  struct node *next;
  union {
    uint8_t bytes[1];
    void *aligner;
  } x;
};

int main(void) {
  struct node n;
  n.next = 0;
  n.x.bytes[0] = 0xAB;

  assert(n.x.bytes[0] == 0xAB);
  assert(sizeof(n.x) >= sizeof(void *));
  assert(((uintptr_t)&n.x % _Alignof(void *)) == 0);
  return 0;
}
