#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#endif

#ifdef __linux__
#ifdef CPP2RUST_USE_INCLUDES
#include <sys/eventfd.h>
#endif

int CPP2RUST_EXPR_RULE(1)() { return EFD_CLOEXEC; }

int CPP2RUST_EXPR_RULE(2)() { return EFD_NONBLOCK; }

int CPP2RUST_EXPR_RULE(3)() { return EFD_SEMAPHORE; }

int CPP2RUST_EXPR_RULE(4)(unsigned int initval, int flags) {
  return eventfd(initval, flags);
}
#endif
