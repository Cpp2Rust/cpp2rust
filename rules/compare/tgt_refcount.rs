// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_less()
}

fn f2() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_equal()
}

fn f3() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_equivalent()
}

fn f4() -> Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_greater()
}
