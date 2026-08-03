extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Inner {
    pub x: i32,
    pub y: i32,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: self.x,
            y: self.y,
        };
        this
    }
}
impl ByteRepr for Inner {
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
#[repr(C)]
#[derive(Default)]
pub struct Outer {
    pub inner: Option<Value<Inner>>,
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.inner.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: <Option<Value<Inner>>>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Option<Value<Outer>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Outer {
            inner: Some(Rc::new(RefCell::new(Inner { x: 10, y: 20 }))),
        })))));
    (*(*(*o.borrow_mut()).as_ref().unwrap().borrow_mut())
        .inner
        .as_ref()
        .unwrap()
        .borrow_mut())
    .x += 5;
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((*(*(*o.borrow()).as_ref().unwrap().borrow())
            .inner
            .as_ref()
            .unwrap()
            .borrow())
        .x + (*(*(*o.borrow()).as_ref().unwrap().borrow())
            .inner
            .as_ref()
            .unwrap()
            .borrow())
        .y),
    ));
    let a: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(100)))));
    let b: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let __rhs = (*(*a.borrow()).as_ref().unwrap().borrow());
    (*(*b.borrow_mut()).as_ref().unwrap().borrow_mut()) = __rhs;
    return ((*sum.borrow()) + (*(*b.borrow()).as_ref().unwrap().borrow()));
}
