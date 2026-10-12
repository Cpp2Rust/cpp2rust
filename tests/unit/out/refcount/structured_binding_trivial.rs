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
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut __decomp_0: Pair = Pair {
        first: 1,
        second: 2,
    };
    assert!((__decomp_0.first == 1));
    assert!((__decomp_0.second == 2));
    let mut p: Pair = Pair {
        first: 10,
        second: 20,
    };
    let mut __decomp_1: Pair = (p).clone();
    __decomp_1.first = 11;
    __decomp_1.second += 1;
    assert!((__decomp_1.first == 11));
    assert!((__decomp_1.second == 21));
    assert!((p.first == 10));
    assert!((p.second == 20));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
