// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <string>
#include <utility>
#include <vector>

// N4950 [forward]/3: forward<T>(t) returns static_cast<T&&>(t).
namespace p3_forward {
void test() {
  int value = 1;
  int forwarded = std::forward<int>(std::move(value));
  assert(forwarded == 1);
  int &lvalue = std::forward<int &>(value);
  lvalue = 2;
  assert(value == 2);
}
} // namespace p3_forward

// N4950 [forward]/6: forward_like<T>(x) forwards x as an rvalue when T is not
// an lvalue reference, and as an lvalue otherwise.
namespace p6_forward_like {
struct Movable {
  int value;
  Movable(int value) : value(value) {}
  Movable(const Movable &other) : value(other.value) {}
  Movable(Movable &&other) : value(other.value) { other.value = 0; }
};

struct Holder {
  Movable member{1};
};

template <typename Self> Movable take(Self &&self) {
  return Movable(std::forward_like<Self>(self.member));
}

void test() {
  Holder holder;
  Movable copied = take(holder);
  assert(copied.value == 1 && holder.member.value == 1);
  Movable moved = take(std::move(holder));
  assert(moved.value == 1 && holder.member.value == 0);
}
} // namespace p6_forward_like

// N4950 [forward]/9: move(t) returns static_cast<remove_reference_t<T>&&>(t).
namespace p9_move {
struct Movable {
  int value;
  Movable(int value) : value(value) {}
  Movable(Movable &&other) : value(other.value) { other.value = 0; }
};

void test() {
  int number = 1;
  int moved = std::move(number);
  assert(moved == 1 && number == 1);

  int target = 2;
  int *pointer = &target;
  int *moved_pointer = std::move(pointer);
  assert(moved_pointer == &target && pointer == &target);

  Movable source(3);
  Movable destination = std::move(source);
  assert(destination.value == 3 && source.value == 0);

  std::vector<std::string> strings;
  std::string text = "text";
  strings.push_back(std::move(text));
  assert(strings[0] == "text");
}
} // namespace p9_move

// N4950 [forward]/10: forward passes an lvalue to A(const A&), which copies,
// and an rvalue to A(A&&), which moves.
namespace p10_forward_in_template {
struct A {
  int value;
  int copies = 0;
  int moves = 0;
  A(int value) : value(value) {}
  A(const A &other) : value(other.value), copies(other.copies + 1) {}
  A(A &&other) : value(other.value), moves(other.moves + 1) { other.value = 0; }
};

template <typename T> A make(T &&source) { return A(std::forward<T>(source)); }

void test() {
  A a(1);
  A copied = make(a);
  assert(copied.value == 1 && copied.copies == 1 && copied.moves == 0);
  assert(a.value == 1);
  A moved = make(std::move(a));
  assert(moved.value == 1 && moved.copies == 0 && moved.moves == 1);
  assert(a.value == 0);
}
} // namespace p10_forward_in_template

// N4950 [forward]/11: move_if_noexcept(x) returns std::move(x), as a const
// lvalue when the move constructor may throw and there is a copy constructor.
namespace p11_move_if_noexcept {
struct NoexceptMove {
  int value;
  NoexceptMove(int value) : value(value) {}
  NoexceptMove(const NoexceptMove &other) : value(other.value) {}
  NoexceptMove(NoexceptMove &&other) noexcept : value(other.value) {
    other.value = 0;
  }
};

struct ThrowingMove {
  int value;
  ThrowingMove(int value) : value(value) {}
  ThrowingMove(const ThrowingMove &other) : value(other.value) {}
  ThrowingMove(ThrowingMove &&other) : value(other.value) { other.value = 0; }
};

void test() {
  NoexceptMove moved_from(1);
  NoexceptMove moved(std::move_if_noexcept(moved_from));
  assert(moved.value == 1 && moved_from.value == 0);
  ThrowingMove copied_from(2);
  ThrowingMove copied(std::move_if_noexcept(copied_from));
  assert(copied.value == 2 && copied_from.value == 2);
}
} // namespace p11_move_if_noexcept

int main() {
  p3_forward::test();
  p6_forward_like::test();
  p9_move::test();
  p10_forward_in_template::test();
  p11_move_if_noexcept::test();
  return 0;
}
