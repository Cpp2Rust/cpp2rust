extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn agg_0(mut n: i32) -> i32 {
    let mut buf40: [i8; 40] = [0_i8; 40];
    let mut buf256: [u8; 256] = [0_u8; 256];
    let mut arr64: [i32; 64] = [0_i32; 64];
    let mut longs: [i64; 33] = [0_i64; 33];
    let mut p: Point = <Point>::default();
    let mut ptr: Ptr<i32> = Ptr::<i32>::null();
    let mut fp: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::null();
    let mut file: Ptr<CFile> = Ptr::null();
    let mut total: i32 = 0_i32;
    goto_block!({
        '__entry: {
            total = 0;
            if (((n < 0) as i32) != 0) {
                goto!('out);
            }
            total = 1;
        }
        'out: {
            return total;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ agg_0(-1_i32,) }) == 0) as i32) != 0));
    assert!((((({ agg_0(1,) }) == 1) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
