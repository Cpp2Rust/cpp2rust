// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>

template <typename T1>
using t1 = typename std::iterator_traits<T1 *>::difference_type;

template <typename T1>
using t2 = typename std::move_iterator<T1>::iterator_type;

template <typename T1>
using t3 = typename std::iterator_traits<T1 *>::reference;
