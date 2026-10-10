#include <cassert>
#include <cstdint>

enum Flag : bool { FLAG_OFF = false, FLAG_ON = true };

enum Small : unsigned char { SMALL_ZERO = 0, SMALL_MAX = 255 };

enum Signed : signed char { SIGNED_MIN = -128, SIGNED_ONE = 1 };

enum Wide : long long { WIDE_NEG = -1, WIDE_BIG = 1LL << 40 };

enum Sized : uint16_t { SIZED_A = 1, SIZED_B = 0xFFFF };

enum class Scoped : int8_t { NEG = -2, POS = 2 };

enum Empty : bool {};

struct HoldsEmpty {
  Empty e{};
};

int main() {
  Flag flag = FLAG_ON;
  assert(flag);
  assert(!FLAG_OFF);
  flag = FLAG_OFF;
  assert(flag == FLAG_OFF);
  bool b = flag;
  assert(b == false);

  Small s = SMALL_MAX;
  assert(s == 255);
  assert(sizeof(s) == 1);
  s = SMALL_ZERO;
  assert(s + 1 == 1);

  Signed sg = SIGNED_MIN;
  assert(sg == -128);
  assert(sg < SIGNED_ONE);

  Wide w = WIDE_BIG;
  assert(w == (1LL << 40));
  assert(sizeof(w) == 8);
  assert(WIDE_NEG < 0);

  Sized z = SIZED_B;
  assert(z == 0xFFFF);
  assert(sizeof(z) == 2);
  assert(SIZED_A + 1 == 2);

  Scoped sc = Scoped::NEG;
  assert(static_cast<int>(sc) == -2);
  assert(static_cast<int8_t>(Scoped::POS) == 2);

  HoldsEmpty he;
  assert(!he.e);
  Empty ev{};
  assert(ev == he.e);
  return 0;
}
