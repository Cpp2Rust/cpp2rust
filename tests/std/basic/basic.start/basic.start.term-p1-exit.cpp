#include <cstdio>
#include <cstdlib>

struct Tracker {
  int tag;

  explicit Tracker(int tag) : tag(tag) { std::printf("construct %d\n", tag); }
  ~Tracker() { std::printf("destroy %d\n", tag); }
};

// N4950 [basic.start.term]/1: Constructed objects (9.4) with static storage
// duration are destroyed and functions registered with std::atexit are called
// as part of a call to std::exit (17.5). The call to std::exit is sequenced
// before the destructions and the registered functions.
Tracker global(1);

void exit_from_nested_call() {
  static Tracker local_static(2);
  std::printf("local static %d\n", local_static.tag);
  Tracker automatic(3);
  std::exit(0);
}

int main() {
  exit_from_nested_call();
  std::printf("not reached\n");
  return 0;
}
