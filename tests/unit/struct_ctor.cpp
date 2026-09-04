// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct StructWithCtor {
private:
  int x1_, x2_;

public:
  StructWithCtor(int x1, int x2) : x1_(x1), x2_(x2) {
    ++this->x1_;
    --this->x2_;
  }
  const int &x1() const { return x1_; }
  const int &x2() const { return x2_; }
};
int &foo(int &x) { return x; }
int main() {
  StructWithCtor struct_with_ctor(1, 2);
  int x = 3;
  assert(foo(x) == 3 && struct_with_ctor.x1() == 2 &&
         struct_with_ctor.x2() == 1);
  return 0;
}
