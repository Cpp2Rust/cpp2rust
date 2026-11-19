extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<u32> = Rc::new(RefCell::new((-1_i32 as u32)));
    assert!(({ ((*x.borrow()) == <u32>::MAX) }));
    let mut v1: i32 = (((*x.borrow()) & 1_u32) as i32);
    let mut v2: u32 = ((*x.borrow()) & 1_u32);
    let mut p: Ptr<u32> = (x.as_pointer());
    let mut b: bool = (((p.read()) & 255_u32) != 0);
    let mut a: i32 = (((p.read()) & 255_u32) as i32);
    assert!((a == 255));
    assert!((0_u32.bit_width() as i32 == 0) && (({ (*x.borrow()) }).bit_width() as i32 == 32));
    assert!((0_u32.isolate_highest_one() == 0_u32) && (100_u32.isolate_highest_one() == 64_u32));
    assert!((0_u32.next_power_of_two() == 1_u32) && (100_u32.next_power_of_two() == 128_u32));
    assert!((64_u32.is_power_of_two()) && (!(0_u32.is_power_of_two())));
    let mut byte: u8 = 129_u8;
    assert!(
        (((byte.rotate_left(1 as u32)) as i32) == 3)
            && (((byte.rotate_left(-1_i32 as u32)) as i32) == 192)
    );
    assert!(
        (((byte.rotate_right(1 as u32)) as i32) == 192)
            && (((byte.rotate_right(9 as u32)) as i32) == 192)
    );
    assert!((1_u8.leading_zeros() as i32 == 7) && (byte.leading_ones() as i32 == 1));
    let mut wide: u64 = 240_u64;
    assert!((wide.trailing_zeros() as i32 == 4) && ((wide >> 4).trailing_ones() as i32 == 4));
    assert!((wide.count_ones() as i32 == 4) && (0_u64.leading_zeros() as i32 == 64));
    let mut floor_byte: u8 = byte.isolate_highest_one();
    let mut ceil_wide: u64 = (wide).wrapping_add(1_u64).next_power_of_two();
    let mut rot_wide: u64 = wide.rotate_left(60 as u32);
    assert!((((floor_byte as i32) == 128) && (ceil_wide == 256_u64)) && (rot_wide == 15_u64));
    let mut ul: u64 = ((1_u64 << 40) | 1_u64);
    assert!((ul.count_ones() as i32 == 2) && (ul.leading_zeros() as i32 == 23));
    assert!((ul.bit_width() as i32 == 41) && (ul.isolate_highest_one() == (1_u64 << 40)));
    assert!((ul.rotate_right(1 as u32) == ((1_u64 << 39) | (1_u64 << 63))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
