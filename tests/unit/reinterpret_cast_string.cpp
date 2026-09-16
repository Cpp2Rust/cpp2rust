#include <cassert>
#include <cstdint>
#include <string>

int main() {
  std::string s = "ABCD";
  const uint8_t *bytes = reinterpret_cast<const uint8_t *>(s.c_str());

  assert(bytes[0] == 'A');
  assert(bytes[1] == 'B');
  assert(bytes[2] == 'C');
  assert(bytes[3] == 'D');
  assert(bytes[4] == 0); // null terminator

  return 0;
}
