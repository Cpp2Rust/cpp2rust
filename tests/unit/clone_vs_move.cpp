#include <assert.h>
#include <map>
#include <memory>
#include <string>
#include <vector>

struct Bar {
  int w;
};

struct Foo {
  int x;
  int &y;
  int *z;
  int a[3];
  Bar bar;
};

int main() {
  int x1 = 1;
  int x2 = x1;
  ++x2;
  assert(x1 == 1);
  assert(x2 == 2);

  double x3 = 3.0;
  double x4 = x3;
  ++x4;
  assert(x3 == 3.0);
  assert(x4 == 4.0);

  int &reference = x1;
  int x5 = reference;
  ++x5;
  assert(reference == 1);
  assert(x5 == 2);

  int *pointer = &x1;
  int x6 = *pointer;
  ++x6;
  assert(*pointer == 1);
  assert(x6 == 2);

  int *other_pointer = pointer;
  assert(other_pointer == pointer);
  ++*other_pointer;
  assert(*other_pointer == *pointer);

  Foo f1{1, x1, &x1, {0, 1, 2}, {10}};
  assert(f1.x == 1);
  assert(f1.y == 2);
  assert(f1.z == &x1);
  assert(*f1.z == 2);

  Foo f2 = f1;
  ++f2.x;
  ++f2.y;
  assert(f2.x == 2);
  assert(f2.y == 3);
  assert(f1.x == 1);
  assert(f1.y == 3);

  ++*f2.z;
  assert(f2.y == 4);
  assert(f2.z == &x1);
  assert(*f2.z == 4);
  assert(f1.y == 4);
  assert(f1.z == &x1);
  assert(*f1.z == 4);

  ++f2.a[0];
  ++f2.a[1];
  ++f2.a[2];
  assert(f2.a[0] == 1);
  assert(f2.a[1] == 2);
  assert(f2.a[2] == 3);
  assert(f1.a[0] == 0);
  assert(f1.a[1] == 1);
  assert(f1.a[2] == 2);

  f2.bar.w = 20;
  assert(f2.bar.w == 20);
  assert(f1.bar.w == 10);

  int N = 5;
  std::vector<int> v1;
  for (int i = 0; i < N; ++i) {
    v1.push_back(i);
  }
  std::vector<int> v2 = v1;
  for (int i = 0; i < N; ++i) {
    assert(v2[i] == i);
  }
  for (int i = 0; i < N; ++i) {
    ++v2[i];
  }
  for (int i = 0; i < N; ++i) {
    assert(v2[i] == i + 1);
    assert(v1[i] == i);
  }
  std::vector<std::vector<int>> m1;
  for (int i = 0; i < N; ++i) {
    m1.push_back(std::vector<int>(10));
  }
  std::vector<std::vector<int>> m2 = m1;
  for (int i = 0; i < N; ++i) {
    assert(m1[i].size() == 10);
    assert(m2[i].size() == 10);
    for (int j = 0; j < 10; ++j) {
      assert(m1[i][j] == 0);
      assert(m2[i][j] == 0);
    }
  }
  for (int i = 0; i < N; ++i)
    for (int j = 0; j < 10; ++j)
      m2[i][j]++;
  for (int i = 0; i < N; ++i) {
    assert(m1[i].size() == 10);
    assert(m2[i].size() == 10);
    for (int j = 0; j < 10; ++j) {
      assert(m1[i][j] == 0);
      assert(m2[i][j] == 1);
    }
  }
  std::map<int, int> map1;
  for (int i = 0; i < N; ++i)
    map1[i] = i;
  std::map<int, int> map2 = map1;
  for (int i = 0; i < N; ++i) {
    assert(map2[i] == i);
    ++map2[i];
  }
  for (int i = 0; i < N; ++i) {
    assert(map1[i] == i);
    assert(map2[i] == i + 1);
  }

  std::pair<int, int> pair1(1, 2);
  std::pair<int, int> pair2 = pair1;
  pair2.first = pair2.first * 10;
  pair2.second = pair2.second * 10;
  assert(pair2.first == 10);
  assert(pair2.second == 20);
  assert(pair1.first == 1);
  assert(pair1.second == 2);
  std::pair<std::vector<int>, int> pair3(std::vector<int>(0), 0);
  auto pair4 = pair3;
  pair4.first.push_back(1);
  pair4.second = 1;
  assert(pair4.first.size() == 1);
  assert(pair4.second == 1);
  assert(pair3.first.size() == 0);
  assert(pair3.second == 0);

  std::string s1(3, 'a');
  std::string s2 = s1;
  s2[0] = 'b';
  s2[1] = 'b';
  s2[2] = 'b';
  assert(s2[0] == 'b');
  assert(s2[1] == 'b');
  assert(s2[2] == 'b');
  assert(s1[0] == 'a');
  assert(s1[1] == 'a');
  assert(s1[2] == 'a');

  Bar b1{1};
  Bar b2{2};
  b2 = b1;
  b2.w++;
  assert(b1.w == 1);
  assert(b2.w == 2);

  std::vector<int> v4;
  v4 = v2;
  for (int i = 0; i < N; ++i) {
    assert(v4[i] == i + 1);
    ++v4[i];
  }
  for (int i = 0; i < N; ++i) {
    assert(v4[i] == i + 2);
    assert(v2[i] == i + 1);
  }

  return 0;
}
