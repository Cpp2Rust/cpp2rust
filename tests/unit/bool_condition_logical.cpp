#include <assert.h>
#include <stddef.h>

enum Code { CODE_OK = 0, CODE_ERR = 1, CODE_FATAL = 2 };

int side_effect = 0;
int observe(int v) {
  ++side_effect;
  return v;
}

int main() {
  int n = 3;
  int zero = 0;
  int storage = 7;
  int *p = &storage;
  int *np = nullptr;
  unsigned u = 4;
  Code code = CODE_OK;

  if (n && p) {
    assert(true);
  }
  if (n && np) {
    assert(false);
  }
  if (zero || p) {
    assert(true);
  }
  if (zero || np) {
    assert(false);
  }
  if (n && u && p && code == CODE_OK) {
    assert(true);
  }

  side_effect = 0;
  if (zero && observe(1)) {
    assert(false);
  }
  assert(side_effect == 0);

  if (n || observe(1)) {
    assert(true);
  }
  assert(side_effect == 0);

  int chunk_count = 5;
  int max_chunks = 3;
  unsigned opts = 0x2u;
  if ((chunk_count > max_chunks) || (opts & 0x1u)) {
    assert(true);
  }
  if ((chunk_count < max_chunks) || (opts & 0x1u)) {
    assert(false);
  }

  unsigned a_id = 1u;
  unsigned b_id = 2u;
  unsigned other_id = 3u;
  if (((a_id != other_id)) && ((b_id != other_id))) {
    assert(true);
  }

  int reply_ms = -1;
  if ((p != nullptr) && (reply_ms < 0)) {
    assert(true);
  }

  unsigned baller_count = 2u;
  bool ballers_complete = false;
  if ((baller_count > 1u) || !ballers_complete) {
    assert(true);
  }

  if (((chunk_count > max_chunks)) || ((opts & 0x4u))) {
    assert(true);
  }

  return 0;
}
