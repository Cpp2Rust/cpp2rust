#include <assert.h>
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
  int *np = nullptr;
  unsigned u = 4;
  Code code = CODE_OK;

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

  return 0;
}
