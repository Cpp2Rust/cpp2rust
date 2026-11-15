#include <cassert>

#include "h.h"

int main() {
  assert(&factor != b_factor_address());
  assert(&scaled == b_scaled_address());
  assert(scaled(2) == 6);
  assert(b_scaled(2) == 6);
  return 0;
}
