#include <cassert>
#include <cctype>

int main() {
  assert(toupper('a') == 'A');
  assert(toupper('Z') == 'Z');
  assert(toupper('5') == '5');
  return 0;
}
