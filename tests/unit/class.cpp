// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

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
  return 0;
}
