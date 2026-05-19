// no-compile: refcount
#define _GNU_SOURCE
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

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

static void test_strcmp(void) {
  assert(strcmp("abc", "abc") == 0);
  assert(strcmp("abc", "abd") < 0);
  assert(strcmp("abd", "abc") > 0);
  const char *p = "abc";
  const char *q = "abd";
  char buf[] = {'a', 'b', 'c', '\0'};
  assert(strcmp(p, p) == 0);
  assert(strcmp(p, q) < 0);
  assert(strcmp(buf, p) == 0);
}

static void test_strncmp(void) {
  assert(strncmp("abcdef", "abcxyz", 3) == 0);
  assert(strncmp("abcdef", "abcxyz", 4) < 0);
  assert(strncmp("abcxyz", "abcdef", 4) > 0);
  const char *p = "abcdef";
  const char *q = "abcxyz";
  char buf[] = {'a', 'b', 'c', 'd', 'e', 'f', '\0'};
  size_t n = 3;
  assert(strncmp(p, q, n) == 0);
  assert(strncmp(p, q, n + 1) < 0);
  assert(strncmp(buf, p, 6) == 0);
}

static void test_memchr(void) {
  const char data[] = {0x10, 0x20, 0x30, 0x40};
  void *r = memchr(data, 0x30, 4);
  assert(r == &data[2]);
  assert(memchr(data, 0x99, 4) == NULL);
  const void *p = data;
  size_t n = 4;
  assert(memchr(p, 0x10, n) == p);
}

static void test_strrchr(void) {
  const char *s = "hello world";
  char *r = strrchr((char *)s, 'l');
  assert(r != NULL);
  assert(*r == 'l');
  assert(r == s + 9);
  assert(strrchr((char *)s, 'z') == NULL);
  char buf[] = {'a', 'b', 'a', '\0'};
  assert(strrchr(buf, 'a') == &buf[2]);
}

static void test_strdup(void) {
  char *d = strdup("hello");
  assert(d != NULL);
  assert(strcmp(d, "hello") == 0);
  free(d);
  const char *p = "world";
  char buf[] = {'a', 'b', 'c', '\0'};
  char *d2 = strdup(p);
  assert(d2 != NULL);
  assert(strcmp(d2, p) == 0);
  free(d2);
  char *d3 = strdup(buf);
  assert(d3 != NULL);
  assert(strcmp(d3, buf) == 0);
  free(d3);
}

static void test_strcspn(void) {
  assert(strcspn("hello", "el") == 1);
  assert(strcspn("abc", "xyz") == 3);
  assert(strcspn("", "abc") == 0);
  const char *s = "hello";
  const char *rej = "el";
  assert(strcspn(s, rej) == 1);
}

static void test_strspn(void) {
  assert(strspn("hello", "hel") == 4);
  assert(strspn("abc", "xyz") == 0);
  assert(strspn("aaa", "a") == 3);
  const char *s = "hello";
  const char *acc = "hel";
  assert(strspn(s, acc) == 4);
}

static void test_strstr(void) {
  const char *h = "hello world";
  char *r = strstr((char *)h, "world");
  assert(r != NULL);
  assert(r == h + 6);
  assert(strstr((char *)h, "xyz") == NULL);
  char buf[] = {'h', 'e', 'l', 'l', 'o', '\0'};
  assert(strstr(buf, "ll") == &buf[2]);
}

static void test_strpbrk(void) {
  const char *s = "hello world";
  char *r = strpbrk((char *)s, "wo");
  assert(r != NULL);
  assert(r == s + 4);
  assert(strpbrk((char *)s, "xyz") == NULL);
  char buf[] = {'a', 'b', 'c', '\0'};
  assert(strpbrk(buf, "b") == &buf[1]);
}

static void test_strcasecmp(void) {
  assert(strcasecmp("HELLO", "hello") == 0);
  assert(strcasecmp("abc", "abd") < 0);
  assert(strcasecmp("abd", "abc") > 0);
  const char *p = "FOO";
  const char *q = "foo";
  assert(strcasecmp(p, q) == 0);
}

int main(void) {
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
