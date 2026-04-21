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
    pub x: Value<u32>,
    pub y: Value<u32>,
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
impl ByteRepr for Point {}
#[derive(Default)]
pub struct Pair {
    pub first: Value<u32>,
    pub second: Value<u32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            first: Rc::new(RefCell::new((*self.first.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()))),
        };
        this
    }
}
impl ByteRepr for Pair {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let pt: Value<Point> = Rc::new(RefCell::new(Point {
        x: Rc::new(RefCell::new(10_u32)),
        y: Rc::new(RefCell::new(20_u32)),
    }));
    let pair: Value<Ptr<Pair>> =
        Rc::new(RefCell::new((pt.as_pointer()).reinterpret_cast::<Pair>()));
    assert!(((*(*(*pair.borrow()).upgrade().deref()).first.borrow()) == 10_u32));
    assert!(((*(*(*pair.borrow()).upgrade().deref()).second.borrow()) == 20_u32));
    (*(*(*pair.borrow()).upgrade().deref()).first.borrow_mut()) = 42_u32;
    assert!(((*(*pt.borrow()).x.borrow()) == 42_u32));
    return 0;
}
