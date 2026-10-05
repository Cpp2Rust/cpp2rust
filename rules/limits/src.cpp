// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <limits>

template <typename T1> T1 f1(std::numeric_limits<T1> &a0) { return a0.max(); }

template <typename T1> T1 f2(std::numeric_limits<T1> &a0) { return a0.min(); }
