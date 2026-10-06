// no-compile: unsafe
#include <cassert>
#include <vector>

static int next = 0;

static int bump(std::vector<int> *v, int x) {
  (*v)[0] += x;
  return next++;
}

int main() {
  std::vector<int> v(4, 1);
  std::vector<unsigned> items(3);

  // Accessed in place.
  for (unsigned i = 0; i < items.size(); ++i) {
    items[i] = i * 2;
  }
  items[1] += 5;
  items[2]++;
  assert(items[0] == 0 && items[1] == 7 && items[2] == 5);

  // The index reads the vector itself.
  v[v[0]] = 3;
  v[v[1]] = v[1] + 4;
  assert(v[1] == 3 && v[3] == 7);

  // The index is read through a pointer to an element of the vector.
  int *p = &v[0];
  v[*p] = 9;
  assert(v[1] == 9);

  // The index has side effects that write to the vector.
  v[bump(&v, 10)] = 2;
  assert(v[0] == 2);

  // An element is passed along with a pointer to the vector, which writes to
  // it.
  assert(bump(&v, v[2]) == 1 && v[0] == 3);
  return 0;
}
