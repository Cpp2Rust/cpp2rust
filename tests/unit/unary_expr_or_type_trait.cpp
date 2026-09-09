#include <cassert>
#include <cstdint>

struct alignas(16) S {
  char c;
  int64_t x;
};

template <typename... Args> unsigned long pack_size(Args... args) {
  return sizeof...(args) + sizeof...(Args);
}

int main() {
  int64_t arr[4] = {};
  S s{};

  assert(sizeof(int32_t) == 4);
  assert(sizeof(arr) == 32);
  assert(sizeof(s) == 16);

  assert(alignof(int32_t) == 4);
  assert(alignof(S) == 16);

  assert(__alignof(arr) == 8);
  assert(__alignof(s) == 16);

  assert(pack_size() == 0);
  assert(pack_size(1, 2.0) == 4);
  return 0;
}
