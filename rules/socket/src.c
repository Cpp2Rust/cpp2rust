#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>
#endif

typedef struct sockaddr CPP2RUST_TYPE_RULE(1);
typedef struct sockaddr_storage CPP2RUST_TYPE_RULE(2);
typedef struct sockaddr_un CPP2RUST_TYPE_RULE(3);

int CPP2RUST_EXPR_RULE(1)() { return MSG_NOSIGNAL; }

int CPP2RUST_EXPR_RULE(2)() { return SOCK_STREAM; }

int CPP2RUST_EXPR_RULE(3)() { return SOCK_DGRAM; }

#ifdef __linux__
int CPP2RUST_EXPR_RULE(4)() { return SOCK_CLOEXEC; }

int CPP2RUST_EXPR_RULE(5)() { return SOCK_NONBLOCK; }
#endif

int CPP2RUST_EXPR_RULE(6)(int domain, int type, int protocol) {
  return socket(domain, type, protocol);
}

int CPP2RUST_EXPR_RULE(7)(int sockfd, int level, int optname,
                          const void *optval, socklen_t optlen) {
  return setsockopt(sockfd, level, optname, optval, optlen);
}

int CPP2RUST_EXPR_RULE(8)(int sockfd, int level, int optname, void *optval,
                          socklen_t *optlen) {
  return getsockopt(sockfd, level, optname, optval, optlen);
}

ssize_t CPP2RUST_EXPR_RULE(9)(int sockfd, void *buf, size_t len, int flags) {
  return recv(sockfd, buf, len, flags);
}

ssize_t CPP2RUST_EXPR_RULE(10)(int sockfd, const void *buf, size_t len,
                               int flags) {
  return send(sockfd, buf, len, flags);
}

int CPP2RUST_EXPR_RULE(11)(int domain, int type, int protocol, int sv[2]) {
  return socketpair(domain, type, protocol, sv);
}

int CPP2RUST_EXPR_RULE(12)(int sockfd, struct sockaddr *addr,
                           socklen_t *addrlen) {
  return getsockname(sockfd, addr, addrlen);
}

int CPP2RUST_EXPR_RULE(13)(int sockfd, const struct sockaddr *addr,
                           socklen_t addrlen) {
  return connect(sockfd, addr, addrlen);
}

int CPP2RUST_EXPR_RULE(14)(int sockfd, struct sockaddr *addr,
                           socklen_t *addrlen) {
  return getpeername(sockfd, addr, addrlen);
}

#ifdef __linux__
int CPP2RUST_EXPR_RULE(15)(int sockfd, struct sockaddr *addr,
                           socklen_t *addrlen, int flags) {
  return accept4(sockfd, addr, addrlen, flags);
}
#endif

int CPP2RUST_EXPR_RULE(16)(int sockfd, const struct sockaddr *addr,
                           socklen_t addrlen) {
  return bind(sockfd, addr, addrlen);
}

int CPP2RUST_EXPR_RULE(17)(int sockfd, int backlog) {
  return listen(sockfd, backlog);
}

ssize_t CPP2RUST_EXPR_RULE(18)(int sockfd, void *buf, size_t len, int flags,
                               struct sockaddr *src_addr, socklen_t *addrlen) {
  return recvfrom(sockfd, buf, len, flags, src_addr, addrlen);
}

ssize_t CPP2RUST_EXPR_RULE(19)(int sockfd, const void *buf, size_t len,
                               int flags, const struct sockaddr *dest_addr,
                               socklen_t addrlen) {
  return sendto(sockfd, buf, len, flags, dest_addr, addrlen);
}

int CPP2RUST_EXPR_RULE(20)() { return AF_INET; }

int CPP2RUST_EXPR_RULE(21)() { return AF_INET6; }

int CPP2RUST_EXPR_RULE(22)() { return SOL_SOCKET; }

int CPP2RUST_EXPR_RULE(23)() { return SO_KEEPALIVE; }

int CPP2RUST_EXPR_RULE(24)() { return SO_ERROR; }
