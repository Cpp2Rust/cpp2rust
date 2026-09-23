// nondet-result: unsafe
void foo(int *array) { delete[] array; }
int main() {
  int *x = new int(1);
  foo(x);
  return 0;
}
