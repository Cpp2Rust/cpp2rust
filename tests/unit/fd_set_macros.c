#include <assert.h>
#include <sys/select.h>

int main() {
  fd_set set;
  FD_ZERO(&set);
  assert(!FD_ISSET(3, &set));
  FD_SET(3, &set);
  assert(FD_ISSET(3, &set));
  FD_CLR(3, &set);
  assert(!FD_ISSET(3, &set));
  return 0;
}
