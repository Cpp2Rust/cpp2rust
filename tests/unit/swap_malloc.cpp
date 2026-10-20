#include <cassert>
#include <cstdlib>
#include <utility>

int main() {
  int *buf = static_cast<int *>(malloc(3 * sizeof(int)));
  buf[0] = 1;
  buf[1] = 2;
  buf[2] = 3;
  std::swap(buf[0], buf[2]);
  assert(buf[0] == 3 && buf[1] == 2 && buf[2] == 1);
  std::swap(buf[1], buf[1]);
  assert(buf[1] == 2);

  int local = 4;
  std::swap(buf[1], local);
  assert(buf[1] == 4 && local == 2);

  int *other = static_cast<int *>(malloc(sizeof(int)));
  *other = 5;
  std::swap(*buf, *other);
  assert(buf[0] == 5 && *other == 3);

  unsigned char *bytes = reinterpret_cast<unsigned char *>(buf);
  bytes[0] = 0xAA;
  bytes[4] = 0xBB;
  std::swap(bytes[0], bytes[4]);
  assert(bytes[0] == 0xBB && bytes[4] == 0xAA);

  unsigned words[2] = {0x01020304, 0x05060708};
  unsigned short *halves = reinterpret_cast<unsigned short *>(words);
  std::swap(halves[0], halves[3]);
  assert(words[0] == 0x01020506 && words[1] == 0x03040708);

  free(other);
  free(buf);
  return 0;
}
