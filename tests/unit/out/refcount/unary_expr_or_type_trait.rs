extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub c: u8,
    #[offset(8)]
    pub x: i64,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.c.to_bytes(&mut buf[0..1]);
        self.x.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            c: <u8>::from_bytes(&buf[0..1]),
            x: <i64>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn pack_size_0() -> u64 {
    return ((0 as usize).wrapping_add((0 as usize)) as u64);
}
pub fn pack_size_1(args_0: i32, args_1: f64) -> u64 {
    let args_0: Value<i32> = Rc::new(RefCell::new(args_0));
    let args_1: Value<f64> = Rc::new(RefCell::new(args_1));
    return ((2 as usize).wrapping_add((2 as usize)) as u64);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([0_i64, 0_i64, 0_i64, 0_i64])));
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    assert!((::std::mem::size_of::<i32>() == 4_usize));
    assert!((::std::mem::size_of::<[i64; 4]>() == 32_usize));
    assert!((16usize == 16_usize));
    assert!((::std::mem::align_of::<i32>() == 4_usize));
    assert!((16usize == 16_usize));
    assert!((::std::mem::align_of::<[i64; 4]>() == 8_usize));
    assert!((16usize == 16_usize));
    assert!((({ pack_size_0() }) == 0_u64));
    assert!((({ pack_size_1(1, 2.0E+0,) }) == 4_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
