#pragma once

struct S {
  int a;
  int b;

  friend bool operator==(const S &x, const S &y);
  friend bool operator!=(const S &x, const S &y);
  friend bool operator<(const S &x, const S &y);
};

int compare(const S &x, const S &y);
