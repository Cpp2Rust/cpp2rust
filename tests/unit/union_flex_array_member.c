// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

struct node {
  size_t len;
  union {
    uint8_t bytes[1];
    void *aligner;
  } x;
};

int main(void) {
  size_t tail_size = 32;
  struct node *n = (struct node *)malloc(sizeof(struct node) + tail_size);
  n->len = tail_size;

  for (size_t i = 0; i < tail_size; i++) {
    n->x.bytes[i] = (uint8_t)(i & 0xFF);
  }
  for (size_t i = 0; i < tail_size; i++) {
    assert(n->x.bytes[i] == (uint8_t)(i & 0xFF));
  }

  uint8_t *p = &n->x.bytes[10];
  assert(*p == 10);
  *p = 0xAA;
  assert(n->x.bytes[10] == 0xAA);

  free(n);
  return 0;
}
