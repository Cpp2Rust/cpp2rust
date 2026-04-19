// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fstream>
#include <iostream>
#include <iterator>

void t1(std::ifstream);
void t2(std::ofstream);
void t3(std::ostream_iterator<char>);

std::ofstream f1(const char *filename, std::ios_base::openmode mode) {
  return std::ofstream(filename, mode);
}

template <typename T1>
std::ostream_iterator<T1> f2(const std::ostream_iterator<T1> &a0) {
  return std::ostream_iterator<T1>(a0);
}

template <typename T1> std::ostream_iterator<T1> f3(std::ostream &a0) {
  return std::ostream_iterator<T1>(a0);
}

std::filebuf *f4(const std::ifstream &o) { return o.rdbuf(); }

std::ifstream f5(const char *filename, std::ios_base::openmode mode) {
  return std::ifstream(filename, mode);
}

template <typename T1>
std::istream_iterator<T1> f6(const std::istream_iterator<T1> &a0) {
  return std::istream_iterator<T1>(a0);
}

template <typename T1>
std::istreambuf_iterator<T1> f7(std::istreambuf_iterator<T1> &a0) {
  return std::istreambuf_iterator<T1>(a0);
}

std::istreambuf_iterator<char> f8(std::basic_streambuf<char> *p) {
  return std::istreambuf_iterator<char>(p);
}
