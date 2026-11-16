// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.mptr]/2: the nested-name-specifier denotes a class; the class
// need not be complete.
#include <cassert>

struct CompletedLater;

static int CompletedLater::*pm = nullptr;

struct CompletedLater {
  int first;
  int second;
};

int main() {
  assert(pm == nullptr);
  pm = &CompletedLater::second;
  CompletedLater obj{3, 4};
  assert(obj.*pm == 4);
  return 0;
}
