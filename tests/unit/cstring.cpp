// no-compile: refcount
#include <cassert>
#include <cstring>

static void test_memcpy() {
  const char src[] = "hello";
  char dst[6] = {0};
  void *r = std::memcpy(dst, src, 6);
  assert(r == dst);
  assert(dst[0] == 'h' && dst[1] == 'e' && dst[2] == 'l');
  assert(dst[3] == 'l' && dst[4] == 'o' && dst[5] == '\0');
}

static void test_memset() {
  char buf[4];
  void *r = std::memset(buf, 'x', 4);
  assert(r == buf);
  assert(buf[0] == 'x' && buf[1] == 'x' && buf[2] == 'x' && buf[3] == 'x');
}

static void test_memcmp() {
  const char a[] = {1, 2, 3, 4};
  const char b[] = {1, 2, 3, 4};
  const char c[] = {1, 2, 9, 4};
  assert(std::memcmp(a, b, 4) == 0);
  assert(std::memcmp(a, c, 4) < 0);
  assert(std::memcmp(c, a, 4) > 0);
}

static void test_memmove() {
  char buf[6] = {'a', 'b', 'c', 'd', 'e', '\0'};
  void *r = std::memmove(buf + 1, buf, 4);
  assert(r == buf + 1);
  assert(buf[0] == 'a' && buf[1] == 'a' && buf[2] == 'b');
  assert(buf[3] == 'c' && buf[4] == 'd' && buf[5] == '\0');
}

static void test_strchr() {
  const char *s = "hello world";
  const char *r = std::strchr(s, 'w');
  assert(r != nullptr);
  assert(*r == 'w');
  assert(std::strchr(s, 'z') == nullptr);
}

static void test_strlen() {
  assert(std::strlen("") == 0);
  assert(std::strlen("hello") == 5);
  assert(std::strlen("hello world") == 11);
}

int main() {
  test_memcpy();
  test_memset();
  test_memcmp();
  test_memmove();
  test_strchr();
  test_strlen();
  return 0;
}
