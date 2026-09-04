// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

std::unique_ptr<std::unique_ptr<int[]>[]> matalloc(int n, int p, int e) {
  std::unique_ptr<std::unique_ptr<int[]>[]> m =
      std::make_unique<std::unique_ptr<int[]>[]>(n);
  for (int i = 0; i < n; ++i) {
    m[i] = std::make_unique<int[]>(p);
    for (int j = 0; j < p; ++j) {
      m[i][j] = e;
    }
  }
  return m;
}

std::unique_ptr<std::unique_ptr<int[]>[]>
matmul(std::unique_ptr<std::unique_ptr<int[]>[]> m1, int n1, int p1,
       std::unique_ptr<std::unique_ptr<int[]>[]> m2, int n2, int p2) {
  std::unique_ptr<std::unique_ptr<int[]>[]> m3 = matalloc(n1, p2, 0);
  for (int i = 0; i < n1; ++i) {
    for (int j = 0, sum = 0; j < p2; ++j) {
      for (int k = 0; k < p1; ++k) {
        sum += m1[i][k] * m2[k][j];
      }
      m3[i][j] = sum;
    }
  }
  return m3;
}

int main() {
  int n = 1, p = 10;
  std::unique_ptr<std::unique_ptr<int[]>[]> m1 = matalloc(n, p, 1);
  std::unique_ptr<std::unique_ptr<int[]>[]> m2 = matalloc(p, n, 2);
  std::unique_ptr<std::unique_ptr<int[]>[]> m3 =
      matmul(std::move(m1), n, p, std::move(m2), p, n);
  assert(m3[0][0] == 20);
  return 0;
}
