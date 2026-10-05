#include <cstdio>
#include <cstring>
#include <string>

// char is signed on x86, so it is translated as i8, not u8.

static int to_int(char c) { return c; }

static bool is_negative(const char *s) { return s[0] < 0; }

int main() {
  char c = (char)200;
  int widened = c;
  printf("%d %d\n", widened, to_int('\xff'));
  printf("%d\n", c < 0);

  char lit[] = "\xe9t\xe9";
  unsigned char ulit[] = "\xe9t\xe9";
  printf("%d %d\n", lit[0], ulit[0]);
  printf("%d\n", is_negative(lit));

  const char *p = "\x80";
  printf("%d %d\n", p[0], (unsigned char)p[0]);

  // C string functions compare bytes as unsigned char.
  printf("%d\n", strcmp("\x80", "a") > 0);

  std::string s = "\xfe!";
  printf("%d %d\n", s[0], s[1]);

  char sum = (char)(c + c);
  printf("%d\n", sum);
  return 0;
}
