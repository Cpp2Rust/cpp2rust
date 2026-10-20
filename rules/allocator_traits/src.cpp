// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <memory>

template <typename T2, typename T1 = typename T2::value_type>
using t1 = typename std::allocator_traits<T2>::value_type;
