#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
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

  // The other functions on paths take the bytes too, not their lossy
  // conversion, which is the sibling. File systems that only take UTF-8
  // names, like APFS, reject the name instead.
  fp = fopen("unlink-\xff.tmp", "wb");
  struct stat st;
  if (fp != NULL) {
    assert(fclose(fp) == 0);
    assert(stat("unlink-\xff.tmp", &st) == 0 && st.st_size == 0);
    assert(rename("unlink-\xff.tmp", "unlink-\xfe.tmp") == 0);
    int fd = open("unlink-\xfe.tmp", O_RDONLY);
    assert(fd >= 0);
    assert(close(fd) == 0);
    char *path = realpath("unlink-\xfe.tmp", NULL);
    assert(path != NULL);
    assert(strcmp(strrchr(path, '/'), "/unlink-\xfe.tmp") == 0);
    free(path);
    assert(unlink("unlink-\xfe.tmp") == 0);
  }
  assert(stat(sibling, &st) == 0 && st.st_size == 8);

  assert(unlink(sibling) == 0);
  return 0;
}
