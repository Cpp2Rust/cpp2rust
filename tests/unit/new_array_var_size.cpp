int main() {
  int N = 5;
  int *A = new int[N];
  delete[] A;
  int &N2 = N;
  int *A2 = new int[N2];
  delete[] A2;
  return 0;
}
