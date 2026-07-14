#include <assert.h>
#include <stdint.h>

int main() {
  char arr[] = {1, 2, 3};
  {
    char *p = &arr[0];
    p += static_cast<uint8_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<int8_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<uint16_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<int16_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<uint32_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<int32_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<uint64_t>(1);
    assert(*p == 2);
  }

  {
    char *p = &arr[0];
    p += static_cast<int64_t>(1);
    assert(*p == 2);
  }

  return 0;
}
