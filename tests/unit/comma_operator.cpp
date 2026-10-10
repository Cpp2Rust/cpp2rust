#include <cassert>
#include <vector>

struct Pair {
  int first;
  int second;
};

long twice(const long &v) { return v * 2; }

int sum(const Pair &p) { return p.first + p.second; }

int main() {
  int x = 1;
  int y = (x = 2, x + 1);
  assert(x == 2);
  assert(y == 3);

  int z = (1, 2, 3);
  assert(z == 3);

  int counter = 0;
  int w = (counter++, counter++, counter);
  assert(counter == 2);
  assert(w == 2);

  int a = 0, b = 0;
  if ((a = 1, b = 2, a + b > 0)) {
    assert(a == 1);
    assert(b == 2);
  }

  std::vector<int> v1 = {1, 2};
  std::vector<int> v2 = (a = 5, v1);
  v2.push_back(3);
  assert(a == 5);
  assert(v1.size() == 2);
  assert(v2.size() == 3);

  Pair p1{1, 2};
  Pair p2 = (b = 6, p1);
  p2.first = 10;
  assert(p1.first == 1);
  assert(p2.first == 10);

  assert(twice(a) == 10);
  assert(sum((a = 7, p1)) == 3);
  assert(a == 7);

  return 0;
}
