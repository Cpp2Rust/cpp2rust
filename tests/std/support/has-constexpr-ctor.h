#pragma once

struct HasConstexprCtor {
  static inline int copyCount = 0;

  int m;

  constexpr HasConstexprCtor(int v) : m(v) {}
  HasConstexprCtor(const HasConstexprCtor &o) : m(o.m) { ++copyCount; }
};
