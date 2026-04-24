// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <streambuf>
#include <string>

using t1 = std::string;
using t2 = std::string::iterator;

std::string f1(const std::string &s, std::size_t pos, std::size_t count) {
  return s.substr(pos, count);
}

std::size_t f2(const std::string &s) { return s.size(); }

std::string f3(const std::string &a, const char *b) { return a + b; }

std::string &f4(std::string &s, const char *p, std::size_t n) {
  return s.append(p, n);
}

const char *f5(const std::string &s) { return s.c_str(); }

char *f6(std::string &s) { return s.data(); }

std::string f7(const char *s, std::size_t n) { return std::string(s, n); }

std::string f8(std::istreambuf_iterator<char> first,
               std::istreambuf_iterator<char> last) {
  return std::string(first, last);
}

std::string f9(std::size_t n, char ch) { return std::string(n, ch); }

std::string f10(const char *s) { return std::string(s); }

const char *f11(const std::string &o) { return o.data(); }

std::string::iterator f12(std::string &s) { return s.begin(); }

void f13(std::string &s, std::size_t n) { return s.resize(n); }

std::string &f14(std::string &s, std::size_t pos, std::size_t count,
                 const char *p, std::size_t n) {
  return s.replace(pos, count, p, n);
}

std::string::iterator f15(std::string &s) { return s.end(); }

std::size_t f16(const std::string &s, const char *chars) {
  return s.find_last_of(chars);
}

std::string f17(std::string &&a0, const char *a1) { return std::move(a0) + a1; }

bool f18(const std::string &a, const char *b) { return a == b; }

std::size_t f19(const std::string &s) { return s.length(); }

std::string::iterator f20(std::string::iterator it, std::size_t n) {
  return it + n;
}

std::string &f21(std::string &s, std::size_t n, char c) {
  return s.append(n, c);
}

bool f22(std::string &s) { return s.empty(); }

std::string f23() { return std::string(); }

void f24(std::string &o) { return o.clear(); }

void f25(std::string &o) { return o.shrink_to_fit(); }

char &f26(std::string &o, std::size_t idx) { return o.at(idx); }
