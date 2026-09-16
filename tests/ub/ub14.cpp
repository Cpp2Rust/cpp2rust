// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr1 = new int[100];
  arr1[100] = 1;
  delete[] arr1;
  return 0;
}
