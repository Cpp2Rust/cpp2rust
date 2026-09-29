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

struct Value {
  int v;
  Value(int u) : v(u) {}
};

struct Ptr {
  Value v1;
  Value v2;

  Ptr() : v1(11), v2(22) {}
};

int main() {
  StructWithCtor struct_with_ctor(1, 2);
  int x = 3;
  assert(foo(x) == 3 && struct_with_ctor.x1() == 2 &&
         struct_with_ctor.x2() == 1);

  auto p = Ptr();
  assert(p.v1.v == 11);
  assert(p.v2.v == 22);
  return 0;
}
