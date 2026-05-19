// no-compile: refcount
#include <assert.h>
#include <pwd.h>
#include <stdio.h>
#include <stddef.h>
#include <unistd.h>

static void test_getpwuid(void) {
  uid_t uid = geteuid();
  struct passwd *pw = getpwuid(uid);
  assert(pw != NULL);
  assert(pw->pw_uid == uid);
  printf("%s\n", pw->pw_name);
}

int main(void) {
  test_getpwuid();
  return 0;
}
