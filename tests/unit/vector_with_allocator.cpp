// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <vector>

template <typename T> struct TestAllocator {
  using value_type = T;

  T *allocate(std::size_t n) { return new T[n]; }

  void deallocate(T *p, std::size_t /*n*/) { delete[] p; }
};

void copy(std::vector<int, TestAllocator<int>> copy_vector) {}

void fn(std::vector<int, TestAllocator<int>> &v,
        std::vector<int, TestAllocator<int>> v3) {
  v.push_back(20);
  int x;
  std::vector<int, TestAllocator<int>> *v4 = &v3;
  std::vector<int, TestAllocator<int>> v2;
  v2.push_back(0);
  v2.push_back(1);
  v2.push_back(3);
  x = v[2];
  v2[0] = 1;
  (true ? v3 : v)[0] = 7;
  (*v4)[1] = 13;
  assert(x == 6);
  assert(v.front() == 4);
  assert(v[1] == 5);
  assert(v[2] == 6);
  assert(v.back() == 20);
  assert(v3[0] == 7);
  assert(v3[1] == 13);
  v.push_back(20);
}

int main() {
  std::vector<int, TestAllocator<int>> v1;
  assert(v1.size() == 0);
  assert(v1.empty());
  v1.push_back(1);
  assert(!v1.empty());
  v1.pop_back();
  assert(v1.empty());

  auto s1 = v1.size();
  v1.resize(100);
  assert(v1.size() == 100);
  assert(v1[99] == 0);
  v1[0] = 40;
  v1[99] = 50;
  assert(v1[0] == 40);
  assert(v1[99] == 50);

  std::vector<int, TestAllocator<int>> v2;
  assert(v2.size() == 0);
  v2.push_back(1);
  v2.push_back(2);
  v2.push_back(3);
  assert(v2.size() == 3);
  v2.erase(v2.begin());
  assert(v2.size() == 2);
  assert(v2[0] == 2);
  assert(v2[1] == 3);
  v2.insert(v2.begin(), 100);
  copy(v2);
  assert(v2.size() == 3);
  assert(v2[0] == 100);
  assert(v2[1] == 2);
  assert(v2[2] == 3);

  auto s2 = v2.size();
  std::vector<int, TestAllocator<int>> v3(100, 1);
  assert(v3.size() == 100);
  for (int i = 0; i < 100; ++i)
    assert(v3[i] == 1);

  std::vector<double, TestAllocator<double>> v6(s2, 2.0);
  assert(v6.size() == s2);
  for (unsigned i = 0; i < s2; ++i)
    assert(v6[i] == 2.0);

  const double *p1 = v6.data();
  assert(*p1 == 2.0);

  int *p2 = v3.data();
  assert(*p2 == 1);
  assert(v3[0] == 1);
  assert(v3[1] == 1);
  *p2 = 99.0;
  assert(*p2 == 99);
  assert(v3[0] == 99);
  assert(v3[1] == 1);
  ++p2;
  *p2 = 98;
  assert(v3[0] == 99);
  assert(v3[1] == 98);
  assert(v3.capacity() == 100);
  assert(v3.size() == 100);
  v3.reserve(200);
  assert(v3.capacity() == 200);
  assert(v3.size() == 100);
  v3.reserve(50);
  assert(v3.capacity() == 200);
  assert(v3.size() == 100);
  v3.reserve(200);
  assert(v3.capacity() == 200);
  assert(v3.size() == 100);
  v3.reserve(201);
  assert(v3.capacity() == 201);
  assert(v3.size() == 100);
  assert(v2.back() == 3);
  assert(v3.back() == 1);
  assert(v6.back() == 2.0);

  double &ref0 = v6.back();
  ref0 = 5.0;
  assert(v6.back() == 5.0);

  double x0 = v6.back();
  assert(x0 == 5.0);
  x0 = 6.0;
  assert(v6.back() == 5.0);

  int idx = 0;
  assert(v6.at(idx) == 2.0);
  assert(v6.at(s2 - 1) == 5.0);

  double &ref1 = v6.at(s2 - 1);
  ref1 += 1.5;
  assert(v6.at(s2 - 1) == 6.5);

  double x1 = v6.at(s2 - 1);
  assert(x1 == 6.5);
  x1 -= 1.5;
  assert(v6.at(s2 - 1) == 6.5);

  std::vector<int, TestAllocator<int>> v7;
  std::vector<int, TestAllocator<int>> v8;
  v7.push_back(4);
  v7.push_back(5);
  v7.push_back(6);
  v8.push_back(8);
  v8.push_back(9);
  fn(v7, v8);

  uint32_t src[3] = {1, 2, 3};

  // Same-type iterator-range constructor.
  std::vector<uint32_t> v9(src, src + 3);
  assert(v9.size() == 3);
  assert(v9[0] == 1 && v9[1] == 2 && v9[2] == 3);

  // Cross-type widening (uint32_t -> size_t).
  std::vector<size_t> v10(src, src + 3);
  assert(v10.size() == 3);
  assert(v10[0] == 1 && v10[1] == 2 && v10[2] == 3);

  // Cross-type sign-flip (uint32_t -> int).
  std::vector<int> v11(src, src + 3);
  assert(v11.size() == 3);
  assert(v11[0] == 1 && v11[1] == 2 && v11[2] == 3);

  const uint32_t src1[3] = {1, 2, 3};
  auto v12 = std::vector<uint32_t>(src1, std::end(src1));
  assert(v12.size() == 3);
  assert(v12[0] == 1 && v12[1] == 2 && v12[2] == 3);

  uint8_t buf[5] = {10, 20, 30, 40, 50};
  const uint8_t *start = buf;
  size_t len = 5;
  std::vector<uint8_t> v13(start, start + len);
  assert(v13.size() == 5);
  assert(v13[0] == 10 && v13[4] == 50);

  return s1 + s2 + v2.at(0);
}
