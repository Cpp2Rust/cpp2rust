#include <assert.h>
#include <stddef.h>

int main() {
  int storage = 7;
  int *p = &storage;
  int *np = nullptr;

  if (p) {
    assert(true);
  }
  if (!p) {
    assert(false);
  }
  if (np) {
    assert(false);
  }
  if (!np) {
    assert(true);
  }

  int *iter = p;
  int iters = 0;
  while (iter) {
    ++iters;
    iter = nullptr;
  }
  assert(iters == 1);

  int t3 = p ? 1 : 0;
  assert(t3 == 1);
  int t4 = np ? 1 : 0;
  assert(t4 == 0);
  int t5 = !p;
  assert(t5 == 0);
  int t6 = !np;
  assert(t6 == 1);

  bool b2 = p;
  bool b3 = np;
  assert(b2);
  assert(!b3);

  return 0;
}
