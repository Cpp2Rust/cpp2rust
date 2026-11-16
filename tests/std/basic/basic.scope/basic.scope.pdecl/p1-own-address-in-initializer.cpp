// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/1: the locus of a declaration that is a declarator
// is immediately after the complete declarator, so the initializer names the
// entity being declared.
#include <cassert>

struct SelfLinked {
  SelfLinked *next;
  int value;
};

int main() {
  SelfLinked node{nullptr, 3};
  SelfLinked *outer = &node;
  {
    SelfLinked node{&node, 7};
    assert(node.next == &node);
    assert(node.next != outer);
    assert(node.next->value == 7);
  }
  assert(node.next == nullptr);
  assert(node.value == 3);
  return 0;
}
