// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int rows[2][3] = {{1, 2, 3}, {4, 5, 6}};
  return rows[1] - (&rows[0][0] + 2);
}
