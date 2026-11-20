#include <algorithm>
#include <cassert>
#include <cstdio>
#include <fstream>
#include <iterator>
#include <string>
#include <unistd.h>

int main() {
  std::string str = "Hello, world!\n";
  // The name is UTF-8, which all file systems take, and not ASCII.
  const char file[] = "test_stdcopy_ostream\xc3\xa9.txt";
  {
    std::ofstream ofs(file, std::ios::binary);
    std::copy(str.begin(), str.end(), std::ostream_iterator<char>(ofs));
    ofs << 42;
  }
  FILE *f = fopen(file, "rb");
  char buf[32] = {0};
  size_t n = fread(buf, 1, sizeof(buf), f);
  fclose(f);
  assert(n == 16);
  assert(buf[14] == '4' && buf[15] == '2');
  unlink(file);
  return 0;
}
