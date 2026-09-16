// panic-ub: refcount
// nondet-result: unsafe
void null(int **p) { *p = nullptr; }
int main() {
  int x = 1;
  int *p = &x;
  null(&p);
  int &r = *p;
  return r;
}
