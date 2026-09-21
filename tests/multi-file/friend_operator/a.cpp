#include <assert.h>

#include "s.h"

int main() {
  S s1{1, 2};
  S s2{1, 2};
  S s3{1, 3};
  assert(s1 == s2);
  assert(s1 != s3);
  assert(!(s1 == s3));
  assert(s1 < s3);
  assert(!(s3 < s1));
  assert(compare(s1, s3) == -1);
  assert(compare(s3, s1) == 1);
  assert(compare(s1, s2) == 0);
  return 0;
}
