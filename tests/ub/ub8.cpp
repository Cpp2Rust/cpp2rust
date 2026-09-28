// nondet-result: refcount, unsafe
int main() {
  int x = 5;
  int *p = &x;
  return *++p;
}
