// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.mem]/1: the null member pointer value is distinguishable from
// any pointer to member not created from a null pointer constant.
#include <cassert>

struct HasVirtualAndNonVirtual {
  virtual void first_virtual() {}
  void non_virtual() {}
};

int main() {
  void (HasVirtualAndNonVirtual::*null_pmf)() = nullptr;
  void (HasVirtualAndNonVirtual::*virtual_pmf)() =
      &HasVirtualAndNonVirtual::first_virtual;
  void (HasVirtualAndNonVirtual::*non_virtual_pmf)() =
      &HasVirtualAndNonVirtual::non_virtual;
  assert(virtual_pmf != null_pmf);
  assert(non_virtual_pmf != null_pmf);
  return 0;
}
