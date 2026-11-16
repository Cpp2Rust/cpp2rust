// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [func.require]/1.3: INVOKE(f, t1, t2) is ((*t1).*f)(t2) when f is a
// pointer to a member function and t1 is neither a T nor a reference_wrapper.
#include <cassert>
#include <functional>

struct AccumulatesCalls {
  int total = 0;
  int add(int amount) {
    total += amount;
    return total;
  }
};

struct InheritsAccumulator : AccumulatesCalls {};

int main() {
  int (AccumulatesCalls::*pmf)(int) = &AccumulatesCalls::add;
  AccumulatesCalls obj;
  assert(std::invoke(pmf, &obj, 3) == 3);
  assert(obj.total == 3);
  return 0;
}
