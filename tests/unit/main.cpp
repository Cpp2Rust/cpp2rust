// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>
#include <string>
int main(int argc, char *argv[]) {
  std::string s = argv[0];
  assert(argc == 1);
  assert(s.size() > 0);
  assert(argc + (s.size() > 0) == 2);
  return 0;
}
