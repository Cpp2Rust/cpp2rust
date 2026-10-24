#include <array>
#include <cassert>
#include <utility>
#include <vector>

// Arrays and vectors in fields are stored inline in their structs, including
// when nested, and are reached through the structs.

struct Elem {
  int id;
  int hist[4];
  std::vector<short> coeffs;
};

struct State {
  int count;
  int grid[2][3];
  std::array<int, 3> arr;
  std::vector<Elem> elems;
  std::vector<std::vector<int>> rows;
};

static int sum(const int *p, int n) {
  int s = 0;
  for (int i = 0; i < n; ++i)
    s += p[i];
  return s;
}

static void fill(State *s) {
  for (int i = 0; i < 2; ++i)
    for (int j = 0; j < 3; ++j)
      s->grid[i][j] = i * 3 + j;
  s->arr[1] = 7;
  s->elems.resize(3);
  for (int i = 0; i < 3; ++i) {
    s->elems[i].id = i;
    s->elems[i].hist[i] = 10 * i;
    s->elems[i].coeffs.resize(4, (short)i);
  }
  s->rows.push_back({1, 2, 3});
}

int main() {
  State s{};
  fill(&s);
  assert(s.grid[1][2] == 5 && sum(s.grid[1], 3) == 12);
  assert(s.arr[1] == 7);

  // Pointers into containers in elements of a vector field, kept and used
  // while other fields of the struct change.
  Elem *e = &s.elems[2];
  int *h = e->hist;
  short *c = &e->coeffs[1];
  int *id = &s.elems[1].id;
  for (int k = 0; k < 4; ++k)
    h[k] += k;
  *c += 5;
  *id = 11;
  s.count = 3;
  assert(sum(s.elems[2].hist, 4) == 26 && s.elems[2].coeffs[1] == 7);
  assert(s.elems[1].id == 11 && h + 3 == &s.elems[2].hist[3]);
  int *row = &s.rows[0][0];
  row[2] = 9;
  assert(s.rows[0][2] == 9);

  // Copies of the struct copy the containers.
  State t = s;
  t.elems[2].hist[0] = 100;
  t.grid[0][0] = 100;
  t.rows[0][0] = 100;
  assert(s.elems[2].hist[0] == 0 && s.grid[0][0] == 0 && s.rows[0][0] == 1);
  assert(t.elems[2].coeffs[1] == 7);

  // Elements of array and vector fields reached through pointers.
  State *p = &t;
  p->elems[0].hist[3] = p->grid[1][1] + p->arr[1];
  assert(t.elems[0].hist[3] == 11);
  p->elems.push_back(p->elems[2]);
  assert(p->elems.size() == 4 && p->elems[3].hist[0] == 100);

  // Elements of array and vector fields of elements of a vector variable
  // whose address is taken.
  std::vector<Elem> elems(2);
  Elem *first = &elems[0];
  elems[1].coeffs.resize(2);
  elems[1].coeffs[1] = 4;
  elems[1].hist[2] = elems[1].coeffs[1] + 1;
  assert(elems[1].hist[2] == 5 && first->hist[2] == 0);

  // Moves of the struct copy the arrays and move the vectors.
  State moved(std::move(t));
  assert(moved.grid[0][0] == 100 && moved.grid[1][2] == 5);
  assert(moved.elems.size() == 4 && t.elems.empty() && t.rows.empty());
  State assigned{};
  assigned = std::move(moved);
  assert(assigned.grid[0][0] == 100 && assigned.grid[1][1] == 4);
  assert(assigned.elems[2].coeffs[1] == 7 && moved.elems.empty());
  return 0;
}
