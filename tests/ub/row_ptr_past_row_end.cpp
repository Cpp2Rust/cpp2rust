// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int rows[2][3] = {{1, 2, 3}, {4, 5, 6}};
  int *p = rows[0];
  return p[3];
}
