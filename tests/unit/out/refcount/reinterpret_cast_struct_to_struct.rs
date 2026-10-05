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
    pub x: u32,
    #[offset(4)]
    pub y: u32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: u32,
    #[offset(4)]
    pub second: u32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let pt: Value<Point> = Rc::new(RefCell::new(Point {
        x: 10_u32,
        y: 20_u32,
    }));
    let mut pair: Ptr<Pair> = (pt.as_pointer()).reinterpret_cast::<Pair>();
    assert!((pair.with(|__s| __s.first) == 10_u32));
    assert!((pair.with(|__s| __s.second) == 20_u32));
    field!(pair, first).write(42_u32);
    assert!(({ (*pt.borrow()).x } == 42_u32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
