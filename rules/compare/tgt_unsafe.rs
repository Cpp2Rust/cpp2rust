// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> std::cmp::Ordering {
    std::cmp::Ordering::Equal
}

unsafe fn f1() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_LESS
}

unsafe fn f2() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_EQUAL
}

unsafe fn f3() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_EQUIVALENT
}

unsafe fn f4() -> *const std::cmp::Ordering {
    &raw const libcc2rs::STRONG_ORDERING_GREATER
}

unsafe fn f5(a0: std::cmp::Ordering, a1: std::cmp::Ordering) -> bool {
    a0 == a1
}

unsafe fn f6(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Equal
}

unsafe fn f7(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Less
}

unsafe fn f8(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Greater
}

unsafe fn f9(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Greater
}

unsafe fn f10(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Less
}
