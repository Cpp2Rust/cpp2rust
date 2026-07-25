#include <assert.h>
#include <fcntl.h>
#include <unistd.h>

int main(void) {
  const char *path = "cpp2rust_isatty_test.tmp";
  int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(isatty(fd) == 0);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
  return 0;
}
