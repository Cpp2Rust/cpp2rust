#include <cassert>
#include <cstddef>

int main() {
  int arr[8] = {10, 11, 12, 13, 14, 15, 16, 17};
  int *p = arr;

  int *q = p + 1;
  assert(*q == 11);

  int *r = p + 3;
  assert(*r == 13);

  int *s = r - 2;
  assert(*s == 11);

  ptrdiff_t diff = r - p;
  assert(diff == 3);

  size_t idx = (size_t)(r - p);
  assert(idx == 3);

  int *q2 = p;
  ++q2;
  assert(*q2 == 11);

  q2++;
  assert(*q2 == 12);

  --q2;
  assert(*q2 == 11);

  q2--;
  assert(*q2 == 10);
  assert(q2 == p);

  int *q3 = p;
  q3 += 4;
  assert(*q3 == 14);
  q3 -= 2;
  assert(*q3 == 12);

  size_t step = 2;
  int *q4 = p + step;
  assert(*q4 == 12);

  int v = p[3];
  assert(v == 13);

  int v2 = *(p + 4);
  assert(v2 == 14);

  *(p + 5) = 99;
  assert(p[5] == 99);
  assert(arr[5] == 99);

  int *end = p + 8;
  int sum = 0;
  for (int *it = p; it != end; ++it) {
    sum += *it;
  }
  assert(sum == 10 + 11 + 12 + 13 + 14 + 99 + 16 + 17);

  unsigned char bytes[8] = {0, 1, 2, 3, 4, 5, 6, 7};
  unsigned char *bp = bytes;
  unsigned char *bq = bp + 4;
  assert(*bq == 4);

  ptrdiff_t bdiff = bq - bp;
  assert(bdiff == 4);

  const int *cp = arr;
  const int *cq = cp + 2;
  assert(*cq == 12);
  ptrdiff_t cdiff = cq - cp;
  assert(cdiff == 2);

  size_t n = 3;
  int *q5 = arr + n;
  assert(*q5 == 13);

  int *q6 = &arr[n];
  assert(q6 == q5);

  int matrix[3][4] = {{0, 1, 2, 3}, {4, 5, 6, 7}, {8, 9, 10, 11}};
  int *row1 = &matrix[1][0];
  assert(row1[2] == 6);

  int *back = end - 1;
  assert(*back == 17);

  return 0;
}
