#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

typedef size_t (*read_fn)(void *, size_t, size_t, FILE *);
typedef void (*free_fn)(void *);
typedef void *(*malloc_fn)(size_t);

int main() {
  read_fn rfn = fread;
  assert(rfn == fread);
  assert(rfn != nullptr);

  free_fn ffn = free;
  assert(ffn == free);
  assert(ffn != nullptr);

  malloc_fn mfn = malloc;
  assert(mfn == malloc);
  assert(mfn != nullptr);

  // Reassign and compare
  read_fn rfn2 = fread;
  assert(rfn == rfn2);

  free_fn ffn2 = nullptr;
  assert(ffn != ffn2);
  ffn2 = free;
  assert(ffn == ffn2);

  return 0;
}
