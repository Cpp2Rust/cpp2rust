extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct list {
    pub head: Value<Ptr<node>>,
    pub size: Value<i32>,
}
impl Clone for list {
    fn clone(&self) -> Self {
        Self {
            head: Rc::new(RefCell::new((*self.head.borrow()).clone())),
            size: Rc::new(RefCell::new((*self.size.borrow()).clone())),
        }
    }
}
impl ByteRepr for list {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.head.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: Rc::new(RefCell::new(<Ptr<node>>::from_bytes(&buf[0..8]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct node {
    pub value: Value<i32>,
    pub next: Value<Ptr<node>>,
}
impl Clone for node {
    fn clone(&self) -> Self {
        Self {
            value: Rc::new(RefCell::new((*self.value.borrow()).clone())),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        }
    }
}
impl ByteRepr for node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
        (*self.next.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            next: Rc::new(RefCell::new(<Ptr<node>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = Rc::new(RefCell::new(node {
        value: Rc::new(RefCell::new(42)),
        next: Rc::new(RefCell::new(Ptr::<node>::null())),
    }));
    let l: Value<list> = Rc::new(RefCell::new(list {
        head: Rc::new(RefCell::new((n.as_pointer()))),
        size: Rc::new(RefCell::new(1)),
    }));
    assert!(
        ((((*(*(*(*l.borrow()).head.borrow()).upgrade().deref())
            .value
            .borrow())
            == 42) as i32)
            != 0)
    );
    assert!(((((*(*l.borrow()).size.borrow()) == 1) as i32) != 0));
    return 0;
}
