// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <memory>
#include <utility>
#include <vector>

// N4950 [sequence.reqmts]/30: a.insert(p, rv) inserts a copy of rv (copy
// according to [container.alloc.reqmts]/2.3, where T is Cpp17MoveInsertable)
// before p.
namespace p30_insert_moves_pair {
void test() {
  std::pair<std::unique_ptr<int>, int> source(std::unique_ptr<int>(new int(1)),
                                              2);
  std::vector<std::pair<std::unique_ptr<int>, int>> pairs;
  pairs.insert(pairs.end(), std::move(source));
  assert(*pairs[0].first == 1);
  assert(source.first == nullptr);
  assert(source.second == 2);
}
} // namespace p30_insert_moves_pair

// N4950 [sequence.reqmts]/104: a.push_back(rv) appends a copy of rv (copy
// according to [container.alloc.reqmts]/2.3, where T is Cpp17MoveInsertable).
namespace p104_push_back_moves_pair {
void test() {
  std::pair<std::unique_ptr<int>, int> source(std::unique_ptr<int>(new int(1)),
                                              2);
  std::vector<std::pair<std::unique_ptr<int>, int>> pairs;
  pairs.push_back(std::move(source));
  assert(*pairs[0].first == 1);
  assert(source.first == nullptr);
  assert(source.second == 2);
}
} // namespace p104_push_back_moves_pair

int main() {
  p30_insert_moves_pair::test();
  p104_push_back_moves_pair::test();
  return 0;
}
