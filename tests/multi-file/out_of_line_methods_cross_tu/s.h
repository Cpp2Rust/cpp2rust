#pragma once

class S {
public:
  S(int x);
  ~S();

  int get() const { return v; }

  void set(int x);
  int add(int x);

  int v;
};

class Base {
public:
  virtual ~Base() {}
  virtual int apply(int x) = 0;
};

class Derived : public Base {
public:
  Derived(int factor);
  int apply(int x) override;

  int factor;
};

extern "C" {
struct Pair {
  int first;
  int second;
};

int pair_sum(const struct Pair *p);

inline int pair_diff(const struct Pair *p) { return p->first - p->second; }
}
