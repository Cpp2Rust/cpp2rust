#include <stdio.h>

void log(const char *file, int line, const char *func) {
  printf("%s %d %s\n", file, line, func);
}

int line() { return __builtin_LINE(); }

const char *function() { return __builtin_FUNCTION(); }

int main() {
  printf("%s %d %s\n", __FILE__, __LINE__, __FUNCTION__);
  log(__FILE__, __LINE__, __FUNCTION__);
  log(__builtin_FILE(), __builtin_LINE(), __builtin_FUNCTION());
  printf("%d %s\n", line(), function());
  return 0;
}
