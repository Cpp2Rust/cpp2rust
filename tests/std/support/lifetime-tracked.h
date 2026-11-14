#pragma once

#include <cassert>

struct LifetimeTracked {
  static inline int ctorCount = 0;
  static inline int copyCount = 0;
  static inline int dtorCount = 0;
  static inline int useCount = 0;

  mutable int uses = 0;

  LifetimeTracked() { ++ctorCount; }
  LifetimeTracked(const LifetimeTracked &) { ++copyCount; }
  ~LifetimeTracked() { ++dtorCount; }
  void use() const {
    ++uses;
    ++useCount;
  }
};

struct HasConstexprCtor {
  static inline int copyCount = 0;

  int m;

  constexpr HasConstexprCtor(int v) : m(v) {}
  HasConstexprCtor(const HasConstexprCtor &o) : m(o.m) { ++copyCount; }
};

struct AssertsDtorCountAtExit {
  int expected;
  ~AssertsDtorCountAtExit() { assert(LifetimeTracked::dtorCount == expected); }
};
