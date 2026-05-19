// no-compile: refcount
#include <assert.h>
#include <string.h>

static void test_memcpy(void) {
  const char src[] = "hello";
  char dst[6] = {0};
  void *r = memcpy(dst, src, 6);
  assert(r == dst);
  assert(dst[0] == 'h' && dst[1] == 'e' && dst[2] == 'l');
  assert(dst[3] == 'l' && dst[4] == 'o' && dst[5] == '\0');
}

static void test_memset(void) {
  char buf[4];
  void *r = memset(buf, 'x', 4);
  assert(r == buf);
  assert(buf[0] == 'x' && buf[1] == 'x' && buf[2] == 'x' && buf[3] == 'x');
}

static void test_memcmp(void) {
  const char a[] = {1, 2, 3, 4};
  const char b[] = {1, 2, 3, 4};
  const char c[] = {1, 2, 9, 4};
  assert(memcmp(a, b, 4) == 0);
  assert(memcmp(a, c, 4) < 0);
  assert(memcmp(c, a, 4) > 0);
}

static void test_memmove(void) {
  char buf[6] = {'a', 'b', 'c', 'd', 'e', '\0'};
  void *r = memmove(buf + 1, buf, 4);
  assert(r == buf + 1);
  assert(buf[0] == 'a' && buf[1] == 'a' && buf[2] == 'b');
  assert(buf[3] == 'c' && buf[4] == 'd' && buf[5] == '\0');
}

static void test_strchr(void) {
  const char *s = "hello world";
  char *r = strchr((char *)s, 'w');
  assert(r != NULL);
  assert(*r == 'w');
  assert(strchr((char *)s, 'z') == NULL);
}

static void test_strlen(void) {
  assert(strlen("") == 0);
  assert(strlen("hello") == 5);
  assert(strlen("hello world") == 11);
}

int main(void) {
  test_memcpy();
  test_memset();
  test_memcmp();
  test_memmove();
  test_strchr();
  test_strlen();
  return 0;
}
