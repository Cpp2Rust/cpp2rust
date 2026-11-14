#include <cassert>
#include <iostream>

int main() {
  bool cond = true;
  std::ostream *os1 = cond ? &std::cout : &std::cerr;
  *os1 << "hello\n";

  std::ostream &os2 = cond ? std::cout : std::cerr;
  os2 << "hello\n";

  // The input of the test is empty.
  std::istream &is = std::cin;
  assert(is.get() == -1);
  std::istream *is_ptr = &std::cin;
  assert(is_ptr->get() == -1);
  return 0;
}
