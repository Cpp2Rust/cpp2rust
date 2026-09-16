#include <assert.h>

typedef void (*callback_t)(int *);

void double_it(int *x) { *x *= 2; }

void maybe_call(callback_t cb, int *x) {
  if (cb) {
    cb(x);
  }
}

int main() {
  int a = 5;
  maybe_call(double_it, &a);
  assert(a == 10);

  int b = 5;
  maybe_call(nullptr, &b);
  assert(b == 5);

  callback_t fn = nullptr;
  if (!fn) {
    fn = double_it;
  }
  int c = 3;
  if (fn) {
    fn(&c);
  }
  assert(c == 6);

  return 0;
}
