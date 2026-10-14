#include <cassert>
#include <string>

int main() {
  std::string arr = "foo";
  arr[1] = 'b';
  const char *p = arr.c_str() + 1;
  assert(*p == 'b');
  assert(arr == "fbo");
  char *names[] = {arr.data()};
  const std::string copy = std::string(names[0]);
  assert(copy == "fbo");
  return 0;
}
