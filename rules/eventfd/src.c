#define _GNU_SOURCE

#ifdef __linux__
#include <sys/eventfd.h>

int f1() {
  return EFD_CLOEXEC;
}

int f2() {
  return EFD_NONBLOCK;
}

int f3() {
  return EFD_SEMAPHORE;
}

int f4(unsigned int initval, int flags) {
  return eventfd(initval, flags);
}
#endif
