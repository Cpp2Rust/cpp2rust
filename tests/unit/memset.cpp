#include <cassert>
#include <cstring>

int main() {
  const int N = 3;
  int *arr = new int[N];
  memset(arr, 1, sizeof(int) * N);
  int sum = 0;
  for (int i = 0; i < N; ++i)
    sum += arr[i];
  delete[] arr;
  assert(sum == 50529027);

  // The value is computed once, before it is written in a loop.
  unsigned char bytes[4];
  unsigned char fill = 7;
  int n = 2;
  memset(&bytes[0], fill + 1, n);
  memset(&bytes[n], fill, 4 - n);
  assert(bytes[0] == 8 && bytes[1] == 8 && bytes[2] == 7 && bytes[3] == 7);
  return 0;
}
