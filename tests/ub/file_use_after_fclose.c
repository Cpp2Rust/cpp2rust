// panic-ub: refcount
// nondet-result: unsafe
#include <assert.h>
#include <stdio.h>

int main(void) {
  FILE *fp = fopen("/tmp/cpp2rust_uafc_test.tmp", "wb");
  assert(fp);
  fclose(fp);
  return fputc('x', fp) == 'x' ? 1 : 0;
}
