#include <cassert>

inline int inlined_free(int a) { return a + 1; }

constexpr int inlined_constexpr_free(int a) { return a + 2; }

inline int inlined_declared_free(int a);

int inlined_declared_free(int a) { return a + 3; }

int not_inlined_free(int a) { return a + 4; }

static int not_inlined_static_free(int a) { return a + 5; }

class Base {
public:
  virtual int inlined_override(int a) const = 0;

  virtual int not_inlined_override(int a) const = 0;
};

class Derived : public Base {
public:
  explicit Derived(int v) : v_(v) {}

  int inlined_override(int a) const override { return v_ + a; }

  int not_inlined_override(int a) const override;

  int inlined_in_class() const { return v_ * 2; }

  static int inlined_static_in_class(int a) { return a * 3; }

  inline int inlined_declared_in_class() const;

  int inlined_defined_out_of_line() const;

  int not_inlined_out_of_line() const;

  static int not_inlined_static_out_of_line(int a);

private:
  int v_;
};

int Derived::not_inlined_override(int a) const { return v_ - a; }

int Derived::inlined_declared_in_class() const { return v_ * 4; }

inline int Derived::inlined_defined_out_of_line() const { return v_ * 5; }

int Derived::not_inlined_out_of_line() const { return v_ * 6; }

int Derived::not_inlined_static_out_of_line(int a) { return a * 7; }

int main() {
  Derived d(3);
  const Base &b = d;
  assert(inlined_free(1) == 2);
  assert(inlined_constexpr_free(1) == 3);
  assert(inlined_declared_free(1) == 4);
  assert(not_inlined_free(1) == 5);
  assert(not_inlined_static_free(1) == 6);
  assert(b.inlined_override(1) == 4);
  assert(b.not_inlined_override(1) == 2);
  assert(d.inlined_in_class() == 6);
  assert(Derived::inlined_static_in_class(2) == 6);
  assert(d.inlined_declared_in_class() == 12);
  assert(d.inlined_defined_out_of_line() == 15);
  assert(d.not_inlined_out_of_line() == 18);
  assert(Derived::not_inlined_static_out_of_line(2) == 14);
  return 0;
}
