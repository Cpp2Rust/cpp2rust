// C string functions stop at the first NUL.
#include <cstdio>
#include <cstring>
#include <string>

int main() {
  char buf[] = {'a', 'b', '\0', 'c', 'd', '\0'};
  fputs(buf, stdout);
  fputc('|', stdout);
  puts(buf);
  std::string s(buf);
  printf("%zu\n", s.size());
  printf("%zu %zu\n", strlen(buf), s.find_last_of(buf));
  const char *lit = "xy\0zw";
  std::string t(lit);
  printf("%zu\n", t.size());
  return 0;
}
