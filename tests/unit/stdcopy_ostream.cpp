#include <algorithm>
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
  unlink(file);
  return 0;
}
