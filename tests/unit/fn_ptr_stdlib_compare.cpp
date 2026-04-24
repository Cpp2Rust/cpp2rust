#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef size_t (*fread_t)(void *, size_t, size_t, FILE *);

typedef size_t (*fread_alternative_t)(char *, size_t, size_t, void *);

size_t my_alternative_fread(char *p, size_t n, size_t m, void *f) { return 22; }

int main() {
  fread_t fn1 = fread;
  assert(fn1 == fread);
  assert(fn1 != nullptr);

  fread_alternative_t fn2 = (fread_alternative_t)fread;
  assert(fn1 == (fread_t)fn2);

  fread_t f3 = (fread_t)my_alternative_fread;
  assert((*f3)(nullptr, 0, 0, nullptr) == 22);

  FILE *stream = fopen("/dev/zero", "rb");
  assert(stream != nullptr);
  char buf[16];
  memset(buf, 'X', sizeof(buf));
  size_t n = (*fn1)(buf, 1, 10, stream);
  assert(n == 10);
  for (int i = 0; i < 10; ++i) {
    assert(buf[i] == 0);
  }
  for (int i = 10; i < 16; ++i) {
    assert(buf[i] == 'X');
  }
  fclose(stream);

  return 0;
}
