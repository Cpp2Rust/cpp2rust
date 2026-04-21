extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Foo {
    pub x1: Value<i32>,
    pub x2: Value<i32>,
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
            x2: Rc::new(RefCell::new((*self.x2.borrow()))),
        };
        this
    }
}
impl ByteRepr for Foo {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo {
            x1: Rc::new(RefCell::new(1)),
            x2: Rc::new(RefCell::new(2)),
        },
        Foo {
            x1: Rc::new(RefCell::new(3)),
            x2: Rc::new(RefCell::new(4)),
        },
    ])));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((*arr.borrow())[(1) as usize].x1.as_pointer()),
    ));
    let a: Value<i32> = Rc::new(RefCell::new(((*p1.borrow()).read())));
    let p2: Value<Ptr<Foo>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<Foo>).offset(0 as isize)),
    ));
    return {
        let _lhs = (*a.borrow());
        _lhs + (*(*(*p2.borrow()).upgrade().deref()).x2.borrow())
    };
}
