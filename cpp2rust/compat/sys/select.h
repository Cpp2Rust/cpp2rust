// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <sys/select.h>

#undef FD_SET
#undef FD_CLR
#undef FD_ISSET
#undef FD_ZERO

void cpp2rust_fd_set(int fd, fd_set *set);
void cpp2rust_fd_clr(int fd, fd_set *set);
int cpp2rust_fd_isset(int fd, const fd_set *set);
void cpp2rust_fd_zero(fd_set *set);

#define FD_SET(fd, set) cpp2rust_fd_set(fd, set)
#define FD_CLR(fd, set) cpp2rust_fd_clr(fd, set)
#define FD_ISSET(fd, set) cpp2rust_fd_isset(fd, set)
#define FD_ZERO(set) cpp2rust_fd_zero(set)
