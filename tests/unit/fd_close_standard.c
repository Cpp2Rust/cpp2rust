#include <assert.h>
#include <fcntl.h>
#include <unistd.h>

int main(void) {
  for (int fd = 0; fd < 3; ++fd) {
    assert(close(fd) == 0);
    int reopened = open("/dev/zero", O_RDONLY);
    assert(reopened == fd);
    unsigned char buf[3] = {1, 127, 255};
    assert(read(reopened, buf, sizeof(buf)) == sizeof(buf));
    assert(buf[0] == 0 && buf[1] == 0 && buf[2] == 0);
  }
  return 0;
}
