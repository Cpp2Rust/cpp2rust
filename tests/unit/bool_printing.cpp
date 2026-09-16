#include <iostream>

bool foo() { return true; }

bool bar() { return true; }

int main() {
  int i1 = 0;
  int i2 = 1;

  std::cout << true << '\n';
  std::cout << false << '\n';
  std::cout << (i1 != i2) << '\n';
  std::cout << (i1 == i2) << '\n';
  std::cout << foo() << '\n';
  std::cout << bar() << '\n';
  return 0;
}
