#pragma once

#include <vector>

struct S {
  std::vector<int> v;
  int n[2];

  S(int x) : v(x, x), n{x, x + 1} {}
  S(const S &) = delete;
  S(S &&) = default;
  S &operator=(const S &) = delete;
  S &operator=(S &&) = default;
};

// Copyable, with implicit members, and moved only in b.cpp.
struct T {
  std::vector<int> v;
  int size() const { return static_cast<int>(v.size()); }
};

int sum(const S &s);
int shuffle(int x);
