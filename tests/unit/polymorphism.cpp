#include <cassert>

class Animal {
public:
  virtual bool bark() const = 0;
};

class Dog : public Animal {
  bool bark() const override { return true; }
};

class Cat : public Animal {
  bool bark() const override { return false; }
  bool meow() const { return true; }
};

int main() {
  Dog dog{};
  Animal *animal = &dog;
  bool eat1 = animal->bark();
  Cat cat{};
  animal = &cat;
  bool eat2 = animal->bark();
  assert(eat1 && !eat2);
  return 0;
}
