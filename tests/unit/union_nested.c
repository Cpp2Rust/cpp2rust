// no-compile: refcount
#include <assert.h>
#include <stdint.h>
#include <string.h>

struct record {
  uint16_t code;
  char pad[14];
};

struct inner {
  union {
    struct record h;
    char raw[128];
  } view;
};

struct Outer {
  int kind;
  int level;
  int variant;
  unsigned int len;
  union {
    struct record h;
    struct inner nested;
  } body;
};

int main(void) {
  struct Outer ex;
  memset(&ex, 0, sizeof(ex));

  ex.kind = 2;
  ex.level = 1;
  ex.variant = 6;
  ex.len = sizeof(struct record);
  ex.body.h.code = 2;
  ex.body.h.pad[0] = 'X';

  assert(ex.body.h.code == 2);
  assert(ex.body.h.pad[0] == 'X');

  assert(ex.body.nested.view.h.code == 2);
  return 0;
}
