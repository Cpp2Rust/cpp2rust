#include <assert.h>

int main() {
  char empty_buf[256] = "";
  assert(empty_buf[0] == '\0');
  assert(empty_buf[255] == '\0');

  char prefix_buf[32] = "%";
  assert(prefix_buf[0] == '%');
  assert(prefix_buf[1] == '\0');
  assert(prefix_buf[31] == '\0');

  char short_buf[16] = "hi";
  assert(short_buf[0] == 'h');
  assert(short_buf[1] == 'i');
  assert(short_buf[2] == '\0');
  assert(short_buf[15] == '\0');

  char exact_buf[6] = "hello";
  assert(exact_buf[0] == 'h');
  assert(exact_buf[4] == 'o');
  assert(exact_buf[5] == '\0');

  assert(sizeof("hello") == 6);
  assert(sizeof("hello") - 1 == 5);
  assert(sizeof("") == 1);
  assert(sizeof("fifteen-bytes!!") - 1 == 15);
  return 0;
}
