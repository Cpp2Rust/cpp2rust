// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <cstdio>
#include <string>
#include <vector>

struct Text {
  std::string &value;
};

std::string &receiver(std::string &s, int &calls) {
  ++calls;
  return s;
}

std::string temporary(int &calls) {
  ++calls;
  return "hello";
}

int main() {
  std::string s = "hello";
  int calls = 0;
  auto out = receiver(s, calls).substr(calls, 2);
  printf("receiver: %s %d\n", out.c_str(), calls);

  calls = 0;
  out = temporary(calls).substr(1, 2);
  printf("temporary: %s %d\n", out.c_str(), calls);

  out = s.substr((s[0] = 'j', 0), 2);
  printf("position mutation: %s %s\n", out.c_str(), s.c_str());
  out = s.substr(0, (s[1] = 'a', 2));
  printf("count mutation: %s %s\n", out.c_str(), s.c_str());

  out[0] = 'X';
  s[1] = 'b';
  printf("independent: %s %s\n", out.c_str(), s.c_str());

  const std::string *p = &s;
  out = p->substr(1, 2);
  printf("pointer: %s\n", out.c_str());
  out = std::string("hello").substr(1, 3).substr(1, 1);
  printf("nested: %s\n", out.c_str());
  out = s.substr(s.size(), 0);
  printf("empty: %zu\n", out.size());

  std::string field = "hello";
  Text text{field};
  out = text.value.substr(1, 2);
  printf("field: %s %s\n", out.c_str(), text.value.c_str());
  std::vector<std::string> strings = {"hello", "world"};
  std::size_t index = 0;
  out = strings[index++].substr(1, 2);
  printf("element: %s %zu\n", out.c_str(), index);
  return 0;
}
