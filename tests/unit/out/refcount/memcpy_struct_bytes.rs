extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct point {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for point {
    fn clone(&self) -> Self {
        Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for point {
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
    let src: Value<point> = Rc::new(RefCell::new(point {
        x: Rc::new(RefCell::new(3)),
        y: Rc::new(RefCell::new(7)),
    }));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((src.as_pointer()) as Ptr<point>).to_any(),
            ::std::mem::size_of::<[u8; 8]>() as usize,
        );
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    let dst: Value<point> = <Value<point>>::default();
    {
        ((dst.as_pointer()) as Ptr<point>).to_any().memcpy(
            &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            8usize as usize,
        );
        ((dst.as_pointer()) as Ptr<point>).to_any().clone()
    };
    assert!(((((*(*dst.borrow()).x.borrow()) == 3) as i32) != 0));
    assert!(((((*(*dst.borrow()).y.borrow()) == 7) as i32) != 0));
    return 0;
}
