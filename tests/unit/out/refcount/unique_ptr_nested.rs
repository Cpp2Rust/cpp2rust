extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl ByteRepr for Inner {}
#[derive(Default)]
pub struct Outer {
    pub inner: Value<Option<Value<Inner>>>,
}
impl ByteRepr for Outer {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Option<Value<Outer>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Outer {
            inner: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Inner {
                x: Rc::new(RefCell::new(10)),
                y: Rc::new(RefCell::new(20)),
            }))))),
        })))));
    (*(*(*(*(*o.borrow()).as_ref().unwrap().borrow()).inner.borrow())
        .as_ref()
        .unwrap()
        .borrow())
    .x
    .borrow_mut()) += 5;
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((*(*(*(*(*o.borrow()).as_ref().unwrap().borrow()).inner.borrow())
            .as_ref()
            .unwrap()
            .borrow())
        .x
        .borrow())
            + (*(*(*(*(*o.borrow()).as_ref().unwrap().borrow()).inner.borrow())
                .as_ref()
                .unwrap()
                .borrow())
            .y
            .borrow())),
    ));
    let a: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(100)))));
    let b: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let __rhs = (*(*a.borrow()).as_ref().unwrap().borrow());
    (*(*b.borrow_mut()).as_ref().unwrap().borrow_mut()) = __rhs;
    return ((*sum.borrow()) + (*(*b.borrow()).as_ref().unwrap().borrow()));
}
