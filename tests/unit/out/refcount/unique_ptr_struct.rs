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
pub fn sum_0(p: Point) -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(p));
    return ((*(*p.borrow()).x.borrow()) + (*(*p.borrow()).y.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Option<Value<Point>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Point {
            x: Rc::new(RefCell::new(3)),
            y: Rc::new(RefCell::new(4)),
        })))));
    (*(*(*p.borrow()).as_ref().unwrap().borrow()).x.borrow_mut()) += 10;
    let __rhs = ((*(*(*p.borrow()).as_ref().unwrap().borrow()).x.borrow())
        + (*(*(*p.borrow()).as_ref().unwrap().borrow()).y.borrow()));
    (*(*(*p.borrow()).as_ref().unwrap().borrow()).y.borrow_mut()) = __rhs;
    let s: Value<i32> = Rc::new(RefCell::new(
        ({ sum_0((*(*p.borrow()).as_ref().unwrap().borrow()).clone()) }),
    ));
    return (*s.borrow());
}
