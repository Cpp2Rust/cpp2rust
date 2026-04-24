// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdlib>

template <typename T1, std::size_t T2> using t1 = T1[T2][T2];
template <typename T1, std::size_t T2> using t2 = T1[T2][T2][T2];
