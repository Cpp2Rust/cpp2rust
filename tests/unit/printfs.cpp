#include <cassert>
#include <cstdio>
#include <string>

std::string fn(std::string v) { return v + " str"; }

const std::string &fn2(const std::string &v) { return v; }

template <class... Args> void log_to(FILE *out, const char *fmt, Args... args) {
  fprintf(out, fmt, args...);
}

void log_fmt(const char *fmt, int v) { printf(fmt, v); }

int main() {
  fprintf(stdout, "%s\n", "fprintf stdout");
  fprintf(stdout, "%d %u %ld\n", 1, 2U, 3L);
  fprintf(stdout, "hello world");
  FILE *in = stdin;
  assert(in != NULL);
  printf("%s\n", "printf");
  printf("hello world");
  std::string s = "a string";
  printf("%s\n", s.data());
  printf("%s\n", fn("foo").c_str());
  printf("%s\n", fn2(s).c_str());
  log_to(stdout, "%s %d\n", "runtime", 4);
  log_to(stderr, "%d\n", 5);
  log_fmt("%d\n", 6);
  return 0;
}
