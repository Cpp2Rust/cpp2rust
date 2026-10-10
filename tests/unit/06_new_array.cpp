static char take(char *&&p) {
  p[0] = 9;
  char c = p[0];
  delete[] p;
  return c;
}

int main() {
  int *e = new int[2];
  e[0] = 6;
  e[1] = 7;
  delete[] e;
  char c = take(new char[4]);
  (void)c;
  return 0;
}
