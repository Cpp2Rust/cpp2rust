// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <stdio.h>

using t1 = FILE *;

FILE *f1(const char *pathname, const char *mode) {
  return fopen(pathname, mode);
}

int f2(FILE *stream) { return fclose(stream); }

long f3(FILE *stream) { return ftell(stream); }

int f4(FILE *stream, long offset, int whence) {
  return fseek(stream, offset, whence);
}

size_t f5(void *ptr, size_t size, size_t nmemb, FILE *stream) {
  return fread(ptr, size, nmemb, stream);
}

size_t f6(const void *ptr, size_t size, size_t nmemb, FILE *stream) {
  return fwrite(ptr, size, nmemb, stream);
}

int f7(FILE *stream) { return fflush(stream); }

FILE *f8() { return stdout; }

FILE *f9() { return stderr; }

FILE *f10() { return stdin; }

int f11(int c, FILE *stream) { return fputc(c, stream); }

int f12(const char *s, FILE *stream) { return fputs(s, stream); }

int f13(const char *s) { return puts(s); }

int f14(FILE *stream) { return fileno(stream); }

int f15(FILE *stream) { return ferror(stream); }

int f16(FILE *stream) { return feof(stream); }

char *f17(char *s, int n, FILE *stream) { return fgets(s, n, stream); }

FILE *f18(const char *pathname, const char *mode, FILE *stream) {
  return freopen(pathname, mode, stream);
}

int f19(FILE *stream, off_t offset, int whence) {
  return fseeko(stream, offset, whence);
}

FILE *f20(int fd, const char *mode) { return fdopen(fd, mode); }

int f22(const char *a0, const char *a1) {
  return rename(a0, a1);
}

int f23(FILE *stream) { return getc(stream); }

int f24(FILE *stream, char *buf, int mode, size_t size) {
  return setvbuf(stream, buf, mode, size);
}
