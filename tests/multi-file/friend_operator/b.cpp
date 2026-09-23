#include "s.h"

bool operator==(const S &x, const S &y) { return x.a == y.a && x.b == y.b; }

bool operator!=(const S &x, const S &y) { return !(x == y); }

bool operator<(const S &x, const S &y) {
  return x.a < y.a || (x.a == y.a && x.b < y.b);
}

int compare(const S &x, const S &y) {
  if (x < y) {
    return -1;
  }
  if (y < x) {
    return 1;
  }
  return 0;
}
