#include <assert.h>
#include <sys/socket.h>
#include <unistd.h>

int main(void) {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  assert(s >= 0);
  assert(close(s) == 0);
  return 0;
}
