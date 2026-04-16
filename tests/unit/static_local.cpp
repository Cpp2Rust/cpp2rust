// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

int foo() {
  static int kX1 = 1;
  static const int kX2 = 2;
  kX1 += 1;
  return kX1 + kX2;
}

int main() { return foo() + foo() + foo(); }
