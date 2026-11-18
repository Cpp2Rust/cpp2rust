// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <memory>
#include <string>
#include <utility>
#include <vector>

// N4950 [pairs.pair]/2: the defaulted move constructor of pair moves its
// elements.
namespace p2_move_constructor {
void test() {
  std::pair<std::unique_ptr<int>, int> source(std::unique_ptr<int>(new int(1)),
                                              2);
  std::pair<std::unique_ptr<int>, int> target(std::move(source));
  assert(*target.first == 1);
  assert(target.second == 2);
  assert(source.first == nullptr);
  assert(source.second == 2);
}
} // namespace p2_move_constructor

// N4950 [pairs.pair]/12: pair(U1&& x, U2&& y) initializes first with
// std::forward<U1>(x) and second with std::forward<U2>(y).
namespace p12_forwarding_constructor {
void test() {
  std::unique_ptr<int> owner(new int(3));
  std::pair<std::unique_ptr<int>, int> moved(std::move(owner), 1);
  assert(owner.get() == nullptr);
  assert(*moved.first == 3);
  assert(moved.second == 1);

  std::string text = "abc";
  std::pair<std::string, int> moved_text(std::move(text), 2);
  assert(moved_text.first == "abc");

  const std::vector<int> values(2, 7);
  std::pair<std::vector<int>, int> copied(std::move(values), 3);
  assert(values.size() == 2);
  assert(copied.first.size() == 2 && copied.first[1] == 7);

  int referent = 5;
  std::pair<const int &, char> bound(referent, 'c');
  referent = 6;
  assert(bound.first == 6);
  assert(&bound.first == &referent);
  assert(bound.second == 'c');
}
} // namespace p12_forwarding_constructor

// N4950 [pairs.pair]/16: pair(pair<U1, U2>&& p) initializes first with
// get<0>(FWD(p)) and second with get<1>(FWD(p)).
namespace p16_converting_move_constructor {
void test() {
  std::pair<int, std::unique_ptr<int>> source(
      0, std::unique_ptr<int>(new int(81)));
  int *original = source.second.get();
  std::pair<long, std::unique_ptr<int>> converted(std::move(source));
  assert(converted.first == 0L);
  assert(converted.second.get() == original);
  assert(*converted.second == 81);
  assert(source.second.get() == nullptr);
}
} // namespace p16_converting_move_constructor

// N4950 [pairs.pair]/40: operator=(pair<U1, U2>&& p) assigns to first with
// std::forward<U1>(p.first) and to second with std::forward<U2>(p.second).
namespace p40_converting_move_assignment {
void test() {
  std::pair<int, std::unique_ptr<int>> source(4,
                                              std::unique_ptr<int>(new int(9)));
  int *original = source.second.get();
  std::pair<long, std::unique_ptr<int>> target;
  target = std::move(source);
  assert(target.first == 4L);
  assert(target.second.get() == original);
  assert(*target.second == 9);
  assert(source.second.get() == nullptr);
}
} // namespace p40_converting_move_assignment

int main() {
  p2_move_constructor::test();
  p12_forwarding_constructor::test();
  p16_converting_move_constructor::test();
  p40_converting_move_assignment::test();
  return 0;
}
