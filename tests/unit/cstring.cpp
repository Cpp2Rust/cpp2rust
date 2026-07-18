#define _GNU_SOURCE
#include <cassert>
#include <cstdlib>
#include <cstring>
#include <strings.h>

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

static void test_strcmp() {
  assert(std::strcmp("abc", "abc") == 0);
  assert(std::strcmp("abc", "abd") < 0);
  assert(std::strcmp("abd", "abc") > 0);
  const char *p = "abc";
  const char *q = "abd";
  char buf[] = {'a', 'b', 'c', '\0'};
  assert(std::strcmp(p, p) == 0);
  assert(std::strcmp(p, q) < 0);
  assert(std::strcmp(buf, p) == 0);
}

static void test_strncmp() {
  assert(std::strncmp("abcdef", "abcxyz", 3) == 0);
  assert(std::strncmp("abcdef", "abcxyz", 4) < 0);
  assert(std::strncmp("abcxyz", "abcdef", 4) > 0);
  const char *p = "abcdef";
  const char *q = "abcxyz";
  char buf[] = {'a', 'b', 'c', 'd', 'e', 'f', '\0'};
  std::size_t n = 3;
  assert(std::strncmp(p, q, n) == 0);
  assert(std::strncmp(p, q, n + 1) < 0);
  assert(std::strncmp(buf, p, 6) == 0);
}

static void test_memchr() {
  const char data[] = {0x10, 0x20, 0x30, 0x40};
  const void *r = std::memchr(data, 0x30, 4);
  assert(r == &data[2]);
  assert(std::memchr(data, 0x99, 4) == nullptr);
  const void *p = data;
  std::size_t n = 4;
  assert(std::memchr(p, 0x10, n) == p);
}

static void test_strrchr() {
  const char *s = "hello world";
  const char *r = std::strrchr(s, 'l');
  assert(r != nullptr);
  assert(*r == 'l');
  assert(r == s + 9);
  assert(std::strrchr(s, 'z') == nullptr);
  char buf[] = {'a', 'b', 'a', '\0'};
  assert(std::strrchr(buf, 'a') == &buf[2]);
}

static void test_strdup() {
  char *d = strdup("hello");
  assert(d != nullptr);
  assert(std::strcmp(d, "hello") == 0);
  std::free(d);
  const char *p = "world";
  char buf[] = {'a', 'b', 'c', '\0'};
  char *d2 = strdup(p);
  assert(d2 != nullptr);
  assert(std::strcmp(d2, p) == 0);
  std::free(d2);
  char *d3 = strdup(buf);
  assert(d3 != nullptr);
  assert(std::strcmp(d3, buf) == 0);
  std::free(d3);
}

static void test_strcspn() {
  assert(std::strcspn("hello", "el") == 1);
  assert(std::strcspn("abc", "xyz") == 3);
  assert(std::strcspn("", "abc") == 0);
  const char *s = "hello";
  const char *rej = "el";
  assert(std::strcspn(s, rej) == 1);
}

static void test_strspn() {
  assert(std::strspn("hello", "hel") == 4);
  assert(std::strspn("abc", "xyz") == 0);
  assert(std::strspn("aaa", "a") == 3);
  const char *s = "hello";
  const char *acc = "hel";
  assert(std::strspn(s, acc) == 4);
}

static void test_strstr() {
  const char *h = "hello world";
  const char *r = std::strstr(h, "world");
  assert(r != nullptr);
  assert(r == h + 6);
  assert(std::strstr(h, "xyz") == nullptr);
  char buf[] = {'h', 'e', 'l', 'l', 'o', '\0'};
  assert(std::strstr(buf, "ll") == &buf[2]);
}

static void test_strpbrk() {
  const char *s = "hello world";
  const char *r = std::strpbrk(s, "wo");
  assert(r != nullptr);
  assert(r == s + 4);
  assert(std::strpbrk(s, "xyz") == nullptr);
  char buf[] = {'a', 'b', 'c', '\0'};
  assert(std::strpbrk(buf, "b") == &buf[1]);
}

static void test_strcasecmp() {
  assert(strcasecmp("HELLO", "hello") == 0);
  assert(strcasecmp("abc", "abd") < 0);
  assert(strcasecmp("abd", "abc") > 0);
  const char *p = "FOO";
  const char *q = "foo";
  assert(strcasecmp(p, q) == 0);
}

int main() {
  test_memcpy();
  test_memset();
  test_memcmp();
  test_memmove();
  test_strchr();
  test_strlen();
  test_strcmp();
  test_strncmp();
  test_memchr();
  test_strrchr();
  test_strdup();
  test_strcspn();
  test_strspn();
  test_strstr();
  test_strpbrk();
  test_strcasecmp();
  return 0;
}
