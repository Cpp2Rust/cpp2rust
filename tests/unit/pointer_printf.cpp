#include <stdio.h>
int main() {
  int a = 1;
  int *p = &a;
  printf("%p %d", p, *p);
  void *q = &a;
  printf(" %p\n", q);
  return 0;
}
