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
    auto out =
        std::copy(str.begin(), str.end(), std::ostream_iterator<char>(ofs));
    std::ostream_iterator<char> copy(out);
    std::copy(str.begin(), str.end(), copy);
  }
  int fd = open(file, O_RDONLY);
  char buf[sizeof "Hello, world!\n"] = {};
  for (int i = 0; i < 2; ++i) {
    assert(read(fd, buf, str.size()) == str.size());
    assert(str == buf);
  }
  assert(close(fd) == 0);
  unlink(file);
  return 0;
}
