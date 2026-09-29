extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct Triple {
    pub a: Value<i32>,
    pub b: Value<bool>,
    pub c: Value<i32>,
}
impl Clone for Triple {
    fn clone(&self) -> Self {
        let __this: Value<Triple> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
            c: Rc::new(RefCell::new((*self.c.borrow()))),
        }));
        let this: Ptr<Triple> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Triple {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..5]);
        (*self.c.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<bool>::from_bytes(&buf[4..5]))),
            c: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn sum_0(t: Ptr<Triple>) -> i32 {
    let __decomp_1: Ptr<Triple> = (t).clone();
    return (((*(*__decomp_1.upgrade().deref()).a.borrow())
        + (if (*(*__decomp_1.upgrade().deref()).b.borrow()) {
            1
        } else {
            0
        }))
        + (*(*__decomp_1.upgrade().deref()).c.borrow()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Triple> = Rc::new(RefCell::new(Triple {
        a: Rc::new(RefCell::new(10)),
        b: Rc::new(RefCell::new(false)),
        c: Rc::new(RefCell::new(20)),
    }));
    let __decomp_2: Ptr<Triple> = t.as_pointer();
    (*(*__decomp_2.upgrade().deref()).a.borrow_mut()) = 11;
    (*(*__decomp_2.upgrade().deref()).b.borrow_mut()) = true;
    (*(*__decomp_2.upgrade().deref()).c.borrow_mut()) += 1;
    assert!(((*(*t.borrow()).a.borrow()) == 11));
    assert!((*(*t.borrow()).b.borrow()));
    assert!(((*(*t.borrow()).c.borrow()) == 21));
    (*(*t.borrow()).a.borrow_mut()) = 12;
    assert!(((*(*__decomp_2.upgrade().deref()).a.borrow()) == 12));
    let __decomp_3: Ptr<Triple> = t.as_pointer();
    (*(*t.borrow()).c.borrow_mut()) = 30;
    assert!(((*(*__decomp_3.upgrade().deref()).a.borrow()) == 12));
    assert!((*(*__decomp_3.upgrade().deref()).b.borrow()));
    assert!(((*(*__decomp_3.upgrade().deref()).c.borrow()) == 30));
    assert!((({ sum_0(t.as_pointer(),) }) == 43));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
