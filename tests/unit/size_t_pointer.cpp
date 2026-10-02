#include <cassert>
#include <cstddef>

static void out_param(size_t *out = nullptr) {
  if (out) {
    *out = 4;
  }
}

namespace ns {
static int parse(int v, std::size_t *idx = 0) {
  if (idx) {
    *idx = 3;
  }
  return v;
}
} // namespace ns

int main() {
  size_t v2 = 0;
  out_param(&v2);
  assert(v2 == 4);

  std::size_t pidx = 0;
  assert(ns::parse(1, &pidx) == 1);
  assert(pidx == 3);

  std::size_t sv = 7;
  std::size_t *sp = &sv;
  std::size_t **spp = &sp;
  assert(**spp == 7);
  return 0;
}
