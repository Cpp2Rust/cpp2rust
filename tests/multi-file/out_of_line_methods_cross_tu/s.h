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
  virtual int scale(int x) = 0;
};

class D : public Base {
public:
  D(int f);
  int scale(int x) override;

  int f;
};
