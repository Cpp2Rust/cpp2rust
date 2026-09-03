extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct S {
    pub a: Value<i32>,
    pub b: Value<bool>,
}
impl S {
    pub fn S() -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new(11)),
            b: Rc::new(RefCell::new(true)),
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
impl Default for S {
    fn default() -> Self {
        { S::S() }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<bool>::from_bytes(&buf[4..5]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::S()));
    assert!(((*(*s.borrow()).a.borrow()) == 11));
    assert!((((*(*s.borrow()).b.borrow()) as i32) == (true as i32)));
    return 0;
}
