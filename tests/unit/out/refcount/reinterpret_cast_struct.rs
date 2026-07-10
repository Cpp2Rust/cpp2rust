extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(<Point>::default()));
    (*(*p.borrow()).x.borrow_mut()) = 67305985;
    (*(*p.borrow()).y.borrow_mut()) = 134678021;
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((p.as_pointer()).reinterpret_cast::<u8>()));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == 1));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == 4));
    assert!(((((*bytes.borrow()).offset((4) as isize).read()) as i32) == 5));
    assert!(((((*bytes.borrow()).offset((7) as isize).read()) as i32) == 8));
    return 0;
}
