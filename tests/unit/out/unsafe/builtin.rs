extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_expect_0() {
    let mut x: i32 = 42;
    assert!((((((x) == (42)) as i32) as i64) != 0));
}
pub unsafe fn test_ctz_1() {
    assert!(((((8_u32.trailing_zeros() as i32) == (3)) as i32) != 0));
    assert!(((((1_u32.trailing_zeros() as i32) == (0)) as i32) != 0));
}
pub unsafe fn test_clz_2() {
    assert!(((((1_u32.leading_zeros() as i32) == (31)) as i32) != 0));
    assert!(((((2147483648_u32.leading_zeros() as i32) == (0)) as i32) != 0));
}
pub unsafe fn test_bswap16_3() {
    assert!((((((4660 as u16).swap_bytes() as i32) == (13330)) as i32) != 0));
}
pub unsafe fn test_bswap32_4() {
    assert!(((((305419896_u32.swap_bytes()) == (2018915346_u32)) as i32) != 0));
}
pub unsafe fn test_bswap64_5() {
    assert!(((((81985529216486895_u64.swap_bytes()) == (17279655951921914625_u64)) as i32) != 0));
}
pub unsafe fn test_ctzl_6() {
    assert!(((((8_u64.trailing_zeros() as i32) == (3)) as i32) != 0));
    assert!(((((1_u64.trailing_zeros() as i32) == (0)) as i32) != 0));
}
pub unsafe fn test_popcountl_7() {
    assert!(((((0_u64.count_ones() as i32) == (0)) as i32) != 0));
    assert!(((((255_u64.count_ones() as i32) == (8)) as i32) != 0));
}
pub unsafe fn test_mul_overflow_long_8() {
    let mut r: i64 = 0_i64;
    assert!(
        ((!{
            let (val, ovf) = 3_i64.overflowing_mul(7_i64);
            *(&mut r as *mut i64) = val;
            ovf
        } as i32)
            != 0)
    );
    assert!(((((r) == (21_i64)) as i32) != 0));
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        *(&mut r as *mut i64) = val;
        ovf
    });
}
pub unsafe fn test_mul_overflow_long_long_9() {
    let mut r: i64 = 0_i64;
    assert!(
        ((!{
            let (val, ovf) = 1000_i64.overflowing_mul(1000_i64);
            *(&mut r as *mut i64) = val;
            ovf
        } as i32)
            != 0)
    );
    assert!(((((r) == (1000000_i64)) as i32) != 0));
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        *(&mut r as *mut i64) = val;
        ovf
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_expect_0() });
    (unsafe { test_ctz_1() });
    (unsafe { test_clz_2() });
    (unsafe { test_bswap16_3() });
    (unsafe { test_bswap32_4() });
    (unsafe { test_bswap64_5() });
    (unsafe { test_ctzl_6() });
    (unsafe { test_popcountl_7() });
    (unsafe { test_mul_overflow_long_8() });
    (unsafe { test_mul_overflow_long_long_9() });
    return 0;
}
