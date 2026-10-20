// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <type_traits>

template <typename Ref, typename T1 = typename std::remove_reference<Ref>::type>
using t1 = typename std::remove_reference<Ref>::type;
