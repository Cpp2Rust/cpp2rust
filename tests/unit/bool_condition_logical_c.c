#include <assert.h>
#include <stdbool.h>
#include <stddef.h>

enum Code { CODE_OK = 0, CODE_ERR = 1, CODE_FATAL = 2 };

int side_effect = 0;
int observe(int v) {
  ++side_effect;
  return v;
}

int main() {
  int n = 3;
  int zero = 0;
  int storage = 7;
  int *p = &storage;
  int *np = NULL;
  unsigned u = 4;
  enum Code code = CODE_OK;

  if (n && p) {
    assert(true);
  }
  if (n && np) {
    assert(false);
  }
  if (zero || p) {
    assert(true);
  }
  if (zero || np) {
    assert(false);
  }
  if (n && u && p && code == CODE_OK) {
    assert(true);
  }

  side_effect = 0;
  if (zero && observe(1)) {
    assert(false);
  }
  assert(side_effect == 0);

  if (n || observe(1)) {
    assert(true);
  }
  assert(side_effect == 0);

  int x = 5;
  int y = 3;
  unsigned flags = 0x2u;
  if ((x > y) || (flags & 0x1u)) {
    assert(true);
  }
  if ((x < y) || (flags & 0x1u)) {
    assert(false);
  }

  unsigned a = 1u;
  unsigned b = 2u;
  unsigned c = 3u;
  if (((a != c)) && ((b != c))) {
    assert(true);
  }

  int s = -1;
  if ((p != NULL) && (s < 0)) {
    assert(true);
  }

  unsigned k = 2u;
  bool done = false;
  if ((k > 1u) || !done) {
    assert(true);
  }

  if (((x > y)) || ((flags & 0x4u))) {
    assert(true);
  }

  return 0;
}
