// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <termios.h>

int f1(int fd, int optional_actions, const struct termios *termios_p) {
  return tcsetattr(fd, optional_actions, termios_p);
}

int f2(int fd, struct termios *termios_p) { return tcgetattr(fd, termios_p); }
