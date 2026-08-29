extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl S {
    pub fn S(a: i32, b: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        let mut this = Self {
            a: Rc::new(RefCell::new((*a.borrow()))),
            b: Rc::new(RefCell::new((*b.borrow()))),
        };
        this
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        };
        this
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(S::S({ 1 }, { 2 })));
    let s2: Ptr<S> = s1.as_pointer();
    assert!(((*(*s2.upgrade().deref()).a.borrow()) == 1));
    assert!(((*(*s2.upgrade().deref()).b.borrow()) == 2));
    return 0;
}
