#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <unistd.h>

int main(void) {
  const char *sibling = "unlink-\xef\xbf\xbd.tmp";
  FILE *fp = fopen(sibling, "wb");
  assert(fp != NULL);
  assert(fputs("sentinel", fp) >= 0);
  assert(fclose(fp) == 0);

  errno = 0;
  assert(unlink("unlink-\xff.tmp") == -1);
  assert(errno == ENOENT);
  fp = fopen(sibling, "rb");
  assert(fp != NULL);
  assert(fclose(fp) == 0);

  assert(unlink(sibling) == 0);
  return 0;
}
