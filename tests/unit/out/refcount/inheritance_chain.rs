extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct A {
    pub a: Value<i32>,
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
        }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for A {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct B {
    pub base_A: Value<A>,
    pub b: Value<i32>,
}
impl B {
    pub fn B(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new(<A>::default())),
            b: Rc::new(RefCell::new(((*x.borrow()) + 1))),
        }));
        let this: Ptr<B> = __this.as_pointer();
        (*(*((*this.upgrade().deref()).base_A.as_pointer())
            .upgrade()
            .deref())
        .a
        .borrow_mut()) = (*x.borrow());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for B {
    fn clone(&self) -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new((*self.base_A.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for B {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_A.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_A: Rc::new(RefCell::new(<A>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct C {
    pub base_B: Value<B>,
}
impl C {
    pub fn C(_a0: i32) -> Self {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let __this: Value<C> = Rc::new(RefCell::new(Self {
            base_B: Rc::new(RefCell::new(B::B({ (*_a0.borrow()) }))),
        }));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for C {
    fn clone(&self) -> Self {
        let __this: Value<C> = Rc::new(RefCell::new(Self {
            base_B: Rc::new(RefCell::new((*self.base_B.borrow()).clone())),
        }));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for C {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_B.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_B: Rc::new(RefCell::new(<B>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn geta_0(x: Ptr<A>) -> i32 {
    return (*(*x.upgrade().deref()).a.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<C> = Rc::new(RefCell::new(C::C({ 1 })));
    assert!((({ CImpl::sum(&c.as_pointer(),) }) == 3));
    assert!((({ geta_0((*(*c.borrow()).base_B.borrow()).base_A.as_pointer(),) }) == 1));
    return 0;
}
pub trait CImpl {
    fn sum(&self) -> i32;
}
impl CImpl for Ptr<C> {
    fn sum(&self) -> i32 {
        return ((*(*((*(*(*self).upgrade().deref()).base_B.borrow())
            .base_A
            .as_pointer())
        .upgrade()
        .deref())
        .a
        .borrow())
            + (*(*((*(*self).upgrade().deref()).base_B.as_pointer())
                .upgrade()
                .deref())
            .b
            .borrow()));
    }
}
