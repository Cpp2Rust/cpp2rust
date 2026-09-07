// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>
#include <memory>

struct SafePointer {
  std::unique_ptr<int> ptr;
  void inc() { ++*ptr; }
};

struct Pair {
  int x, y;
  void inc(int k);
};

void Pair::inc(int k) {
  x += k;
  y += k;
}

void DoStuffWithSafePointer(std::unique_ptr<SafePointer> &safe_ptr) {
  std::unique_ptr<int> x1 = std::make_unique<int>(0);
  std::unique_ptr<int> x2 = std::make_unique<int>(0);
  *x2 = 1;
  x1 = std::move(x2);
  int *raw_ptr1 = &*x1;
  ++*raw_ptr1;
  (*safe_ptr).ptr = std::move(x1);
  (*safe_ptr).inc();
  (*safe_ptr).inc();
  std::unique_ptr<int> x3 = std::make_unique<int>(10);
  std::unique_ptr<int> x4 = std::make_unique<int>(20);
  *x3 = *x3 + *x4;
  x4 = std::move(x3);
  int *raw_ptr2 = &*x4;
  *raw_ptr2 += 1;
  std::unique_ptr<Pair> pair = std::make_unique<Pair>(Pair{*raw_ptr2, 5});
  (*pair).inc(10);
  *(*safe_ptr).ptr = *(*safe_ptr).ptr + (*pair).x + (*pair).y;
}

int Consume(std::unique_ptr<SafePointer> safe_ptr) {
  auto x(std::move(safe_ptr));
  std::unique_ptr<Pair> p(new Pair());
  return *(*x).ptr + p->x;
}

void RndStuff() {
  std::unique_ptr<int[]> x1;
  std::unique_ptr<int[]> x2(new int[100]);
  for (int i = 0; i < 100; ++i)
    x2[i] = 1;
  x2.reset(new int[200]);
  for (int i = 0; i < 200; ++i)
    x2[i] = 2;
  int *p2 = x2.get();
  for (int i = 0; i < 200; ++i)
    assert(p2[i] == 2);
  std::unique_ptr<Pair[]> x3 = std::make_unique<Pair[]>(10);
  for (int i = 0; i < 10; ++i)
    x3[i] = Pair{1, 2};
  Pair *p3_0 = x3.get();
  for (int i = 0; i < 10; ++i) {
    assert(p3_0[i].x == 1);
    assert(p3_0[i].y == 2);
    x3[i].inc(10);
    assert(p3_0[i].x == 11);
    assert(p3_0[i].y == 12);
  }
  x3.reset(new Pair[50]);
  for (int i = 0; i < 50; ++i)
    x3[i] = Pair{-1, -2};
  Pair *p3_1 = x3.get();
  assert(p3_0 != p3_1);
  for (int i = 0; i < 50; ++i) {
    assert(p3_1[i].x == -1);
    assert(p3_1[i].y == -2);
    x3[i].inc(-10);
    assert(p3_1[i].x == -11);
    assert(p3_1[i].y == -12);
  }
}

int main() {
  std::unique_ptr<int> x = std::make_unique<int>(0);
  std::unique_ptr<SafePointer> safe_ptr =
      std::make_unique<SafePointer>(SafePointer{std::move(x)});
  DoStuffWithSafePointer(safe_ptr);
  assert(Consume(std::move(safe_ptr)) == 60);
  return 0;
}
