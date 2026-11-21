// ADDITIONAL_COMPILE_FLAGS: -std=c++17

#include <cstdio>
#include <map>
#include <vector>
using Map = std::map<int, std::vector<int>>;
Map *receiver(Map *m, std::vector<int> *v, int *calls) {
  ++*calls;
  (*v)[2] = 9;
  return m;
}
Map::iterator hint(Map *m, std::vector<int> *v, int *calls) {
  ++*calls;
  (*v)[2] = 7;
  return m->end();
}
int main() {
  for (int select_receiver = 1; select_receiver >= 0; --select_receiver) {
    Map m;
    std::vector<int> v{1, 2, 3};
    int a = 0, b = 0;
    if (select_receiver)
      receiver(&m, &v, &a)->emplace_hint(hint(&m, &v, &b), 1, v);
    else
      m.emplace_hint(hint(&m, &v, &b), 1, v);
    const std::vector<int> &inserted = m.at(1);
    std::printf("receiver=%d hint=%d source=%d inserted=%d\n", a, b, v[2],
                inserted[2]);
  }
}
