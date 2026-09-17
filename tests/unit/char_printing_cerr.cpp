#include <iostream>
#include <string>
#include <vector>

int main() {
  std::vector<unsigned char> vec = {0xc3, 0xa7};
  int i = 27;
  std::string str = "rdas.";

  std::cerr << i << " a" << vec[0] << vec[1] << 'o' << str << std::endl
            << std::hex << "0x" << 27 << " açordas?" << '\n'
            << "Sim, 0x" << i << '.' << std::endl
            << std::dec;

  std::cerr << 'H' << 'e' << 'l' << 'l' << 'o' << ',' << ' ' << 'W' << 'o'
            << 'r' << 'l' << 'd' << '!' << '\n';

  std::cerr << vec[0] << '\n' << vec[1] << '\n';

  return 0;
}
