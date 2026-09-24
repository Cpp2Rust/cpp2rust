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
    pub step: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            n: Rc::new(RefCell::new((*self.n.borrow()))),
            step: Rc::new(RefCell::new((*self.step.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
        (*self.step.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            step: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        n: Rc::new(RefCell::new(0)),
        step: Rc::new(RefCell::new(2)),
    }));
    ({ SImpl::bump(&s.as_pointer(), 3) });
    assert!(((*(*s.borrow()).n.borrow()) == 6));
    ({ SImpl::bump_via_method(&s.as_pointer(), 4) });
    assert!(((*(*s.borrow()).n.borrow()) == 10));
    assert!((({ SImpl::read_scaled(&s.as_pointer(),) }) == 20));
    return 0;
}
pub trait SImpl {
    fn add(&self, k: i32);
    fn scaled(&self) -> i32;
    fn bump(&self, by: i32);
    fn bump_via_method(&self, by: i32);
    fn read_scaled(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn add(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        (*(*(*self).upgrade().deref()).n.borrow_mut()) += (*k.borrow());
    }
    fn scaled(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).n.borrow())
            * (*(*(*self).upgrade().deref()).step.borrow()));
    }
    fn bump(&self, by: i32) {
        let by: Value<i32> = Rc::new(RefCell::new(by));
        let inc: Value<_> = Rc::new(RefCell::new(
            (|k: i32| {
                let k: Value<i32> = Rc::new(RefCell::new(k));
                (*self.n.borrow_mut()) += (*k.borrow());
            }),
        ));
        ({ (*inc.borrow_mut())((*by.borrow())) });
        ({ (*inc.borrow_mut())((*by.borrow())) });
    }
    fn bump_via_method(&self, by: i32) {
        let by: Value<i32> = Rc::new(RefCell::new(by));
        let inc: Value<_> = Rc::new(RefCell::new(
            (|k: i32| {
                let k: Value<i32> = Rc::new(RefCell::new(k));
                ({ SImpl::add(&(*self), (*k.borrow())) });
            }),
        ));
        ({ (*inc.borrow_mut())((*by.borrow())) });
    }
    fn read_scaled(&self) -> i32 {
        let get: Value<_> = Rc::new(RefCell::new(
            (|| {
                return ({ SImpl::scaled(&(*self)) });
            }),
        ));
        return ({ (*get.borrow_mut())() }).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
