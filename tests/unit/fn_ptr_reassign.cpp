#include <assert.h>

typedef int (*op_t)(int, int);

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }

int main() {
  op_t fn = add;
  assert(fn(3, 4) == 7);

  fn = sub;
  assert(fn(10, 3) == 7);

  fn = mul;
  assert(fn(6, 7) == 42);

  fn = nullptr;
  assert(fn == nullptr);

  fn = add;
  assert(fn != nullptr);
  assert(fn(1, 1) == 2);

  return 0;
}
