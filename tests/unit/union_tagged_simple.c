#include <assert.h>

typedef enum {
  KIND_NONE,
  KIND_DONE,
} Kind;

struct Event {
  Kind kind;
  void *handle;
  union {
    void *obj;
    int code;
  } payload;
};

int main(void) {
  int dummy = 0;

  struct Event m1;
  m1.kind = KIND_DONE;
  m1.handle = &dummy;
  m1.payload.code = 42;
  assert(m1.kind == KIND_DONE);
  assert(m1.payload.code == 42);

  struct Event m2;
  m2.kind = KIND_NONE;
  m2.handle = &dummy;
  m2.payload.obj = &dummy;
  assert(m2.payload.obj == &dummy);

  return 0;
}
