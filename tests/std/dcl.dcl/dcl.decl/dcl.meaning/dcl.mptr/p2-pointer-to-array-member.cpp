// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.mptr]/2: the type is "derived-declarator-type-list pointer to
// member of class C of type T"; T can be an array type.
#include <cassert>

struct HasIntArray {
  int before;
  int elements[3];
};

int main() {
  int (HasIntArray::*pm)[3] = &HasIntArray::elements;
  HasIntArray obj{1, {2, 3, 4}};
  assert((obj.*pm)[0] == 2);
  assert((obj.*pm)[2] == 4);
  (obj.*pm)[1] = 7;
  assert(obj.elements[1] == 7);
  assert(obj.before == 1);
  return 0;
}
