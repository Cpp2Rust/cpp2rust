#include <assert.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "ab", 2) == 2);
  char buf[4];
  memset(buf, 0, sizeof(buf));
  assert(read(fds[0], buf, sizeof(buf)) == 2);
  assert(strcmp(buf, "ab") == 0);
  assert(close(fds[1]) == 0);
  assert(read(fds[0], buf, sizeof(buf)) == 0);
  assert(close(fds[0]) == 0);
  return 0;
}
