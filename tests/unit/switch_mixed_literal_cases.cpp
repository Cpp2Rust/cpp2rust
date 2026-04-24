#include <cassert>

int mixed_literal_cases(int x) {
  switch (x) {
  case -1:
    return 1;
  case 0x10:
    return 2;
  case 0xFE80:
    return 3;
  case -0xFF:
    return 4;
  default:
    return 0;
  }
}

int main() {
  assert(mixed_literal_cases(-1) == 1);
  assert(mixed_literal_cases(0x10) == 2);
  assert(mixed_literal_cases(0xFE80) == 3);
  assert(mixed_literal_cases(-0xFF) == 4);
  assert(mixed_literal_cases(7) == 0);
  return 0;
}
