// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> std::cmp::Ordering {
    std::cmp::Ordering::Equal
}

fn f1() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_LESS
}

fn f2() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_EQUAL
}

fn f3() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_EQUIVALENT
}

fn f4() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_GREATER
}

fn f5(a0: std::cmp::Ordering, a1: std::cmp::Ordering) -> bool {
    a0 == a1
}

fn f6(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Equal
}

fn f7(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Less
}

fn f8(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Greater
}

fn f9(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Greater
}

fn f10(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Less
}
