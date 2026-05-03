#include <assert.h>
#include <stddef.h>

enum Code { CODE_OK = 0, CODE_ERR = 1, CODE_FATAL = 2 };

int side_effect = 0;
int observe(int v) {
  ++side_effect;
  return v;
}

int returns_one() { return 1; }
int returns_zero() { return 0; }

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
  if ((p != nullptr) && (s < 0)) {
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

  unsigned long long ull = 7ull;
  if ((p != nullptr) && ull) {
    assert(true);
  }
  if ((x > y) && ull) {
    assert(true);
  }

  long long mask = (1ll << 4) | (1ll << 5);
  long long bits = 1ll << 4;
  if ((n != 0) && (bits & mask)) {
    assert(true);
  }
  if ((n != 0) || (bits & 0x100ll)) {
    assert(true);
  }

  const char *cp = "hi";
  const char *cnp = nullptr;
  if ((x > y) && cp) {
    assert(true);
  }
  if ((x < y) || cnp) {
    assert(false);
  }
  if ((x > y) && (n && cp)) {
    assert(true);
  }

  if ((x > y) && returns_one()) {
    assert(true);
  }
  if ((x > y) && !returns_zero()) {
    assert(true);
  }
  if ((x < y) || returns_one()) {
    assert(true);
  }
  if ((x < y) || !returns_one()) {
    assert(false);
  }
  if ((p != nullptr) && returns_one() && (n != 0)) {
    assert(true);
  }

  return 0;
}
