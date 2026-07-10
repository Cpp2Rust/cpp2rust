extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl ByteRepr for Pair {
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
    let p: Value<Ptr<Pair>> = Rc::new(RefCell::new(Ptr::alloc(Pair {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    })));
    let out: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(*p.borrow()).upgrade().deref()).x.borrow());
        _lhs + (*(*(*p.borrow()).upgrade().deref()).y.borrow())
    }));
    (*p.borrow()).delete();
    return (*out.borrow());
}
