// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

struct chunk {
  size_t dlen;
  union {
    uint8_t data[1];
    void *dummy;
  } x;
};

int main(void) {
  size_t chunk_size = 32;
  struct chunk *c = (struct chunk *)malloc(sizeof(struct chunk) + chunk_size);
  c->dlen = chunk_size;

  for (size_t i = 0; i < chunk_size; i++) {
    c->x.data[i] = (uint8_t)(i & 0xFF);
  }
  for (size_t i = 0; i < chunk_size; i++) {
    assert(c->x.data[i] == (uint8_t)(i & 0xFF));
  }

  uint8_t *p = &c->x.data[10];
  assert(*p == 10);
  *p = 0xAA;
  assert(c->x.data[10] == 0xAA);

  free(c);
  return 0;
}
