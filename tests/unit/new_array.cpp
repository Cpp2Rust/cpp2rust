static int sum(int *p, int n) {
  int total = 0;
  for (int i = 0; i < n; ++i) {
    total += p[i];
  }
  return total;
}

int main() {
  int *array = new int[100];
  delete[] array;

  int *filled = new int[4];
  for (int i = 0; i < 4; ++i) {
    filled[i] = i + 1;
  }
  if (sum(filled, 4) != 10) {
    return 1;
  }
  delete[] filled;
  return 0;
}
