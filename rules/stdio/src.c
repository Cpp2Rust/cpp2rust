// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#define _GNU_SOURCE
#include <stdio.h>
#endif

typedef FILE *CPP2RUST_TYPE_RULE(1);

FILE *CPP2RUST_EXPR_RULE(1)(const char *pathname, const char *mode) {
  return fopen(pathname, mode);
}

int CPP2RUST_EXPR_RULE(2)(FILE *stream) { return fclose(stream); }

long CPP2RUST_EXPR_RULE(3)(FILE *stream) { return ftell(stream); }

int CPP2RUST_EXPR_RULE(4)(FILE *stream, long offset, int whence) {
  return fseek(stream, offset, whence);
}

size_t CPP2RUST_EXPR_RULE(5)(void *ptr, size_t size, size_t nmemb,
                             FILE *stream) {
  return fread(ptr, size, nmemb, stream);
}

size_t CPP2RUST_EXPR_RULE(6)(const void *ptr, size_t size, size_t nmemb,
                             FILE *stream) {
  return fwrite(ptr, size, nmemb, stream);
}

int CPP2RUST_EXPR_RULE(7)(FILE *stream) { return fflush(stream); }

FILE *CPP2RUST_EXPR_RULE(8)() { return stdout; }

FILE *CPP2RUST_EXPR_RULE(9)() { return stderr; }

FILE *CPP2RUST_EXPR_RULE(10)() { return stdin; }

int CPP2RUST_EXPR_RULE(11)(int c, FILE *stream) { return fputc(c, stream); }

int CPP2RUST_EXPR_RULE(12)(const char *s, FILE *stream) {
  return fputs(s, stream);
}

int CPP2RUST_EXPR_RULE(13)(const char *s) { return puts(s); }

int CPP2RUST_EXPR_RULE(14)(FILE *stream) { return fileno(stream); }

int CPP2RUST_EXPR_RULE(15)(FILE *stream) { return ferror(stream); }

int CPP2RUST_EXPR_RULE(16)(FILE *stream) { return feof(stream); }

char *CPP2RUST_EXPR_RULE(17)(char *s, int n, FILE *stream) {
  return fgets(s, n, stream);
}

FILE *CPP2RUST_EXPR_RULE(18)(const char *pathname, const char *mode,
                             FILE *stream) {
  return freopen(pathname, mode, stream);
}

int CPP2RUST_EXPR_RULE(19)(FILE *stream, off_t offset, int whence) {
  return fseeko(stream, offset, whence);
}

FILE *CPP2RUST_EXPR_RULE(20)(int fd, const char *mode) {
  return fdopen(fd, mode);
}

int (*CPP2RUST_EXPR_RULE(21))(char *, size_t, const char *, ...) = snprintf;

int CPP2RUST_EXPR_RULE(22)(const char *a0, const char *a1) {
  return rename(a0, a1);
}

int CPP2RUST_EXPR_RULE(23)(FILE *stream) { return getc(stream); }

int CPP2RUST_EXPR_RULE(24)(FILE *stream, char *buf, int mode, size_t size) {
  return setvbuf(stream, buf, mode, size);
}

int CPP2RUST_EXPR_RULE(25)(void) { return SEEK_SET; }

int CPP2RUST_EXPR_RULE(26)(void) { return SEEK_CUR; }

int CPP2RUST_EXPR_RULE(27)(void) { return SEEK_END; }
