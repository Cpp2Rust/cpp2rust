// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <bit>

template <typename T1> bool f1(T1 x) { return std::has_single_bit(x); }

template <typename T1> T1 f2(T1 x) { return std::bit_ceil(x); }

template <typename T1> T1 f3(T1 x) { return std::bit_floor(x); }

template <typename T1> int f4(T1 x) { return std::bit_width(x); }

template <typename T1> T1 f5(T1 x, int s) { return std::rotl(x, s); }

template <typename T1> T1 f6(T1 x, int s) { return std::rotr(x, s); }

template <typename T1> int f7(T1 x) { return std::countl_zero(x); }

template <typename T1> int f8(T1 x) { return std::countl_one(x); }

template <typename T1> int f9(T1 x) { return std::countr_zero(x); }

template <typename T1> int f10(T1 x) { return std::countr_one(x); }

template <typename T1> int f11(T1 x) { return std::popcount(x); }
