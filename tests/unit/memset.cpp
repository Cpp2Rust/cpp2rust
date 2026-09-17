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
  return 0;
}
