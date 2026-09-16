// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr = new int[10];
  int out = arr[10];
  delete[] arr;
  return out;
}
