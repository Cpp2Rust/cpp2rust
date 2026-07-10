#include <assert.h>

typedef enum { MODE_IDLE, MODE_ACTIVE, MODE_DONE } widget;

struct widget {
  int id;
  widget mode;
};

typedef struct {
  int x;
  int y;
} point;

union point {
  int whole;
  short half;
};

typedef union {
  int i;
  unsigned u;
} slot;

enum slot { SLOT_A, SLOT_B };

struct Outer {
  struct Inner {
    int tag_field;
  } field;
};

typedef struct {
  int typedef_field;
} Inner;

static int is_active(struct widget *w) { return w->mode == MODE_ACTIVE; }

int main(void) {
  struct widget w;
  w.id = 7;
  w.mode = MODE_ACTIVE;
  assert(is_active(&w));
  w.mode = MODE_DONE;
  assert(w.mode == MODE_DONE);

  point p;
  p.x = 3;
  p.y = 4;
  assert(p.x + p.y == 7);

  union point up;
  up.whole = 5;
  assert(up.whole == 5);

  slot b;
  b.i = 9;
  assert(b.i == 9);

  enum slot e = SLOT_B;
  assert(e == SLOT_B);

  struct Inner inner_tag;
  inner_tag.tag_field = 11;
  assert(inner_tag.tag_field == 11);

  Inner inner_typedef;
  inner_typedef.typedef_field = 22;
  assert(inner_typedef.typedef_field == 22);

  struct Outer o;
  o.field.tag_field = 33;
  assert(o.field.tag_field == 33);

  return w.id;
}
