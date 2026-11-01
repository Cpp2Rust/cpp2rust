#include <assert.h>
#include <iostream>
#include <unistd.h>

int main() {
  assert(close(1) == 0);
  std::cout << "not written\n";
  std::cerr << "still open\n";
  assert(close(2) == 0);
  std::cerr << "not written either\n";
  return 0;
}
