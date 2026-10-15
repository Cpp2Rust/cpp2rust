extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: Value<i32>,
    pub second: Value<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new((*self.first.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.first.borrow()).to_bytes(&mut buf[0..4]);
        (*self.second.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            second: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __decomp_0: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: Rc::new(RefCell::new(1)),
        second: Rc::new(RefCell::new(2)),
    }));
    assert!(((*(*__decomp_0.borrow()).first.borrow()) == 1));
    assert!(((*(*__decomp_0.borrow()).second.borrow()) == 2));
    let p: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: Rc::new(RefCell::new(10)),
        second: Rc::new(RefCell::new(20)),
    }));
    let __decomp_1: Value<Pair> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*(*__decomp_1.borrow()).first.borrow_mut()) = 11;
    (*(*__decomp_1.borrow()).second.borrow_mut()) += 1;
    assert!(((*(*__decomp_1.borrow()).first.borrow()) == 11));
    assert!(((*(*__decomp_1.borrow()).second.borrow()) == 21));
    assert!(((*(*p.borrow()).first.borrow()) == 10));
    assert!(((*(*p.borrow()).second.borrow()) == 20));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
