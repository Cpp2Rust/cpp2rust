#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef size_t (*fread_t)(void *, size_t, size_t, FILE *);

typedef size_t (*fread_alternative_t)(char *, size_t, size_t, void *);

size_t my_alternative_fread(char *p, size_t n, size_t m, void *f) { return 22; }

typedef size_t (*fwrite_t)(const void *, size_t, size_t, FILE *);

typedef size_t (*fwrite_alternative_t)(const char *, size_t, size_t, void *);

size_t my_alternative_fwrite(const char *p, size_t n, size_t m, void *f) {
  return 33;
}

#define CHECK_FREAD(call)                                                      \
  do {                                                                         \
    FILE *stream = fopen("/dev/zero", "rb");                                   \
    assert(stream != nullptr);                                                 \
    char buf[16];                                                              \
    memset(buf, 'X', sizeof(buf));                                             \
    size_t n = (call)(buf, 1, 10, stream);                                     \
    assert(n == 10);                                                           \
    for (int i = 0; i < 10; ++i) {                                             \
      assert(buf[i] == 0);                                                     \
    }                                                                          \
    for (int i = 10; i < 16; ++i) {                                            \
      assert(buf[i] == 'X');                                                   \
    }                                                                          \
    fclose(stream);                                                            \
  } while (0)

#define CHECK_FWRITE(call)                                                     \
  do {                                                                         \
    FILE *stream = fopen("/dev/null", "wb");                                   \
    assert(stream != nullptr);                                                 \
    char buf[10];                                                              \
    memset(buf, 'Y', sizeof(buf));                                             \
    size_t n = (call)(buf, 1, 10, stream);                                     \
    assert(n == 10);                                                           \
    fclose(stream);                                                            \
  } while (0)

int main() {
  fread_t fn1 = fread;
  assert(fn1 == fread);
  assert(fn1 != nullptr);

  fread_alternative_t fn2 = (fread_alternative_t)fread;
  assert(fn1 == (fread_t)fn2);

  fread_t f3 = (fread_t)my_alternative_fread;
  assert((*f3)(nullptr, 0, 0, nullptr) == 22);

  CHECK_FREAD(fread);
  CHECK_FREAD((*fn1));

  fwrite_t gn1 = fwrite;
  assert(gn1 == fwrite);
  assert(gn1 != nullptr);

  fwrite_alternative_t gn2 = (fwrite_alternative_t)fwrite;
  assert(gn1 == (fwrite_t)gn2);

  fwrite_t g3 = (fwrite_t)my_alternative_fwrite;
  assert((*g3)(nullptr, 0, 0, nullptr) == 33);

  CHECK_FWRITE(fwrite);
  CHECK_FWRITE((*gn1));

  return 0;
}
