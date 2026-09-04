// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct Item {
  int value;

  void foo(Item *other) { other->value = 10; }
};

int main() {
  Item *arr = new Item[2];
  arr[0].value = 1;
  arr[1].value = 2;
  arr[0].foo(&arr[1]);
  int result = arr[0].value + arr[1].value;
  delete[] arr;
  assert(result == 11);
  return 0;
}
