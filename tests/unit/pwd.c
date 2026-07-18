#include <assert.h>
#include <errno.h>
#include <pwd.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

static void test_getpwuid(void) {
  struct passwd *pw = getpwuid(geteuid());
  assert(pw != NULL);
  assert(pw->pw_uid == geteuid());
  assert(strlen(pw->pw_name) > 0);
  assert(pw->pw_dir != NULL);
  printf("%s\n", pw->pw_name);
}

static void test_getpwuid_missing(void) {
  errno = 0;
  struct passwd *pw = getpwuid(0x7ffffffe);
  assert(pw == NULL);
  assert(errno == 0);
}

static void test_getpwuid_r(void) {
  struct passwd pw;
  char buf[4096];
  struct passwd *result = NULL;
  assert(getpwuid_r(geteuid(), &pw, buf, sizeof(buf), &result) == 0);
  assert(result == &pw);
  assert(pw.pw_uid == geteuid());
  assert(strlen(pw.pw_name) > 0);
  struct passwd *pw2 = getpwuid(geteuid());
  assert(pw2 != NULL);
  assert(strcmp(pw.pw_name, pw2->pw_name) == 0);
  printf("%s\n", pw.pw_name);
}

static void test_getpwuid_r_erange(void) {
  struct passwd pw;
  char tiny[1];
  struct passwd *result = NULL;
  assert(getpwuid_r(geteuid(), &pw, tiny, sizeof(tiny), &result) == ERANGE);
  assert(result == NULL);
}

int main(void) {
  test_getpwuid();
  test_getpwuid_missing();
  test_getpwuid_r();
  test_getpwuid_r_erange();
  return 0;
}
