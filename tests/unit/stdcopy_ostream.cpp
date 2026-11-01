#include <algorithm>
#include <cassert>
#include <fcntl.h>
#include <fstream>
#include <iterator>
#include <string>
#include <unistd.h>

int main() {
  std::string str = "Hello, world!\n";
  const char file[] = "test_stdcopy_ostream.txt";
  {
    std::ofstream ofs(file, std::ios::binary);
    std::copy(str.begin(), str.end(), std::ostream_iterator<char>(ofs));
  }
  int fd = open(file, O_RDONLY);
  char buf[sizeof "Hello, world!\n"] = {};
  assert(read(fd, buf, sizeof(buf)) == str.size());
  assert(str == buf);
  assert(close(fd) == 0);
  unlink(file);
  return 0;
}
