#include <cstdio>
#include <string>
#include <utility>

static void print_moved_from(const char *label, const std::string &from,
                             const std::string &to) {
  std::string sub = from.substr(0, 0);
  printf("%s: %zu %zu %d %zu %d [%s]\n", label, from.size(), sub.size(),
         from.c_str()[0], to.size(), sub.c_str()[0], to.c_str());
}

static std::string return_local() {
  std::string word = "returned local";
  return (word);
}

static std::string return_parameter(std::string word) { return word; }

static std::string return_reference(std::string &word) {
  return std::move(word);
}

static std::string return_rvalue_reference(std::string &&word) { return word; }

int main() {
  std::string empty;
  std::string from_empty(std::move(empty));
  print_moved_from("construct empty", empty, from_empty);

  std::string word = "hello";
  std::string from_word(std::move(word));
  print_moved_from("construct word", word, from_word);

  std::string long_word = "a string that does not fit in place";
  std::string assigned = "x";
  assigned = std::move(long_word);
  print_moved_from("assign long", long_word, assigned);

  std::string short_word = "ab";
  std::string assigned_short;
  assigned_short = std::move(short_word);
  print_moved_from("assign short", short_word, assigned_short);

  word.append("again", 5);
  printf("reuse: %zu %s\n", word.size(), word.c_str());
  std::string local = return_local();
  std::string parameter = return_parameter("returned parameter");
  printf("returns: %s %s\n", local.c_str(), parameter.c_str());

  std::string source = "returned reference";
  std::string reference = return_reference(source);
  print_moved_from("return reference", source, reference);

  source.append("rvalue reference", 16);
  std::string rvalue_reference = return_rvalue_reference(std::move(source));
  print_moved_from("return rvalue reference", source, rvalue_reference);
  return 0;
}
