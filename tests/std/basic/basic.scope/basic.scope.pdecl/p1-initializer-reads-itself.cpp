// nondet-result
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/1: the locus of a declaration that is a declarator
// is immediately after the complete declarator, so the initializer accesses
// the variable being declared outside its lifetime.

int main() {
  unsigned char level = 12;
  {
    unsigned char level = level;
    return level;
  }
}
