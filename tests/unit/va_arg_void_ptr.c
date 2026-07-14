#include <assert.h>
#include <stdarg.h>

struct registry {
  void *slot;
  long level;
};

enum field {
  FIELD_SLOT,
  FIELD_LEVEL,
};

static int registry_update(struct registry *r, enum field field, ...) {
  int result = 0;
  va_list ap;
  va_start(ap, field);
  switch (field) {
  case FIELD_SLOT:
    r->slot = va_arg(ap, void *);
    break;
  case FIELD_LEVEL:
    r->level = va_arg(ap, long);
    break;
  default:
    result = 1;
    break;
  }
  va_end(ap);
  return result;
}

int main() {
  struct registry r = {0, 0};
  int payload = 7;
  assert(registry_update(&r, FIELD_SLOT, &payload) == 0);
  assert(registry_update(&r, FIELD_LEVEL, 5L) == 0);
  assert(r.slot == (void *)&payload);
  assert(*(int *)r.slot == 7);
  assert(r.level == 5);
  return 0;
}
