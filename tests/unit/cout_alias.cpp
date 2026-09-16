#include <iostream>

int main() {
  bool cond = true;
  std::ostream *os1 = cond ? &std::cout : &std::cerr;
  *os1 << "hello\n";

  std::ostream &os2 = cond ? std::cout : std::cerr;
  os2 << "hello\n";
  return 0;
}
