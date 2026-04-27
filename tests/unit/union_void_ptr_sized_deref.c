// translation-fail
#include <assert.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
  W_INT64,
  W_INT32,
  W_INT16,
} IntWidth;

struct Arg {
  IntWidth width;
  union {
    const char *str;
    void *ptr;
    int64_t nums;
    double dnum;
  } val;
};

static void store_count(struct Arg *in, int64_t count) {
  switch (in->width) {
  case W_INT64:
    *(int64_t *)in->val.ptr = count;
    break;
  case W_INT32:
    *(int32_t *)in->val.ptr = (int32_t)count;
    break;
  case W_INT16:
    *(int16_t *)in->val.ptr = (int16_t)count;
    break;
  }
}

int main(void) {
  int64_t buf64 = 0;
  int32_t buf32 = 0;
  int16_t buf16 = 0;

  struct Arg in;

  in.width = W_INT64;
  in.val.ptr = &buf64;
  store_count(&in, 0x1122334455667788LL);
  assert(buf64 == 0x1122334455667788LL);

  in.width = W_INT32;
  in.val.ptr = &buf32;
  store_count(&in, 0x12345678);
  assert(buf32 == 0x12345678);

  in.width = W_INT16;
  in.val.ptr = &buf16;
  store_count(&in, 0x1234);
  assert(buf16 == 0x1234);

  return 0;
}
