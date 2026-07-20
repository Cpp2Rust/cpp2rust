// panic-ub: refcount
#include <fcntl.h>
#include <unistd.h>

int main(void) {
  int fd = open("/dev/null", O_RDONLY);
  close(fd);
  char buf[4];
  ssize_t n = read(fd, buf, sizeof(buf));
  return n == -1 ? 0 : 1;
}
