void foo_mut(char *str) {}
void foo_const(const char *str) {}

int main() {
  char *mutable_strings[] = {"a", "b", "c"};
  const char *immutable_strings[] = {"a", "b", "c"};

  char *mutable_string = "hello";
  const char *immutable_string = "hello";

  char mutable_string_arr[] = "papanasi";
  const char immutable_string_arr[] = "papanasi";

  char *mutable_empty = "";
  const char *immutable_empty = "";
  char mutable_empty_arr[] = "";
  const char immutable_empty_arr[] = "";

  foo_mut("world");
  foo_mut(mutable_string);
  foo_mut(mutable_string_arr);

  foo_const("world");
  foo_const(mutable_string);
  foo_const(immutable_string);
  foo_const(mutable_string_arr);
  foo_const(immutable_string_arr);

  foo_const("");
  foo_const(mutable_empty);
  foo_const(immutable_empty);
  foo_const(mutable_empty_arr);
  foo_const(immutable_empty_arr);
  return 0;
}
