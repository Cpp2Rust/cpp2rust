#include "s.h"

S::S(int x) : v(x) {}

S::~S() {}

void S::set(int x) { v = x; }

int S::add(int x) {
  v += x;
  return v;
}

D::D(int f) : f(f) {}

int D::scale(int x) { return f * x; }
