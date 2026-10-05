// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fstream>
#include <iostream>
#include <iterator>

using t1 = std::ifstream;
using t2 = std::ofstream;

std::ofstream f1(const char *filename, std::ios_base::openmode mode) {
  return std::ofstream(filename, mode);
}

std::filebuf *f4(const std::ifstream &o) { return o.rdbuf(); }

std::ifstream f5(const char *filename, std::ios_base::openmode mode) {
  return std::ifstream(filename, mode);
}
