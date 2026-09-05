#include <cassert>

int main() {
  unsigned a = 0b1100, b = 0b1010, z = 0;
  assert(not z);
  assert(a and b);
  assert(not(a and z));
  assert(a or z);
  assert(a not_eq b);
  assert((compl a) == compl 0b1100u);
  assert((a bitand b) == 0b1000);
  assert((a bitor b) == 0b1110);
  assert((a xor b) == 0b0110);
  a and_eq b;
  assert(a == 0b1000);
  a or_eq b;
  assert(a == 0b1010);
  a xor_eq b;
  assert(a == 0);
  return 0;
}
