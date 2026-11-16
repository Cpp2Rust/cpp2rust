// translation-fail: refcount, unsafe
// nondet-result
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/4.3: if either operand is a pointer to a virtual member
// function, the result is unspecified.

struct DeclaresVirtual {
  virtual void run() {}
};

int main() {
  void (DeclaresVirtual::*first_pmf)() = &DeclaresVirtual::run;
  void (DeclaresVirtual::*second_pmf)() = &DeclaresVirtual::run;
  return first_pmf == second_pmf;
}
