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

int f6(int domain, int type, int protocol) {
  return socket(domain, type, protocol);
}

int f7(int sockfd, int level, int optname, const void *optval, socklen_t optlen) {
  return setsockopt(sockfd, level, optname, optval, optlen);
}

int f8(int sockfd, int level, int optname, void *optval, socklen_t *optlen) {
  return getsockopt(sockfd, level, optname, optval, optlen);
}

ssize_t f9(int sockfd, void *buf, size_t len, int flags) {
  return recv(sockfd, buf, len, flags);
}

ssize_t f10(int sockfd, const void *buf, size_t len, int flags) {
  return send(sockfd, buf, len, flags);
}

int f11(int domain, int type, int protocol, int sv[2]) {
  return socketpair(domain, type, protocol, sv);
}

int f12(int sockfd, struct sockaddr *addr, socklen_t *addrlen) {
  return getsockname(sockfd, addr, addrlen);
}

int f13(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
  return connect(sockfd, addr, addrlen);
}

int f14(int sockfd, struct sockaddr *addr, socklen_t *addrlen) {
  return getpeername(sockfd, addr, addrlen);
}

#ifdef __linux__
int f15(int sockfd, struct sockaddr *addr, socklen_t *addrlen, int flags) {
  return accept4(sockfd, addr, addrlen, flags);
}
#endif
