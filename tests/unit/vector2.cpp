#include <cassert>
#include <vector>

void fn(std::vector<int> &v, std::vector<int> v3) {
  v.push_back(20);
  int x;
  std::vector<int> v2, *v4 = &v3;
  v2.push_back(0);
  v2.push_back(1);
  v2.push_back(3);
  x = v[2];
  v2[0] = 1;
  (true ? v3 : v)[0] = 7;
  v2 = v;
  (*v4)[1] = 13;
  assert(x == 6);
  assert(v.front() == 4);
  assert(v[1] == 5);
  assert(v[2] == 6);
  assert(v.back() == 20);
  assert(v2[0] == 4);
  assert(v2[1] == 5);
  assert(v2[2] == 6);
  assert(v3[0] == 7);
  assert(v3[1] == 13);
  v.push_back(20);
}

int main() {
  std::vector<int> v, v2;
  v.push_back(4);
  v.push_back(5);
  v.push_back(6);
  v2.push_back(8);
  v2.push_back(9);
  fn(v, v2);
  return 0;
}
