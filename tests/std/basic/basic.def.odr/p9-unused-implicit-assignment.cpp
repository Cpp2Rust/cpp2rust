// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.def.odr]/9: an assignment operator is odr-used by an
// implicitly-defined assignment operator of another class.
// HasMemberWithUndefinedCopyAssignment is never assigned, so its assignment
// operator is never implicitly defined and
// HasUndefinedCopyAssignment::operator= is never defined.
#include <cassert>

static int m_ctors;

struct HasUndefinedCopyAssignment {
  HasUndefinedCopyAssignment() { ++m_ctors; }
  HasUndefinedCopyAssignment &operator=(const HasUndefinedCopyAssignment &);
};

struct HasMemberWithUndefinedCopyAssignment {
  HasUndefinedCopyAssignment m;
};

int main() {
  HasMemberWithUndefinedCopyAssignment a;
  HasMemberWithUndefinedCopyAssignment b;
  assert(m_ctors == 2);
  return 0;
}
