// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.scope.scope]/4: two declarations correspond if they
// (re)introduce the same name; they can correspond even if neither binds a
// name.
#include <cassert>

static int friend_calls;

struct DeclaresFriend {
  friend int shared_friend();
};

struct DefinesFriend {
  friend int shared_friend() {
    ++friend_calls;
    return 7;
  }
};

int shared_friend();

int main() {
  assert(shared_friend() == 7);
  assert(friend_calls == 1);
  return 0;
}
