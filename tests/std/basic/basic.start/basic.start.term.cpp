#include <cstdio>

struct Tracker {
  int tag;

  explicit Tracker(int tag) : tag(tag) { std::printf("construct %d\n", tag); }
  ~Tracker() { std::printf("destroy %d\n", tag); }
  void touch() const { std::printf("touch %d\n", tag); }
};

struct HoldsTracker {
  Tracker member;
};

struct TriviallyDestructible {
  int value;
};

// N4950 [basic.start.term]/1: Constructed objects (9.4) with static storage
// duration are destroyed and functions registered with std::atexit are called
// as part of a call to std::exit (17.5). The call to std::exit is sequenced
// before the destructions and the registered functions.
// [Note 1 : Returning from main invokes std::exit (6.9.3.1). - end note]
namespace p1_constructed_objects_destroyed {
Tracker global(1);

void never_called() {
  static Tracker never_constructed(100);
  never_constructed.touch();
}

void test() { global.touch(); }
} // namespace p1_constructed_objects_destroyed

// N4950 [basic.start.term]/3: If the completion of the constructor or dynamic
// initialization of an object with static storage duration strongly happens
// before that of another, the completion of the destructor of the second is
// sequenced before the initiation of the destructor of the first.
namespace p3_reverse_completion_order {
const Tracker &local_static_during_global_init() {
  static Tracker during_init(3);
  return during_init;
}

Tracker before(2);
Tracker after(local_static_during_global_init().tag + 1);
HoldsTracker holder{Tracker(5)};
TriviallyDestructible trivial{6};

struct WithStaticMember {
  static Tracker member;
};
Tracker WithStaticMember::member(7);

auto capturing = [captured = Tracker(8)] { return captured.tag; };

void run_local_static() {
  static Tracker during_main(9);
  during_main.touch();
}

void test() {
  run_local_static();
  run_local_static();
  local_static_during_global_init().touch();
  std::printf("trivial %d\n", trivial.value);
  WithStaticMember::member.touch();
  std::printf("captured %d\n", capturing());
}
} // namespace p3_reverse_completion_order

int main() {
  p1_constructed_objects_destroyed::test();
  p3_reverse_completion_order::test();
  std::printf("return from main\n");
  return 0;
}
