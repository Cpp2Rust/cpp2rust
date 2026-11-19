// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// The patterns take any unsigned type; u32 only type-checks the bodies.

unsafe fn f1(a0: u32) -> bool {
    a0.is_power_of_two()
}

unsafe fn f2(a0: u32) -> u32 {
    a0.next_power_of_two()
}

unsafe fn f3(a0: u32) -> u32 {
    a0.isolate_highest_one()
}

unsafe fn f4(a0: u32) -> i32 {
    a0.bit_width() as i32
}

// A negative count rotates the other way, as a count modulo the width.
unsafe fn f5(a0: u32, a1: i32) -> u32 {
    a0.rotate_left(a1 as u32)
}

unsafe fn f6(a0: u32, a1: i32) -> u32 {
    a0.rotate_right(a1 as u32)
}

unsafe fn f7(a0: u32) -> i32 {
    a0.leading_zeros() as i32
}

unsafe fn f8(a0: u32) -> i32 {
    a0.leading_ones() as i32
}

unsafe fn f9(a0: u32) -> i32 {
    a0.trailing_zeros() as i32
}

unsafe fn f10(a0: u32) -> i32 {
    a0.trailing_ones() as i32
}

unsafe fn f11(a0: u32) -> i32 {
    a0.count_ones() as i32
}
