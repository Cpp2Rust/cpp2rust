// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Tests deref of unique_ptr field through const raw pointer (read and write).
#include <cassert>
#include <memory>

struct Holder {
  std::unique_ptr<int> val;
};

int read_val(const Holder *h) { return *h->val; }

void write_val(const Holder *h, int v) { *h->val = v; }

int main() {
  Holder h;
  h.val = std::make_unique<int>(10);
  write_val(&h, 42);
  assert(read_val(&h) == 42);
  return 0;
}
