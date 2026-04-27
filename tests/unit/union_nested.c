// translation-fail
#include <assert.h>
#include <stdint.h>
#include <string.h>

struct header {
  uint16_t tag;
  char data[14];
};

struct inner {
  union {
    struct header h;
    char raw[128];
  } buffer;
};

struct Outer {
  int kind;
  int subtype;
  int proto;
  unsigned int len;
  union {
    struct header h;
    struct inner nested;
  } body;
};

int main(void) {
  struct Outer ex;
  memset(&ex, 0, sizeof(ex));

  ex.kind = 2;
  ex.subtype = 1;
  ex.proto = 6;
  ex.len = sizeof(struct header);
  ex.body.h.tag = 2;
  ex.body.h.data[0] = 'X';

  assert(ex.body.h.tag == 2);
  assert(ex.body.h.data[0] == 'X');

  assert(ex.body.nested.buffer.h.tag == 2);
  return 0;
}
