#include <sys/types.h>
#include <sys/socket.h>

int f1() {
  return MSG_NOSIGNAL;
}

int f2() {
  return SOCK_STREAM;
}

int f3() {
  return SOCK_DGRAM;
}

#ifdef __linux__
int f4() {
  return SOCK_CLOEXEC;
}

int f5() {
  return SOCK_NONBLOCK;
}
#endif
