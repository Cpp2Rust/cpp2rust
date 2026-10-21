// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>
#include <filesystem>
#include <string>

int main() {
  const char *name = "dir/file.txt";
  std::filesystem::path p(name);
  std::string s = static_cast<std::string>(p);
  assert(s.size() == 12);
  assert(s == "dir/file.txt");
  return 0;
}
