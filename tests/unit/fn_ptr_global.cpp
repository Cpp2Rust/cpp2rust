#include <assert.h>

typedef int (*op_t)(int);

int double_it(int x) { return x * 2; }
int triple_it(int x) { return x * 3; }

static op_t g_op = nullptr;

void set_op(op_t fn) { g_op = fn; }

int call_op(int x) {
  if (g_op) {
    return g_op(x);
  }
  return x;
}

int main() {
  assert(call_op(5) == 5);

  set_op(double_it);
  assert(g_op != nullptr);
  assert(g_op == double_it);
  assert(call_op(5) == 10);

  set_op(triple_it);
  assert(g_op == triple_it);
  assert(call_op(5) == 15);

  set_op(nullptr);
  assert(g_op == nullptr);
  assert(call_op(5) == 5);

  return 0;
}
