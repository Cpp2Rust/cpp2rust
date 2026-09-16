#include <cassert>

unsigned strlen(char *ptr) {
  unsigned count = 0;
  while (*ptr++ != '\0')
    ++count;
  return count;
}

int main() {
  char string[] = {'h', 'e', 'l', 'l', 'o', '\0'};
  assert(strlen(&string[0]) == 5);
  return 0;
}
