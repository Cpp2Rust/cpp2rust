int main() {
  int out = 0;
  int x = 0;
  int *p1 = &x;
  *p1 = 1;
  out *= x;
  int **p2 = &p1;
  **p2 = 2;
  out *= x;
  int ***p3 = &p2;
  ***p3 = 3;
  out *= x;
  return out;
}
