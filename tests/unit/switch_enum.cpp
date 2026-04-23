// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

enum Color { kRed, kGreen, kBlue };

int switch_enum(Color c) {
  switch (c) {
  case kRed:
    return 10;
  case kGreen:
    return 20;
  case kBlue:
    return 30;
  }
  return -1;
}


int main() {
  assert(switch_enum(kRed) == 10);
  assert(switch_enum(kGreen) == 20);
  assert(switch_enum(kBlue) == 30);
  return 0;
}
