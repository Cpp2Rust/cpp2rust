#include <assert.h>

struct Val {
  int x;
};

static int sum(Val a, Val b) { return a.x + b.x; }

int main() {
  int total = 0;
  auto tally = [&total]<typename T, typename U> {
    total += sizeof(T) + sizeof(U);
  };
  tally.operator()<char, char>();
  tally.operator()<int, char>();
  assert(total == 7);

  Val v{5};
  int acc = 0;
  auto pick = [&v, &acc]<typename Q> { acc += sum(static_cast<Q>(v), v); };
  pick.operator()<Val &>();
  pick.operator()<const Val &>();
  pick.operator()<Val &&>();
  assert(acc == 30);

  auto cast_to = []<typename T>(int x) { return static_cast<T>(x) / 2; };
  assert(cast_to.operator()<int>(5) == 2);
  assert(cast_to.operator()<double>(5) == 2.5);

  return 0;
}
