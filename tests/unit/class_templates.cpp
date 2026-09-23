#include <cassert>
#include <vector>

template <typename T> class MyContainer {
  std::vector<T> vec;

public:
  using value_type = T;
  using reference = T &;
  using const_reference = const T &;
  using size_type = std::size_t;

  bool empty() const { return vec.empty(); }
  size_type size() const { return vec.size(); }
  const_reference back() const { return vec.back(); }
  reference back() { return vec.back(); }
  void pop_back() { return vec.pop_back(); }
  void push_back(const_reference item) { vec.push_back(item); }
};

template <typename T> struct Boxed {
  T value;
  static T twice(T v);
  T plus(T other) const;
};

template <typename T> T Boxed<T>::twice(T v) { return v + v; }

template <typename T> T Boxed<T>::plus(T other) const { return value + other; }

template <typename T> struct Outer {
  template <typename U> struct Inner {
    T t;
    U u;
    int sum() const { return (int)t + (int)u; }
  };

  T v;
  Inner<int> with(int n) const { return Inner<int>{v, n}; }
};

int main() {
  Outer<int> oi{3};
  assert(oi.with(4).sum() == 7);
  Outer<long> ol{5};
  Outer<long>::Inner<char> ic{6, 'a'};
  assert(ol.with(2).sum() == 7);
  assert(ic.sum() == 6 + 'a');

  assert(Boxed<int>::twice(3) == 6);
  Boxed<int> bi{4};
  assert(bi.plus(5) == 9);

  assert(Boxed<long>::twice(10) == 20);
  Boxed<long> bl{7};
  assert(bl.plus(1) == 8);

  MyContainer<int> imc;
  assert(imc.empty());
  imc.push_back(1);
  assert(imc.size() == 1 && imc.back() == 1);
  imc.pop_back();
  assert(imc.empty());

  MyContainer<char> cmc;
  assert(cmc.empty());
  cmc.push_back('a');
  assert(cmc.size() == 1 && cmc.back() == 'a');
  cmc.pop_back();
  assert(cmc.empty());

  MyContainer<float> fmc;
  assert(fmc.empty());
  fmc.push_back(1.0);
  assert(fmc.size() == 1 && fmc.back() == 1.0);
  fmc.pop_back();
  assert(fmc.empty());
  return 0;
}
