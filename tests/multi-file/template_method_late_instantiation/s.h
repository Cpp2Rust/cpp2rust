#pragma once

template <typename T>
struct S {
  T x = 0;

  void set(T v) { x = v; }
  T get() { return x; }
};

int f(S<int> *p);
