// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr = new int[15];
  int *ptr = arr + 15;
  int out = *ptr;
  delete[] arr;
  return out;
}
