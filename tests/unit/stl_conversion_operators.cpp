// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <assert.h>
#include <chrono>
#include <filesystem>
#include <string>

int main() {
  unsigned n = 17;
  std::chrono::day d(n);
  assert(static_cast<unsigned>(d) == 17);

  const char *name = "dir/file.txt";
  std::filesystem::path p(name);
  std::string s = static_cast<std::string>(p);
  assert(s.size() == 12);
  assert(s == "dir/file.txt");
  return 0;
}
