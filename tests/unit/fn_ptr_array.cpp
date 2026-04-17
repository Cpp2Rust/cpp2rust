// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef int (*op_t)(int, int);

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }

int main() {
  op_t ops[3] = {add, sub, mul};

  assert(ops[0](2, 3) == 5);
  assert(ops[1](7, 4) == 3);
  assert(ops[2](6, 5) == 30);

  assert(ops[0] != nullptr);
  assert(ops[0] == add);
  assert(ops[0] != sub);

  return 0;
}
