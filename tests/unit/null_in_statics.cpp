#include <cassert>
#include <cstddef>

static int *p_mut;
static const int *p_const;
static const char *cp;
static int *arr_of_ptr[4];
static int **pp;
static const int *const_arr_of_ptr[3];
static const char *cp_explicit_null = nullptr;
static int *p_zero = 0;

int main() {
  assert(p_mut == nullptr);
  assert(p_const == nullptr);
  assert(cp == nullptr);
  for (int i = 0; i < 4; ++i) {
    assert(arr_of_ptr[i] == nullptr);
  }
  assert(pp == nullptr);
  for (int i = 0; i < 3; ++i) {
    assert(const_arr_of_ptr[i] == nullptr);
  }
  assert(cp_explicit_null == nullptr);
  assert(p_zero == nullptr);
  return 0;
}
