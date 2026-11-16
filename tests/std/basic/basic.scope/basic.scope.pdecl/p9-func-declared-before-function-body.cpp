// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.pdecl]/9: the locus of the implicit declaration of a
// function-local predefined variable is immediately before the function-body
// of its function's definition.
#include <cassert>
#include <cstring>

struct RecordsConstructorName {
  const char *name;

  RecordsConstructorName() : name(__func__) { assert(name == __func__); }
};

const char *plain_function() { return __func__; }

int main() {
  assert(std::strcmp(plain_function(), "plain_function") == 0);
  assert(plain_function() == plain_function());

  RecordsConstructorName object;
  assert(std::strcmp(object.name, "RecordsConstructorName") == 0);

  const char *enclosing = __func__;
  auto lambda = [] { return __func__; };
  assert(lambda() != enclosing);
  assert(std::strcmp(enclosing, "main") == 0);
  return 0;
}
