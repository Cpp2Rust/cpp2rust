// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstdint>
#include <string>

int main() {
  std::string s1 = "hello";
  assert(s1.length() == 5);
  assert(s1.size() == s1.length());
  assert(s1[0] == 'h');
  assert(s1[1] == 'e');
  assert(s1[2] == 'l');
  assert(s1[3] == 'l');
  assert(s1[4] == 'o');
  assert(s1 == "hello");
  const char *p1 = s1.data();
  assert(p1[0] == 'h');
  assert(p1[1] == 'e');
  assert(p1[2] == 'l');
  assert(p1[3] == 'l');
  assert(p1[4] == 'o');
  std::string s2(10, 'a');
  const char *p2 = s2.data();
  for (unsigned i = 0; i < s2.size(); ++i)
    assert(p2[i] == 'a' && s2[i] == 'a');
  assert(s2.length() == 10);
  assert(s2.size() == s2.length());
  s2[0] = 'b';
  s2[1] = 'c';
  assert(s2[0] == 'b');
  assert(s2[1] == 'c');
  for (unsigned i = 2; i < s2.size(); ++i)
    assert(p2[i] == 'a' && s2[i] == 'a');
  std::string s3 = s2.substr(2, 5);
  assert(s3.length() == 5);
  assert(s3.size() == s3.length());
  const char *p3 = s3.data();
  for (unsigned i = 0; i < s3.size(); ++i)
    assert(p3[i] == s3[i]);
  std::string s4 = s1.substr(1, s1.find_last_of("l"));
  assert(s4.length() == 3);
  assert(s4.size() == s4.length());
  const char *p4 = s4.data();
  for (unsigned i = 0; i < s4.size(); ++i)
    assert(p4[i] == s4[i]);
  std::string s5 = s1 + ", world";
  assert(s5.length() == 12);
  assert(s5.size() == s5.length());
  const char *p5 = s5.data();
  for (unsigned i = 0; i < s5.size(); ++i)
    assert(p5[i] == s5[i]);
  char arr[] = {'b', 'a', 'r', ' ', 'f', 'o', 'o'};
  std::string string(arr, 3);
  assert(string.size() == 3);
  assert(string[0] == 'b');
  assert(string[1] == 'a');
  assert(string[2] == 'r');
  assert(string == "bar");
  string.resize(3);
  assert(string.size() == 3);
  assert(string[0] == 'b');
  assert(string[1] == 'a');
  assert(string[2] == 'r');
  assert(string == "bar");
  string.resize(5);
  assert(string.size() == 5);
  assert(string[0] == 'b');
  assert(string[1] == 'a');
  assert(string[2] == 'r');
  assert(string[3] == 0);
  assert(string[4] == 0);
  string[3] = 'a';
  string[4] = 'b';
  assert(string[3] == 'a');
  assert(string[4] == 'b');
  string[3] = 0;
  string[4] = 0;
  string.resize(4);
  assert(string.size() == 4);
  assert(string[0] == 'b');
  assert(string[1] == 'a');
  assert(string[2] == 'r');
  assert(string[3] == 0);
  std::string result = string + " foo";
  assert(result.size() == 8);
  assert(result[0] == 'b');
  assert(result[1] == 'a');
  assert(result[2] == 'r');
  assert(result[3] == 0);
  assert(result[4] == ' ');
  assert(result[5] == 'f');
  assert(result[6] == 'o');
  assert(result[7] == 'o');
  std::string substr_0 = result.substr(5, 3);
  assert(substr_0.size() == 3);
  assert(substr_0[0] == 'f');
  assert(substr_0[1] == 'o');
  assert(substr_0[2] == 'o');
  std::string substr_1 = result.substr(0, 5);
  assert(substr_1.size() == 5);
  assert(substr_1[0] == 'b');
  assert(substr_1[1] == 'a');
  assert(substr_1[2] == 'r');
  assert(substr_1[3] == 0);
  assert(substr_1[4] == ' ');
  std::string substr_2 = result.substr(0, 15);
  assert(substr_2.size() == 8);
  assert(substr_2[0] == 'b');
  assert(substr_2[1] == 'a');
  assert(substr_2[2] == 'r');
  assert(substr_2[3] == 0);
  assert(substr_2[4] == ' ');
  assert(substr_2[5] == 'f');
  assert(substr_2[6] == 'o');
  assert(substr_2[7] == 'o');
  auto pos = result.find_last_of("b");
  assert(pos == 0);
  pos = result.find_last_of("f");
  assert(pos == 5);
  pos = result.find_last_of("o");
  assert(pos == 7);
  pos = result.find_last_of("x");
  assert(pos == -1ul);
  std::string string_to_cast = "cast";
  [[maybe_unused]] std::uint8_t *output_data =
      reinterpret_cast<std::uint8_t *>(&string_to_cast[0]);
  assert(*output_data == 'c');
  assert(output_data[1] == 'a');
  assert(output_data[2] == 's');
  assert(output_data[3] == 't');

  // FIXME
  auto t0 = s1.length();
  auto t1 = t0 + *p1;
  auto t2 = t1 + s2.length();
  auto t3 = t2 + s3.size();
  auto t4 = t3 + s4.size();
  assert(t4 + s5.size() == 139);
  return 0;
  // return s1.length() + *p1 + s2.length() + s3.size() + s4.size() + s5.size();
}
