// translation-fail
#include <assert.h>
#include <stdint.h>

typedef enum {
  K_INT,
  K_UINT,
  K_STR,
  K_DBL,
  K_PTR,
} Kind;

struct Arg {
  Kind kind;
  union {
    const char *str;
    void *ptr;
    int64_t nums;
    uint64_t numu;
    double dnum;
  } val;
};

int main(void) {
  struct Arg a;
  a.kind = K_INT;
  a.val.nums = -7;
  assert(a.val.nums == -7);

  struct Arg b;
  b.kind = K_UINT;
  b.val.numu = 0xDEADBEEFu;
  assert(b.val.numu == 0xDEADBEEFu);

  struct Arg c;
  c.kind = K_STR;
  c.val.str = "hello";
  assert(c.val.str[0] == 'h');

  struct Arg d;
  d.kind = K_DBL;
  d.val.dnum = 1.5;
  assert(d.val.dnum == 1.5);

  int x = 0;
  struct Arg e;
  e.kind = K_PTR;
  e.val.ptr = &x;
  assert(e.val.ptr == &x);

  return 0;
}
