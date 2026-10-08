#include <stdio.h>

int main() {
  char c = 'a';
  int n = 3;
  printf("%c\n", c);
  printf("%d %c\n", n, c);
  printf("100%% %c\n", c);
  printf("%c%c%d\n", c, c + 1, n);
  return 0;
}
