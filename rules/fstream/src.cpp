// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <fstream>
#include <iostream>
#include <iterator>
#endif

using CPP2RUST_TYPE_RULE(1) = std::ifstream;
using CPP2RUST_TYPE_RULE(2) = std::ofstream;
using CPP2RUST_TYPE_RULE(3) = std::ostream_iterator<char>;

std::ofstream CPP2RUST_EXPR_RULE(1)(const char *filename,
                                    std::ios_base::openmode mode) {
  return std::ofstream(filename, mode);
}

template <typename T1>
std::ostream_iterator<T1>
CPP2RUST_EXPR_RULE(2)(const std::ostream_iterator<T1> &a0) {
  return std::ostream_iterator<T1>(a0);
}

template <typename T1>
std::ostream_iterator<T1> CPP2RUST_EXPR_RULE(3)(std::ostream &a0) {
  return std::ostream_iterator<T1>(a0);
}

std::filebuf *CPP2RUST_EXPR_RULE(4)(const std::ifstream &o) {
  return o.rdbuf();
}

std::ifstream CPP2RUST_EXPR_RULE(5)(const char *filename,
                                    std::ios_base::openmode mode) {
  return std::ifstream(filename, mode);
}

template <typename T1>
std::istream_iterator<T1>
CPP2RUST_EXPR_RULE(6)(const std::istream_iterator<T1> &a0) {
  return std::istream_iterator<T1>(a0);
}

template <typename T1>
std::istreambuf_iterator<T1>
CPP2RUST_EXPR_RULE(7)(std::istreambuf_iterator<T1> &a0) {
  return std::istreambuf_iterator<T1>(a0);
}

std::istreambuf_iterator<char>
CPP2RUST_EXPR_RULE(8)(std::basic_streambuf<char> *p) {
  return std::istreambuf_iterator<char>(p);
}
