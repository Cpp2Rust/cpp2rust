// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <termios.h>
#include <sys/ioctl.h>
#endif

typedef struct termios CPP2RUST_TYPE_RULE(1);
typedef struct winsize CPP2RUST_TYPE_RULE(2);

int CPP2RUST_EXPR_RULE(1)(int fd, int optional_actions,
                          const struct termios *termios_p) {
  return tcsetattr(fd, optional_actions, termios_p);
}

int CPP2RUST_EXPR_RULE(2)(int fd, struct termios *termios_p) {
  return tcgetattr(fd, termios_p);
}
