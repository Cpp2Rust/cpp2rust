#include <assert.h>
#include <stdint.h>
#include <string>
#include <vector>

#define UNCONST(p) ((void *)(uintptr_t)(const void *)(p))

void set(int &r) { r = 7; }

template <typename T> void set_through(const T &v) { set(const_cast<T &>(v)); }

struct Pair {
  int x;
};

void take(int &&r) { r = 8; }

void grow(std::vector<int> &&v) { v.push_back(3); }

void reset(int *&r, int *to) { r = to; }

int main() {
  const int a = 1;
  const int *p = &a;
  auto q = static_cast<int *>(UNCONST(p));
  assert(p == q);

  int v = 1;
  const int &cr = v;
  set(const_cast<int &>(cr));
  assert(v == 7);
  v = 0;
  set_through(v);
  assert(v == 7);
  v = 0;
  set((int &)cr);
  assert(v == 7);
  const_cast<int &>(cr) = 2;
  assert(v == 2);
  (int &)cr = 3;
  assert(v == 3);
  take((int &&)cr);
  assert(v == 8);
  assert(static_cast<const int &>(v) == 8);

  Pair pair{1};
  const Pair &cpair = pair;
  const_cast<Pair &>(cpair).x = 2;
  assert(pair.x == 2);
  ((Pair &)cpair).x = 3;
  assert(static_cast<const Pair &>(pair).x == 3);
  int *px = &const_cast<Pair &>(cpair).x;
  *px = 4;
  const_cast<Pair &>(cpair).x += 1;
  assert(pair.x == 5);

  int w = 0;
  int *ptr = &v;
  int *const &cptr = ptr;
  reset((int *&)cptr, &w);
  assert(ptr == &w);
  *(int *&)cptr = 4;
  assert(w == 4);
  const_cast<int *&>(cptr) = &v;
  assert(ptr == &v);
  (int *&)cptr = &w;
  assert(ptr == &w);

  std::string str = "ab";
  const std::string &cstr = str;
  const_cast<std::string &>(cstr).resize(3);
  assert(str.size() == 3);
  ((std::string &)cstr).clear();
  assert(static_cast<const std::string &>(str).empty());

  std::vector<int> vec{1};
  const std::vector<int> &cvec = vec;
  const_cast<std::vector<int> &>(cvec).push_back(2);
  assert(vec.size() == 2);
  ((std::vector<int> &)cvec)[0] = 5;
  assert(vec[0] == 5);
  set(const_cast<std::vector<int> &>(cvec)[0]);
  assert(vec[0] == 7);
  grow((std::vector<int> &&) cvec);
  assert(static_cast<const std::vector<int> &>(vec).size() == 3);
  return 0;
}
