#include <assert.h>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>

int main(void) {
  const char *path = "/tmp/cpp2rust_termios_test.tmp";
  int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  struct termios tio;
  assert(tcgetattr(fd, &tio) == -1);
  assert(tcsetattr(fd, TCSANOW, &tio) == -1);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
  return 0;
}
