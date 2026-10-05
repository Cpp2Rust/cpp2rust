// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cmath>

int f1(int x) { return std::abs(x); }

long f2(long x) { return std::abs(x); }

long long f3(long long x) { return std::abs(x); }

double f4(double x) { return std::log2(x); }

double f5(int x) { return std::log2(x); }
