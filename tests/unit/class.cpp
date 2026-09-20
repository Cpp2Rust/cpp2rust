#include <cassert>

class Pair {
public:
  int first;
  int second;

  void NOP() {}

  int GetFirst() const { return first; }

  int GetSecond() const { return second; }

  int Set(int &field, int new_val) {
    NOP();
    int old_val = field;
    field = new_val;
    return old_val;
  }

  int SetFirst(int new_first) { return GetFirst() + Set(first, new_first); }

  int SetSecond(int new_second) {
    return GetSecond() + Set(second, new_second);
  }
};

struct Route {
  Pair path;
  double cost;

  double SetCost(double new_cost) {
    double old_cost = cost;
    cost = new_cost;
    return old_cost;
  }
};

struct Counter {
  int v;
  mutable int calls;

  int Get() const {
    ++calls;
    return v;
  }

  bool operator==(const Counter &o) const {
    ++calls;
    return v == o.v;
  }
};

int RandomRoute(Route &route) {
  if (route.path.first % 2) {
    return route.path.SetFirst(route.path.SetSecond(10));
  } else {
    return route.path.SetSecond(route.path.SetFirst(-10));
  }
}

int main() {
  Route route1 = {{0, 1}, 5};
  Route route2 = {{1, 0}, 10};
  double old_cost = route1.SetCost(route2.SetCost(15));
  assert(RandomRoute(route1) + RandomRoute(route2) + old_cost == 9);
  Counter c1{3, 0};
  const Counter c2{3, 0};
  const Counter *pc = &c1;
  assert(c1.Get() == 3);
  assert(c2.Get() == 3);
  assert(pc->Get() == 3);
  assert(c1 == c2);
  assert(c2 == c1);
  assert(c1.calls == 3);
  assert(c2.calls == 2);
  return 0;
}
