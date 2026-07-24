#include <assert.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

int main(void) {
  const char *path = "cpp2rust_fstat_test.tmp";
  int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello", 5) == 5);
  struct stat st;
  assert(fstat(fd, &st) == 0);
  assert(st.st_size == 5);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
  return 0;
}
