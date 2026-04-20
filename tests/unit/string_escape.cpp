#include <assert.h>
#include <string.h>

int main() {
  const char *special = "\a\b\t\n\v\f\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~";
  static const char expected[] = {
      7,  8,  9,  10, 11, 12, 13, 32, 33,  34,  35,  36,  37, 38,
      39, 40, 41, 42, 43, 44, 45, 46, 47,  58,  59,  60,  61, 62,
      63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126,
  };
  for (int i = 0; i < (int)(sizeof(expected) / sizeof(expected[0])); i++) {
    assert(special[i] == expected[i]);
  }

  return 0;
}
