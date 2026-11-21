#include <cassert>
#include <iostream>
#include <string>
#include <vector>

int made = 0;

std::string make(const char *s) {
  ++made;
  return s;
}

int main() {
  std::vector<unsigned char> vec = {0xc3, 0xa7};
  int i = 27;
  std::string str = "foo.";

  std::cout << i << " a" << vec[0] << vec[1] << 'o' << str << std::endl
            << std::hex << "0x" << 27 << " açordas?" << '\n'
            << "Sim, 0x" << i << '.' << std::endl
            << std::dec;

  std::cout << 'H' << 'e' << 'l' << 'l' << 'o' << ',' << ' ' << 'W' << 'o'
            << 'r' << 'l' << 'd' << '!' << '\n';

  std::cout << vec[0] << '\n' << vec[1] << '\n';

  std::cout << make("once") << '\n';
  std::cout << make("alone");
  std::cout << std::endl;
  assert(made == 2);

  // The bytes of C strings need not be UTF-8.
  const char *bytes = "\xe9\xc3\xa9";
  const unsigned char *ubytes = reinterpret_cast<const unsigned char *>(bytes);
  std::cout << bytes << '|' << ubytes << '|' << "\xff" << std::endl;

  return 0;
}
