extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct iter {
    pub p: Value<Ptr<i32>>,
}
impl Clone for iter {
    fn clone(&self) -> Self {
        let mut this = Self {
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for iter {}
#[derive(Default)]
pub struct const_iter {
    pub p: Value<Ptr<i32>>,
}
impl const_iter {
    pub fn const_iter(o: Ptr<iter>) -> Self {
        let mut this = Self {
            p: Rc::new(RefCell::new((*(*o.upgrade().deref()).p.borrow()).clone())),
        };
        this
    }
}
impl Clone for const_iter {
    fn clone(&self) -> Self {
        let mut this = Self {
            p: Rc::new(RefCell::new(Ptr::<i32>::null())),
        };
        this
    }
}
impl ByteRepr for const_iter {}
pub fn sink_0(i: const_iter) {
    let i: Value<const_iter> = Rc::new(RefCell::new(i));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let it: Value<iter> = Rc::new(RefCell::new(iter {
        p: Rc::new(RefCell::new((buf.as_pointer() as Ptr<i32>))),
    }));
    ({
        let _i: const_iter = const_iter::const_iter(it.as_pointer());
        sink_0(_i)
    });
    return 0;
}
