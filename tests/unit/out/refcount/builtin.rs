extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_expect_0() {
    let x: Value<i32> = Rc::new(RefCell::new(42));
    assert!((((((*x.borrow()) == 42) as i32) as i64) != 0));
}
pub fn test_ctz_1() {
    assert!((((8_u32.trailing_zeros() as i32 == 3) as i32) != 0));
    assert!((((1_u32.trailing_zeros() as i32 == 0) as i32) != 0));
}
pub fn test_clz_2() {
    assert!((((1_u32.leading_zeros() as i32 == 31) as i32) != 0));
    assert!((((2147483648_u32.leading_zeros() as i32 == 0) as i32) != 0));
}
pub fn test_bswap16_3() {
    assert!((((((4660 as u16).swap_bytes() as i32) == 13330) as i32) != 0));
}
pub fn test_bswap32_4() {
    assert!((((305419896_u32.swap_bytes() == 2018915346_u32) as i32) != 0));
}
pub fn test_bswap64_5() {
    assert!((((81985529216486895_u64.swap_bytes() == 17279655951921914625_u64) as i32) != 0));
}
pub fn test_ctzl_6() {
    assert!((((8_u64.trailing_zeros() as i32 == 3) as i32) != 0));
    assert!((((1_u64.trailing_zeros() as i32 == 0) as i32) != 0));
}
pub fn test_popcountl_7() {
    assert!((((0_u64.count_ones() as i32 == 0) as i32) != 0));
    assert!((((255_u64.count_ones() as i32 == 8) as i32) != 0));
}
pub fn test_mul_overflow_long_8() {
    let r: Value<i64> = Rc::new(RefCell::new(0_i64));
    assert!(
        ((!{
            let (val, ovf) = 3_i64.overflowing_mul(7_i64);
            (r.as_pointer()).write(val);
            ovf
        } as i32)
            != 0)
    );
    assert!(((((*r.borrow()) == 21_i64) as i32) != 0));
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        (r.as_pointer()).write(val);
        ovf
    });
}
pub fn test_mul_overflow_long_long_9() {
    let r: Value<i64> = Rc::new(RefCell::new(0_i64));
    assert!(
        ((!{
            let (val, ovf) = 1000_i64.overflowing_mul(1000_i64);
            (r.as_pointer()).write(val);
            ovf
        } as i32)
            != 0)
    );
    assert!(((((*r.borrow()) == 1000000_i64) as i32) != 0));
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        (r.as_pointer()).write(val);
        ovf
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_expect_0() });
    ({ test_ctz_1() });
    ({ test_clz_2() });
    ({ test_bswap16_3() });
    ({ test_bswap32_4() });
    ({ test_bswap64_5() });
    ({ test_ctzl_6() });
    ({ test_popcountl_7() });
    ({ test_mul_overflow_long_8() });
    ({ test_mul_overflow_long_long_9() });
    return 0;
}
