// panic-ub: refcount
// nondet-result: unsafe
int *second_row() {
  int rows[2][3] = {{1, 2, 3}, {4, 5, 6}};
  return rows[1];
}
int main() {
  int *p = second_row();
  return p[0];
}
