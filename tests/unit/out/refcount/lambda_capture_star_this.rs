extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub n: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            n: Rc::new(RefCell::new((*self.n.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        n: Rc::new(RefCell::new(1)),
    }));
    assert!((({ SImpl::modify_copy(&s.as_pointer(),) }) == 1101));
    assert!(((*(*s.borrow()).n.borrow()) == 1));
    assert!((({ SImpl::snapshot(&s.as_pointer(),) }) == 2));
    assert!(((*(*s.borrow()).n.borrow()) == 99));
    assert!((({ SImpl::mixed(&s.as_pointer(), 1,) }) == 100));
    assert!(((*(*s.borrow()).n.borrow()) == 0));
    return 0;
}
pub trait SImpl {
    fn twice(&self) -> i32;
    fn modify_copy(&self) -> i32;
    fn snapshot(&self) -> i32;
    fn mixed(&self, k: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn twice(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).n.borrow()) * 2);
    }
    fn modify_copy(&self) -> i32 {
        let f: Value<_> = Rc::new(RefCell::new(
            (|| {
                (*self.n.borrow_mut()) += 10;
                return (*self.n.borrow());
            }),
        ));
        let r: Value<i32> = Rc::new(RefCell::new(({ (*f.borrow_mut())() }).clone()));
        return (((*r.borrow()) * 100) + (*(*(*self).upgrade().deref()).n.borrow()));
    }
    fn snapshot(&self) -> i32 {
        let f: Value<_> = Rc::new(RefCell::new(
            (|| {
                return ({ SImpl::twice(&(*self)) });
            }),
        ));
        (*(*(*self).upgrade().deref()).n.borrow_mut()) = 99;
        return ({ (*f.borrow_mut())() }).clone();
    }
    fn mixed(&self, k: i32) -> i32 {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        let f: Value<_> = Rc::new(RefCell::new(
            (|| {
                return ((*self.n.borrow()) + (*k.borrow()));
            }),
        ));
        (*(*(*self).upgrade().deref()).n.borrow_mut()) = 0;
        return ({ (*f.borrow_mut())() }).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
