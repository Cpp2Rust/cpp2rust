#include <cassert>
#include <cstdint>
#include <string>

int next() {
  static int counter = 0;
  return ++counter;
}

// constexpr function call
constexpr uint8_t marker(uint8_t tag) { return (tag << 3) | 2; }
const uint8_t signature[3] = {marker(1), 0x04, 'B'};
static uint8_t single = marker(2);

// non-constexpr function call
static int from_call = next();
static int depends_on_call = from_call + 1;

// constructors
struct Ctor {
  int v;
  Ctor() : v(next()) {}
  explicit Ctor(int x) : v(x) {}
};
static Ctor default_ctor;
static Ctor arg_ctor(7);

// std::string
std::string str = "abc";

// static data members
struct Holder {
  static int member;
  static inline Ctor inline_member{5};
};
int Holder::member = next();

// static locals
int local_static() {
  static int once = next();
  static Ctor local_ctor(3);
  return once + local_ctor.v;
}

// singleton via function local static
struct Singleton {
  int hits;
  Singleton() : hits(0) {}
  static Singleton &instance() {
    static Singleton s;
    return s;
  }
};

int main() {
  assert(signature[0] == 10);
  assert(signature[1] == 0x04);
  assert(single == 18);

  assert(from_call == 1);
  assert(depends_on_call == 2);

  assert(default_ctor.v == 2);
  assert(arg_ctor.v == 7);

  assert(str == "abc");

  assert(Holder::member == 3);
  assert(Holder::inline_member.v == 5);

  assert(local_static() == 7);
  assert(local_static() == 7);

  Singleton::instance().hits++;
  Singleton::instance().hits++;
  assert(Singleton::instance().hits == 2);
  assert(&Singleton::instance() == &Singleton::instance());
  return 0;
}
