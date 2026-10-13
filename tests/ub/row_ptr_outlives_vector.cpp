// panic-ub: refcount
// nondet-result: unsafe
#include <vector>

int *second_row() {
  std::vector<std::vector<int>> rows = {{1, 2, 3}, {4, 5, 6}};
  return rows[1].data();
}
int main() {
  int *p = second_row();
  return p[0];
}
