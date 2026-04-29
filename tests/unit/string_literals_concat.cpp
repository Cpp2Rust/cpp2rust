#include <cassert>

int main() {
  const char *joined = "alpha\n"
                       "beta\n"
                       "gamma\n";
  assert(joined[0] == 'a');
  assert(joined[5] == '\n');
  assert(joined[6] == 'b');

  char arr[] = "foo"
               "bar";
  assert(arr[0] == 'f');
  assert(arr[3] == 'b');
  assert(arr[5] == 'r');
  assert(arr[6] == '\0');

  const char *split_pieces = "abc"
                             "def"
                             "ghi";
  assert(split_pieces[0] == 'a');
  assert(split_pieces[3] == 'd');
  assert(split_pieces[6] == 'g');
  assert(split_pieces[8] == 'i');
  assert(split_pieces[9] == '\0');
  return 0;
}
