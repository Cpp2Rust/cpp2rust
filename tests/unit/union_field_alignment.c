// no-compile: refcount
#include <assert.h>
#include <stddef.h>
#include <stdint.h>

// Just check if this compiles. node::x::aligner is used to impose a specific
// alignment on the bytes field.
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
  return 0;
}
