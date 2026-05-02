#include <assert.h>
#include <stdbool.h>

enum Code { CODE_OK = 0, CODE_ERR = 1, CODE_FATAL = 2 };

int main() {
  enum Code code = CODE_OK;
  enum Code err = CODE_ERR;

  if (code) {
    assert(false);
  }
  if (!code) {
    assert(true);
  }
  if (err) {
    assert(true);
  }
  if (!err) {
    assert(false);
  }

  int t9 = !code;
  assert(t9 == 1);

  bool b4 = code;
  assert(!b4);

  return 0;
}
