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
