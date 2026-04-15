#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

typedef size_t (*read_fn)(void *, size_t, size_t, FILE *);

typedef size_t (*read_alternative_fn)(char *, size_t, size_t, void *);

int main() {
  read_fn rfn = fread;
  assert(rfn == fread);
  assert(rfn != nullptr);

  read_alternative_fn rfn2 = (read_alternative_fn)fread;
  assert(rfn == (read_fn)rfn2);

  return 0;
}
