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
