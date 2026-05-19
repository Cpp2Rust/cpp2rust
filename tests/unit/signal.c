// no-compile: refcount
#include <assert.h>
#include <errno.h>
#include <signal.h>
#include <stddef.h>

static void test_sigaction(void) {
  struct sigaction sa;
  assert(sigaction(SIGINT, NULL, &sa) == 0);
  assert(sigaction(SIGTERM, NULL, &sa) == 0);
  errno = 0;
  assert(sigaction(99999, NULL, &sa) == -1);
  assert(errno == EINVAL);
  errno = 0;
}

int main(void) {
  test_sigaction();
  return 0;
}
