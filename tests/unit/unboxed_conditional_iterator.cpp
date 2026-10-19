#include <algorithm>
#include <cassert>
#include <vector>

// Only the values of w and n are read through the conditional.
int median(int w, int n, int nw) {
  const int mx = (w > n) ? w : n;
  const int mn = w + n - mx;
  if (nw > mx) {
    return mn;
  } else if (nw < mn) {
    return mx;
  }
  return n + w - nw;
}

// a and b are written through the conditional.
int assign(bool c, int v) {
  int a = 1;
  int b = 2;
  (c ? a : b) = v;
  ++(c ? a : b);
  return a * 10 + b;
}

void lehmer(const unsigned *sigma, unsigned len, unsigned *code) {
  std::vector<unsigned> items(len);
  for (unsigned i = 0; i < len; ++i)
    items[i] = i;
  for (unsigned i = 0; i < len; ++i) {
    std::vector<unsigned>::iterator it =
        std::find(items.begin(), items.end(), sigma[i]);
    assert(it != items.end());
    code[i] = static_cast<unsigned>(it - items.begin());
    items.erase(it);
  }
}

int sum(std::vector<int> &v) {
  int s = 0;
  for (std::vector<int>::iterator it = v.begin(); it != v.end(); ++it) {
    s += *it;
  }
  return s;
}

int main() {
  assert(median(1, 5, 7) == 1);
  assert(median(1, 5, 0) == 5);
  assert(median(1, 5, 3) == 3);
  assert(assign(true, 5) == 62);
  assert(assign(false, 5) == 16);
  unsigned sigma[] = {2, 0, 3, 1};
  unsigned code[4];
  lehmer(sigma, 4, code);
  assert(code[0] == 2 && code[1] == 0 && code[2] == 1 && code[3] == 0);
  std::vector<int> v = {1, 2, 3};
  assert(sum(v) == 6);
  return 0;
}
