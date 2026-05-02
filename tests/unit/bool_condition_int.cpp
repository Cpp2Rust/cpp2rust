#include <assert.h>

int main() {
  int n = 3;
  int zero = 0;
  unsigned u = 4;
  unsigned long ul = 5;
  long long ll = 6;
  char ch = 'a';

  if (n) {
    assert(true);
  }
  if (!n) {
    assert(false);
  }
  if (zero) {
    assert(false);
  }
  if (!zero) {
    assert(true);
  }

  if (u) {
    assert(true);
  }
  if (ul) {
    assert(true);
  }
  if (ll) {
    assert(true);
  }
  if (ch) {
    assert(true);
  }

  int loop_count = 0;
  int counter = 3;
  while (counter) {
    --counter;
    ++loop_count;
  }
  assert(loop_count == 3);

  for (int i = 5; i; --i) {
    ++loop_count;
  }
  assert(loop_count == 8);

  int t = n ? 100 : 200;
  assert(t == 100);
  int t2 = zero ? 100 : 200;
  assert(t2 == 200);
  int t7 = !n;
  assert(t7 == 0);
  int t8 = !zero;
  assert(t8 == 1);

  bool b1 = n;
  assert(b1);

  return 0;
}
