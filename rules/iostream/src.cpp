// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iostream>

void t1(std::ostream);
void t2(std::ostream &);
void t3(std::ostream *);

std::ostream &f1() { return std::cout; }

std::ostream &f2() { return std::cerr; }

std::ostream *f3() { return &std::cout; }

std::ostream *f4() { return &std::cerr; }
