// no-compile: refcount
#include <pwd.h>
#include <unistd.h>

int main(void) {
  struct passwd *pw = getpwuid(geteuid());
  if (!pw) {
    return 0;
  }
  char *home = pw->pw_dir;
  return home == 0;
}
