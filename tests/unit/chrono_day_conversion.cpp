// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <assert.h>
#include <chrono>

int main() {
  unsigned n = 17;
  std::chrono::day d(n);
  assert(static_cast<unsigned>(d) == 17);
  return 0;
}
