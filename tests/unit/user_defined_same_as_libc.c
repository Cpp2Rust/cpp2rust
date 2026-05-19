#include <assert.h>
#include <stdio.h>

FILE *fopen(const char *path, const char *mode) {
  (void)path;
  (void)mode;
  return NULL;
}

int main() {
  FILE *fp = fopen("/tmp/irrelevant-file", "r");
  assert(fp == NULL);
  return 0;
}
