#include <stdio.h>

void log(const char *file, int line, const char *func) {
  printf("%s %d %s\n", file, line, func);
}

int main() {
  printf("%s %d %s\n", __FILE__, __LINE__, __FUNCTION__);
  log(__FILE__, __LINE__, __FUNCTION__);
  return 0;
}
