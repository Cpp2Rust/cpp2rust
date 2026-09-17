#include <stdio.h>
int main() {
  int a = 1;
  int *p = &a;
  printf("%p %d", p, *p);
  return 0;
}
