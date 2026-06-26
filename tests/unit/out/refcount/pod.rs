extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct POD {
    pub x1: Value<i32>,
    pub x2: Value<i32>,
    pub x3: Value<i32>,
}
impl Clone for POD {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
            x2: Rc::new(RefCell::new((*self.x2.borrow()))),
            x3: Rc::new(RefCell::new((*self.x3.borrow()))),
        };
        this
    }
}
impl ByteRepr for POD {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x1.borrow()).to_bytes(&mut buf[0..4]);
        (*self.x2.borrow()).to_bytes(&mut buf[4..8]);
        (*self.x3.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            x2: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            x3: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn PODIncrement_0(pod: Ptr<POD>) {
    (*(*pod.upgrade().deref()).x1.borrow_mut()) += 1;
    (*(*pod.upgrade().deref()).x2.borrow_mut()) += 2;
    (*(*pod.upgrade().deref()).x3.borrow_mut()) += 3;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<POD> = Rc::new(RefCell::new(POD {
        x1: Rc::new(RefCell::new(10)),
        x2: Rc::new(RefCell::new(11)),
        x3: Rc::new(RefCell::new(12)),
    }));
    let p2: Value<POD> = Rc::new(RefCell::new(POD {
        x1: Rc::new(RefCell::new((*(*p1.borrow()).x1.borrow()))),
        x2: Rc::new(RefCell::new((*(*p1.borrow()).x2.borrow()))),
        x3: Rc::new(RefCell::new((*(*p1.borrow()).x3.borrow()))),
    }));
    ({ PODIncrement_0(p2.as_pointer()) });
    return (((*(*p2.borrow()).x1.borrow()) + (*(*p2.borrow()).x2.borrow()))
        + (*(*p2.borrow()).x3.borrow()));
}
