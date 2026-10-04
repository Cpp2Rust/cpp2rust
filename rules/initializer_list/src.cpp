// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <initializer_list>
#endif

template <typename T1> using CPP2RUST_TYPE_RULE(1) = std::initializer_list<T1>;

template <typename T1>
typename std::initializer_list<T1>::size_type
CPP2RUST_EXPR_RULE(1)(std::initializer_list<T1> &o) {
  return o.size();
}
