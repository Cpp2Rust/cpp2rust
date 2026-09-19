// ADDITIONAL_COMPILE_FLAGS: -std=c++23

#include <cassert>
#include <cstddef>

constexpr const char *get(const char *s) { return s; }
constexpr const wchar_t *get(const wchar_t *s) { return s; }
constexpr const char8_t *get(const char8_t *s) { return s; }
constexpr const char16_t *get(const char16_t *s) { return s; }
constexpr const char32_t *get(const char32_t *s) { return s; }

constexpr char second(const char (&s)[3]) { return s[1]; }
constexpr wchar_t second(const wchar_t (&s)[3]) { return s[1]; }
constexpr char8_t second(const char8_t (&s)[3]) { return s[1]; }
constexpr char16_t second(const char16_t (&s)[3]) { return s[1]; }
constexpr char32_t second(const char32_t (&s)[3]) { return s[1]; }

int main() {
  const char *c = "A";
  const wchar_t *w = L"AĂ";
  const char8_t *b = u8"Ă";
  const char16_t *s = u"AĂ";
  const char32_t *l = U"AĂ";

  assert(c[0] == 'A');
  assert(b[0] == 0xC4 && b[1] == 0x82);
  assert(w[1] == 0x102);
  assert(s[1] == 0x102);
  assert(l[1] == 0x102);
  assert(w[2] == 0 && s[2] == 0 && l[2] == 0);

  assert(get("A")[0] == 'A');
  assert(get(L"Ă")[0] == 0x102);
  assert(get(u8"Ă")[0] == 0xC4);
  assert(get(u"Ă")[0] == 0x102);
  assert(get(U"Ă")[0] == 0x102);

  assert(second("AB") == 'B');
  assert(second(L"AĂ") == 0x102);
  assert(second(u8"Ă") == 0x82);
  assert(second(u"AĂ") == 0x102);
  assert(second(U"AĂ") == 0x102);

  const std::size_t nw = sizeof(L"abc") / sizeof(wchar_t) - 1;
  const std::size_t nb = sizeof(u8"abc") / sizeof(char8_t) - 1;
  const std::size_t ns = sizeof(u"abc") / sizeof(char16_t) - 1;
  const std::size_t nl = sizeof(U"abc") / sizeof(char32_t) - 1;
  assert(nw == 3 && nb == 3 && ns == 3 && nl == 3);

  const wchar_t pw[4] = L"Ă";
  const char16_t ps[4] = u"Ă";
  assert(pw[0] == 0x102 && pw[1] == 0 && pw[3] == 0);
  assert(ps[0] == 0x102 && ps[1] == 0 && ps[3] == 0);

  const wchar_t ew[2] = L"";
  assert(ew[0] == 0 && ew[1] == 0);
  return 0;
}
