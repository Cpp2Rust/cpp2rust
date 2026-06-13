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

int main() {
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
  fmc.push_back('a');
  assert(fmc.size() == 1 && fmc.back() == 'a');
  fmc.pop_back();
  assert(fmc.empty());
  return 0;
}
