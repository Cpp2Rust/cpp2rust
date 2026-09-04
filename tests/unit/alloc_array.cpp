// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

void All(std::unique_ptr<int[]> &arr, int N, int element) {
  std::unique_ptr<int[]> all = std::make_unique<int[]>(N);
  for (int i = 0; i < N; ++i)
    all[i] = element;
  arr = std::move(all);
}

int Consume(std::unique_ptr<int[]> arr, int N) {
  int sum = 0;
  int i = -1;
  while (++i < N)
    sum += arr[i];
  return sum;
}

int main() {
  const int N = 10;
  std::unique_ptr<int[]> arr = std::make_unique<int[]>(N);
  All(arr, N, 1);
  assert(Consume(std::move(arr), N) == 10);
  return 0;
}
