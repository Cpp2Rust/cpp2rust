// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <inttypes.h>

int main() {
  const uint8_t xu8 = 8;
  const uint16_t xu16 = 16;
  uint32_t xu32 = 32;
  uint64_t xu64 = 64;
  const size_t xsz1 = 64;
  std::size_t xsz2 = 64;
  const int8_t xi1 = -8;
  const int16_t xi2 = 16;
  int32_t xi3 = 32;
  int64_t xi4 = 64;
  bool b = (xu64 == 64ULL);
  assert(xu8 + xu16 + xu32 + xu64 + xsz1 + xsz2 + xi1 + xi2 + xi3 + xi4 == 352);
  return 0;
}
