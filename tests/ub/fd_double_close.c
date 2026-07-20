// panic-ub: refcount
#include <fcntl.h>
#include <unistd.h>

int main(void) {
  int fd = open("/dev/null", O_RDONLY);
  close(fd);
  return close(fd) == -1 ? 0 : 1;
}
