// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
#include <memory>

struct Pair {
  int &x1, &x2;
};

Pair mkPair(int &x1, int &x2) { return Pair{x1, x2}; }

void fill(std::unique_ptr<int *[]> &arr, int &n1) {
  int n2 = n1;
  Pair pair = mkPair(n1, n2);
  arr[0] = &pair.x1;
  arr[1] = &pair.x2;
}

bool any(std::unique_ptr<int *[]> &arr, int &n1) {
  bool out = false;
  for (int i = 0; i < n1; ++i)
    out = out || *arr[i] == 0;
  return out;
}

int main() {
  int n = 2;
  std::unique_ptr<int *[]> arr = std::make_unique<int *[]>(n);
  fill(arr, n);
  return any(arr, n);
}
