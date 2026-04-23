// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int empty_switch(int x) {
  switch (x) {}
  return x;
}


int main() {
  assert(empty_switch(5) == 5);
  return 0;
}
