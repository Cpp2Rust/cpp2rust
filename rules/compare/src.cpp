// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <compare>

typedef std::strong_ordering t1;

const std::strong_ordering &f1() { return std::strong_ordering::less; }

const std::strong_ordering &f2() { return std::strong_ordering::equal; }

const std::strong_ordering &f3() { return std::strong_ordering::equivalent; }

const std::strong_ordering &f4() { return std::strong_ordering::greater; }

bool f5(std::strong_ordering a0, std::strong_ordering a1) {
  return operator==(a0, a1);
}

bool f6(std::strong_ordering a0) { return operator==(a0, 0); }

bool f7(std::strong_ordering a0) { return operator<(a0, 0); }

bool f8(std::strong_ordering a0) { return operator>(a0, 0); }

bool f9(std::strong_ordering a0) { return operator<=(a0, 0); }

bool f10(std::strong_ordering a0) { return operator>=(a0, 0); }
