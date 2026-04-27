// translation-fail
#include <assert.h>
#include <stdint.h>

typedef enum {
  T_NUM_S,
  T_NUM_U,
  T_TEXT,
  T_FLOAT,
  T_REF,
} Tag;

struct Slot {
  Tag tag;
  union {
    const char *text;
    void *handle;
    int64_t signed_n;
    uint64_t unsigned_n;
    double f;
  } payload;
};

int main(void) {
  struct Slot a;
  a.tag = T_NUM_S;
  a.payload.signed_n = -7;
  assert(a.payload.signed_n == -7);

  struct Slot b;
  b.tag = T_NUM_U;
  b.payload.unsigned_n = 0xDEADBEEFu;
  assert(b.payload.unsigned_n == 0xDEADBEEFu);

  struct Slot c;
  c.tag = T_TEXT;
  c.payload.text = "hello";
  assert(c.payload.text[0] == 'h');

  struct Slot d;
  d.tag = T_FLOAT;
  d.payload.f = 1.5;
  assert(d.payload.f == 1.5);

  int x = 0;
  struct Slot e;
  e.tag = T_REF;
  e.payload.handle = &x;
  assert(e.payload.handle == &x);

  return 0;
}
