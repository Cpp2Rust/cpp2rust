#include <cstdio>
#include <cstring>
#include <vector>

struct Inner {
  std::vector<int> values;
};

struct Outer {
  std::vector<int> values;
  Inner inner;
  char name[8];
};

static Outer make(int &calls) {
  ++calls;
  return {{1, 2, 3}, {{4, 5, 6}}, "abc"};
}

static int sum(const std::vector<int> &values) {
  int total = 0;
  for (int value : values) {
    total += value;
  }
  return total;
}

int main() {
  int calls = 0;
  int member = sum(make(calls).values);
  int nested = sum(make(calls).inner.values);
  int element = make(calls).values.at(calls - 2);
  std::size_t length = strlen(make(calls).name);
  int aggregate = sum(Outer{{7}, {{8, 9}}, "x"}.inner.values);
  printf("%d %d %d %zu %d %d\n", member, nested, element, length, aggregate,
         calls);
  return 0;
}
