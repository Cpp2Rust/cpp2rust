// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int foo(int x) { return x; }
int *ptr(int *x) { return x; }
int &bar(int &x) { return x; }
struct X1 {
  int v;
};
struct X2 {
  X1 &v;
  X1 &get() { return v; }
};
struct X3 {
  X2 *v;
  X2 *get() { return v; }
};
struct X4 {
  X3 v;
  X3 &get() { return v; }
};
int main() {
  int x1 = 0;
  int x2 = foo(x1);
  int x3 = foo(x2) + foo(x1) + 1;
  x2 += 1;
  x2 += foo(x1);
  x3 += foo(x2) + foo(x3) + 1;
  int *p1 = &x1;
  int *p2 = ptr(p1);
  p1 = p2;
  p2 = ptr(p1);
  int &r1 = x1;
  int &r2 = bar(x1);
  int &r3 = bar(r1);
  r2 += x1;
  r3 += r1;
  int x4 = foo(x3) + *ptr(&x3) + bar(x2);
  X1 a{0};
  X2 b{a};
  X3 c{&b};
  X4 d{c};
  d.v.v->v.v = 0;
  d.get().get()->get().v = 0;
  d.v.v = &b;
  [[maybe_unused]] const int &r4 = (*d.get().get()).get().v;
  [[maybe_unused]] X1 &r5 = d.get().get()->get();
  [[maybe_unused]] X2 *p = d.get().get();
  [[maybe_unused]] X3 &r6 = d.get();
  [[maybe_unused]] X3 &r7 = d.v;
  [[maybe_unused]] int &r8 = d.v.get()->get().v;
  [[maybe_unused]] int x5 = (*d.get().get()).get().v;
  bar(x1) += 10;
  bar(x1)++;
  [[maybe_unused]] int bar_out = bar(d.get().get()->get().v);
  [[maybe_unused]] int bar_inc = ++bar(x1);
  bar_inc = bar(x1)++;
  bar_inc = bar(x1) + foo(x4) + 1;
  bar(d.get().get()->get().v) += 10;
  bar(d.get().get()->get().v)++;
  [[maybe_unused]] int bar_inc2 = ++bar(d.get().get()->get().v);
  bar_inc2 = bar(d.get().get()->get().v)++;
  ++*ptr(&x1);
  *ptr(&x1) += 1;
  ++*ptr(&d.get().get()->get().v);
  *ptr(&d.get().get()->get().v) += 1;
  *&*ptr(&d.get().get()->get().v) += 1;
  [[maybe_unused]] int ptr1 = (*ptr(&d.get().get()->get().v))++;
  [[maybe_unused]] int &ptr2 = *ptr(&d.get().get()->get().v);
  [[maybe_unused]] int *ptr3 = &*ptr(&d.get().get()->get().v);
  [[maybe_unused]] int vptr = *ptr(&d.get().get()->get().v);
  [[maybe_unused]] int *pref = &bar(d.get().get()->get().v);
  (*(&bar(d.get().get()->get().v)))++;
  assert(*ptr(&d.get().get()->get().v) + bar(d.get().get()->get().v) +
             foo(d.get().get()->get().v) ==
         54);
  return 0;
}
