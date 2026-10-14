// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <new>

template <typename T1> T1 *f5(T1 *a0) { return std::launder(a0); }
