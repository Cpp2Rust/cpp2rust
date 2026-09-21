#include "s.h"

S::S(int x) : v(x) {}

S::~S() {}

void S::set(int x) { v = x; }

int S::add(int x) {
  v += x;
  return v;
}

Derived::Derived(int factor) : factor(factor) {}

int Derived::apply(int x) { return factor * x; }
