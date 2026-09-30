extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Point {
    #[offset(0)]
    pub x: u32,
    #[offset(4)]
    pub y: u32,
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <u32>::from_bytes(&buf[0..4]),
            y: <u32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Pair {
    #[offset(0)]
    pub first: u32,
    #[offset(4)]
    pub second: u32,
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..4]);
        self.second.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <u32>::from_bytes(&buf[0..4]),
            second: <u32>::from_bytes(&buf[4..8]),
        }
    }
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
    let pair: Value<Ptr<Pair>> =
        Rc::new(RefCell::new((pt.as_pointer()).reinterpret_cast::<Pair>()));
    assert!(((*pair.borrow()).with(|__s| __s.first) == 10_u32));
    assert!(((*pair.borrow()).with(|__s| __s.second) == 20_u32));
    field!((*pair.borrow()), first).write(42_u32);
    assert!(({ (*pt.borrow()).x } == 42_u32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
