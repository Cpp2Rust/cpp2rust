// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/2.13: the names of the members of an anonymous
// union are bound in the union's parent scope.
#include <cassert>

static union {
  int namespace_count;
  double namespace_ratio;
};

struct HasAnonymousUnion {
  int tag;
  union {
    int member_count;
    double member_ratio;
  };

  int read_count() const { return member_count; }
};

int main() {
  namespace_count = 4;
  assert(namespace_count == 4);
  namespace_ratio = 0.25;
  assert(namespace_ratio == 0.25);

  HasAnonymousUnion object;
  object.tag = 1;
  object.member_count = 6;
  assert(object.member_count == 6);
  assert(object.read_count() == 6);
  object.member_ratio = 0.5;
  assert(object.member_ratio == 0.5);
  assert(object.tag == 1);

  union {
    int block_count;
    double block_ratio;
  };
  block_count = 9;
  assert(block_count == 9);
  block_ratio = 0.75;
  assert(block_ratio == 0.75);
  return 0;
}
