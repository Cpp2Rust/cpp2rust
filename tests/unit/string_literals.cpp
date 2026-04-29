void foo_mut(char *str) {}
void foo_const(const char *str) {}

int main() {
  const char *immutable_strings[] = {"a", "b", "c"};

  const char *immutable_string = "hello";

  char mutable_string_arr[] = "papanasi";
  const char immutable_string_arr[] = "papanasi";

  const char *immutable_empty = "";
  char mutable_empty_arr[] = "";
  const char immutable_empty_arr[] = "";

  foo_mut(mutable_string_arr);

  foo_const("world");
  foo_const(immutable_string);
  foo_const(immutable_string_arr);

  foo_const("");
  foo_const(immutable_empty);
  foo_const(immutable_empty_arr);
  return 0;
}
