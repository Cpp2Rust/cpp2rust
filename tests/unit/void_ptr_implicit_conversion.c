#include <assert.h>
#include <stdint.h>

static int32_t bump(void *arg) {
  int32_t *value = arg;
  *value += 1;
  return *value;
}

int main() {
  int32_t value = 41;
  void *opaque = &value;
  int32_t *typed = opaque;
  assert(bump(opaque) == 42);
  assert(*typed == 42);
  *typed = 7;
  assert(value == 7);
  return 0;
}
