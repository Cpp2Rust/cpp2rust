#include <assert.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
  W_64,
  W_32,
  W_16,
} Width;

struct Sink {
  Width width;
  union {
    const char *text;
    void *handle;
    int64_t signed_n;
    double f;
  } out;
};

static void write_count(struct Sink *s, int64_t count) {
  switch (s->width) {
  case W_64:
    *(int64_t *)s->out.handle = count;
    break;
  case W_32:
    *(int32_t *)s->out.handle = (int32_t)count;
    break;
  case W_16:
    *(int16_t *)s->out.handle = (int16_t)count;
    break;
  }
}

int main(void) {
  int64_t buf64 = 0;
  int32_t buf32 = 0;
  int16_t buf16 = 0;

  struct Sink s;

  s.width = W_64;
  s.out.handle = &buf64;
  write_count(&s, 0x1122334455667788LL);
  assert(buf64 == 0x1122334455667788LL);

  s.width = W_32;
  s.out.handle = &buf32;
  write_count(&s, 0x12345678);
  assert(buf32 == 0x12345678);

  s.width = W_16;
  s.out.handle = &buf16;
  write_count(&s, 0x1234);
  assert(buf16 == 0x1234);

  return 0;
}
