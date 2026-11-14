// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>

using t1 = std::ostream;
using t2 = std::ostream &;
using t3 = std::ostream *;
using t4 = std::istream;
using t5 = std::istream &;
using t6 = std::istream *;

std::ostream &f1() { return std::cout; }

std::ostream &f2() { return std::cerr; }

std::istream &f3() { return std::cin; }

int f4(std::istream &a0) { return a0.get(); }
