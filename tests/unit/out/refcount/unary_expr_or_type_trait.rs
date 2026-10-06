extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub c: i8,
    #[offset(8)]
    pub x: i64,
}
pub fn pack_size_0() -> u64 {
    return ((0 as usize).wrapping_add((0 as usize)) as u64);
}
pub fn pack_size_1(mut args_0: i32, mut args_1: f64) -> u64 {
    return ((2 as usize).wrapping_add((2 as usize)) as u64);
}
pub fn may_throw_2() {}
pub fn no_throw_3() {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut arr: [i64; 4] = [0_i64, 0_i64, 0_i64, 0_i64];
    let mut s: S = <S>::default();
    assert!((::std::mem::size_of::<i32>() == 4_usize));
    assert!((::std::mem::size_of::<[i64; 4]>() == 32_usize));
    assert!((16usize == 16_usize));
    assert!((::std::mem::align_of::<i32>() == 4_usize));
    assert!((16usize == 16_usize));
    assert!((::std::mem::align_of::<[i64; 4]>() == 8_usize));
    assert!((16usize == 16_usize));
    assert!((({ pack_size_0() }) == 0_u64));
    assert!((({ pack_size_1(1, 2.0E+0,) }) == 4_u64));
    assert!(true);
    assert!(!(false));
    let mut may: bool = false;
    assert!(!(may));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
