#include <assert.h>
#include <stdio.h>
#include <string.h>

const char *base_name(const char *path) {
  const char *slash = strrchr(path, '/');
  return slash ? slash + 1 : path;
}

void log(const char *file, int line, const char *func) {
  printf("%s %d %s\n", file, line, func);
}

int line() { return __builtin_LINE(); }

const char *function() { return __builtin_FUNCTION(); }

int main() {
  printf("%s %d %s\n", __FILE__, __LINE__, __FUNCTION__);
  log(__FILE__, __LINE__, __FUNCTION__);
  assert(__builtin_LINE() > 0);
  assert(base_name(__builtin_FILE())[0] != '\0');
  assert(line() > 0);
  printf("%s\n", function());
  return 0;
}
