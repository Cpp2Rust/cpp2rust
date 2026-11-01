#include <cassert>
#include <cstdio>
#include <string>

void append_to_reference(std::string &s) { s.append(1, 'r'); }

int main() {
  const std::size_t lengths[] = {0, 1, 2, 3, 7, 8, 15, 16, 31, 64};
  const std::size_t counts[] = {0, 1, 2, 3, 8, 33};
  const char characters[] = {0,  1,  2,  7,   9,   10,   13,   31,  32, 42,
                             65, 90, 97, 100, 127, -128, -127, -64, -2, -1};
  const char initial[] = {'a', 0, 'b', -128, -1};
  std::size_t cases = 0;
  for (std::size_t length : lengths) {
    for (std::size_t count : counts) {
      for (char character : characters) {
        for (int shrink = 0; shrink < 2; ++shrink) {
          std::string s(length, 'x');
          for (std::size_t i = 0; i < length; ++i)
            s[i] = initial[i % sizeof(initial)];
          if (shrink)
            s.shrink_to_fit();
          s.append(count, character);
          assert(s.size() == length + count);
          const char *data = s.data();
          for (std::size_t i = 0; i < length + count; ++i)
            assert(data[i] ==
                   (i < length ? initial[i % sizeof(initial)] : character));
          assert(data[s.size()] == 0);
          ++cases;
        }
      }
    }
  }
  std::string unboxed = "a";
  unboxed.append(2, 'b');
  assert(unboxed == "abb");
  std::string indirect = "c";
  append_to_reference(indirect);
  assert(indirect == "cr");
  std::string *pointer = &indirect;
  pointer->append(std::size_t(0), '\0');
  assert(indirect == "cr");
  printf("checked %zu appends\n", cases);
  return 0;
}
