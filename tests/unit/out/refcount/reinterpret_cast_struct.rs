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
    pub x: i32,
    #[offset(4)]
    pub y: i32,
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
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(<Point>::default()));
    (*p.borrow_mut()).x = 67305985;
    (*p.borrow_mut()).y = 134678021;
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((p.as_pointer()).reinterpret_cast::<u8>()));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == 1));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == 4));
    assert!(((((*bytes.borrow()).offset((4) as isize).read()) as i32) == 5));
    assert!(((((*bytes.borrow()).offset((7) as isize).read()) as i32) == 8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
