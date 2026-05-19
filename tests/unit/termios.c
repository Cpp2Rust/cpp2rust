// no-compile: refcount
#include <assert.h>
#include <errno.h>
#include <string.h>
#include <termios.h>

static void test_tcgetattr(void) {
  struct termios t;
  errno = 0;
  assert(tcgetattr(0, &t) == -1);
  assert(errno == ENOTTY);
  errno = 0;
}

static void test_tcsetattr(void) {
  struct termios t;
  memset(&t, 0, sizeof(t));
  errno = 0;
  assert(tcsetattr(-1, TCSANOW, &t) == -1);
  assert(errno == EBADF);
  errno = 0;
}

int main(void) {
  test_tcgetattr();
  test_tcsetattr();
  return 0;
}
