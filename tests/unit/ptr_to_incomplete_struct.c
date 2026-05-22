#include <assert.h>
#include <stdio.h>

int main(void) {
  FILE *fp = stdout;
  void *p = (void *)fp;
  FILE *fp2 = (FILE *)p;
  assert(fp == fp2);
  return 0;
}
