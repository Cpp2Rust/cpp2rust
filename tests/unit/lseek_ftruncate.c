#include <assert.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  const char *path = "cpp2rust_lseek_ftruncate_test.tmp";
  int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello world", 11) == 11);
  assert(lseek(fd, 0, SEEK_END) == 11);
  assert(lseek(fd, 6, SEEK_SET) == 6);
  char buf[16];
  memset(buf, 0, sizeof(buf));
  assert(read(fd, buf, sizeof(buf)) == 5);
  assert(strcmp(buf, "world") == 0);
  assert(ftruncate(fd, 5) == 0);
  assert(lseek(fd, 0, SEEK_END) == 5);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
  return 0;
}
