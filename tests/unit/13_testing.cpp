int main() {
  int a = 1;
  int &r = a;
  int *p = &a;
  r = 2;
  *p = 3;

  int **p2 = &p;
  **p2 = 3;
  return 0;
}
