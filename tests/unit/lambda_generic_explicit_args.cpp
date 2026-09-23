#include <assert.h>

struct Val {
  int x;
};

static int sum(Val a, Val b) { return a.x + b.x; }

int main() {
  int total = 0;
  auto tally = [&total]<typename T, typename U> { total += sizeof(T) + sizeof(U); };
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

  return 0;
}
