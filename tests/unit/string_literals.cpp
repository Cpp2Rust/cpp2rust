void foo_mut(char *str) {}
void foo_const(const char *str) {}

int main() {
  char *mutable_strings[] = {"a", "b", "c"};
  const char *immutable_strings[] = {"a", "b", "c"};

  char *mutable_string = "hello";
  const char *immutable_string = "hello";

  char mutable_string_arr[] = "papanasi";
  const char immutable_string_arr[] = "papanasi";

  foo_mut("world");
  foo_mut(mutable_string);
  foo_mut(mutable_string_arr);

  foo_const("world");
  foo_const(mutable_string);
  foo_const(immutable_string);
  foo_const(mutable_string_arr);
  foo_const(immutable_string_arr);
  return 0;
}
