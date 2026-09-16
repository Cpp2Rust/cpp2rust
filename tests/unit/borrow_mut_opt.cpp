void convert_without_rhs() {
  int x = 0;
  int y = 1;
  x = 0;
  x = y + 1;
  y = 0;
  y = x + 1;
  x += 1;
  y += 1;
  y = 0;
  x = 0;
  int z = x + y;
  z = x + y + 1;
  int arr[2] = {1, 2};
  [[maybe_unused]] int w = arr[y] + arr[x];
  w += z + y + x;
  char arr2[3] = {'a', 'b', 'c'};
  const int *p1 = &x;
  [[maybe_unused]] char c = arr2[*p1];
  c = arr2[*p1];
  int *p2 = &x;
  *p2 = 1;
  int &r = x;
  r = 1;
}
void convert_with_rhs() {
  int x = 0;
  x = x + 1;
  int y = 0;
  y = y + 1;
  int arr[2] = {1, 2};
  arr[y] = y + 1;
  arr[x] = x + 1;
  arr[x] = arr[y] + 1;
  int &z = x;
  x += z;
  y += z;
  int *p = &x;
  x += *p;
  y += *p;
  p = &arr[0];
  arr[0] = *p;
  z += x;
  z += y;
  z += *p;
  *p += y + x;
  *p += x + z;
  *p += y + z;
}
int main() {
  convert_without_rhs();
  convert_with_rhs();
  return 0;
}
