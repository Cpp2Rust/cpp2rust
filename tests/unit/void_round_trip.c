#include <stdint.h>
#include <assert.h>

int main() {
  uint32_t a = 42;
  assert(*(int32_t*)(void*)&a == 42);
  return 0;
}
