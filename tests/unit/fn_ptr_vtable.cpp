// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef void *(*create_fn)(int);
typedef int (*get_fn)(void *);
typedef void (*destroy_fn)(void *);

struct Vtable {
  create_fn create;
  get_fn get;
  destroy_fn destroy;
};

static int storage;

void *int_create(int val) {
  storage = val;
  return &storage;
}

int int_get(void *p) { return *(int *)p; }

void int_destroy(void *p) { *(int *)p = 0; }

int main() {
  Vtable vt = {int_create, int_get, int_destroy};

  assert(vt.create != nullptr);
  assert(vt.get != nullptr);
  assert(vt.destroy != nullptr);

  void *obj = vt.create(42);
  assert(vt.get(obj) == 42);

  vt.destroy(obj);
  assert(storage == 0);

  vt.get = nullptr;
  assert(vt.get == nullptr);

  return 0;
}
