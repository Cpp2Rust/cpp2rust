#include <cassert>
#include <fcntl.h>
#include <fstream>
#include <iterator>
#include <string>
#include <unistd.h>

const unsigned char data[] = {0, 127, 255};

void check(const std::string &s) {
  assert(s.size() == sizeof(data));
  for (unsigned i = 0; i < sizeof(data); ++i)
    assert(static_cast<unsigned char>(s[i]) == data[i]);
}

int main() {
  const char file[] = "test_stdcopy_istream.txt";
  int fd = open(file, O_WRONLY | O_CREAT | O_TRUNC, 0600);
  assert(fd >= 0);
  assert(write(fd, data, sizeof(data)) == sizeof(data));
  assert(close(fd) == 0);
  {
    std::ifstream f(file, std::ios::binary);
    std::string s((std::istreambuf_iterator<char>(f.rdbuf())),
                  std::istreambuf_iterator<char>());
    check(s);
  }
  {
    std::ifstream f(file, std::ios::binary);
    std::istreambuf_iterator<char> begin(f.rdbuf());
    std::istreambuf_iterator<char> copy(begin);
    std::istreambuf_iterator<char> end;
    std::string s(copy, end);
    check(s);
  }
  unlink(file);
  return 0;
}
