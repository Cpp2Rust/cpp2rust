// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <iterator>
#include <streambuf>
#include <string>
#endif

using CPP2RUST_TYPE_RULE(1) = std::string;
using CPP2RUST_TYPE_RULE(2) = std::string::iterator;
typedef std::string::size_type CPP2RUST_TYPE_RULE(3);

std::string CPP2RUST_EXPR_RULE(1)(const std::string &s, std::size_t pos,
                                  std::size_t count) {
  return s.substr(pos, count);
}

std::size_t CPP2RUST_EXPR_RULE(2)(const std::string &s) { return s.size(); }

std::string CPP2RUST_EXPR_RULE(3)(const std::string &a, const char *b) {
  return a + b;
}

std::string &CPP2RUST_EXPR_RULE(4)(std::string &s, const char *p,
                                   std::size_t n) {
  return s.append(p, n);
}

const char *CPP2RUST_EXPR_RULE(5)(const std::string &s) { return s.c_str(); }

char *CPP2RUST_EXPR_RULE(6)(std::string &s) { return s.data(); }

std::string CPP2RUST_EXPR_RULE(7)(const char *s, std::size_t n) {
  return std::string(s, n);
}

std::string CPP2RUST_EXPR_RULE(8)(std::istreambuf_iterator<char> first,
                                  std::istreambuf_iterator<char> last) {
  return std::string(first, last);
}

std::string CPP2RUST_EXPR_RULE(9)(std::size_t n, char ch) {
  return std::string(n, ch);
}

std::string CPP2RUST_EXPR_RULE(10)(const char *s) { return std::string(s); }

const char *CPP2RUST_EXPR_RULE(11)(const std::string &o) { return o.data(); }

std::string::iterator CPP2RUST_EXPR_RULE(12)(std::string &s) {
  return s.begin();
}

void CPP2RUST_EXPR_RULE(13)(std::string &s, std::size_t n) {
  return s.resize(n);
}

std::string &CPP2RUST_EXPR_RULE(14)(std::string &s, std::size_t pos,
                                    std::size_t count, const char *p,
                                    std::size_t n) {
  return s.replace(pos, count, p, n);
}

std::string::iterator CPP2RUST_EXPR_RULE(15)(std::string &s) { return s.end(); }

std::size_t CPP2RUST_EXPR_RULE(16)(const std::string &s, const char *chars) {
  return s.find_last_of(chars);
}

std::string CPP2RUST_EXPR_RULE(17)(std::string &&a0, const char *a1) {
  return std::move(a0) + a1;
}

bool CPP2RUST_EXPR_RULE(18)(const std::string &a, const char *b) {
  return a == b;
}

std::size_t CPP2RUST_EXPR_RULE(19)(const std::string &s) { return s.length(); }

std::string::iterator CPP2RUST_EXPR_RULE(20)(std::string::iterator it,
                                             std::size_t n) {
  return it + n;
}

std::string &CPP2RUST_EXPR_RULE(21)(std::string &s, std::size_t n, char c) {
  return s.append(n, c);
}

bool CPP2RUST_EXPR_RULE(22)(std::string &s) { return s.empty(); }

std::string CPP2RUST_EXPR_RULE(23)() { return std::string(); }

void CPP2RUST_EXPR_RULE(24)(std::string &o) { return o.clear(); }

void CPP2RUST_EXPR_RULE(25)(std::string &o) { return o.shrink_to_fit(); }

char &CPP2RUST_EXPR_RULE(26)(std::string &o, std::size_t idx) {
  return o.at(idx);
}

std::string CPP2RUST_EXPR_RULE(27)(const std::string &o) {
  return std::string(o);
}

std::string CPP2RUST_EXPR_RULE(28)(std::string &&o) {
  return std::string(std::move(o));
}

std::string &CPP2RUST_EXPR_RULE(29)(std::string &dst, const std::string &src) {
  return dst.operator=(src);
}

std::string &CPP2RUST_EXPR_RULE(30)(std::string &dst, std::string &&src) {
  return dst.operator=(std::move(src));
}
