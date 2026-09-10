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
impl A {
    pub fn A(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*x.borrow()))),
        }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
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
    pub b: Value<i32>,
}
impl B {
    pub fn B(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            b: Rc::new(RefCell::new((*x.borrow()))),
        }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for B {
    fn clone(&self) -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for B {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.b.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct C {
    pub base_A: Value<A>,
    pub base_B: Value<B>,
    pub c: Value<i32>,
}
impl C {
    pub fn C(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<C> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new(A::A({ (*x.borrow()) }))),
            base_B: Rc::new(RefCell::new(B::B({ ((*x.borrow()) + 1) }))),
            c: Rc::new(RefCell::new(((*x.borrow()) + 2))),
        }));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for C {
    fn clone(&self) -> Self {
        let __this: Value<C> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new((*self.base_A.borrow()).clone())),
            base_B: Rc::new(RefCell::new((*self.base_B.borrow()).clone())),
            c: Rc::new(RefCell::new((*self.c.borrow()))),
        }));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for C {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_A.borrow()).to_bytes(&mut buf[0..4]);
        (*self.base_B.borrow()).to_bytes(&mut buf[4..8]);
        (*self.c.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_A: Rc::new(RefCell::new(<A>::from_bytes(&buf[0..4]))),
            base_B: Rc::new(RefCell::new(<B>::from_bytes(&buf[4..8]))),
            c: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn geta_0(x: Ptr<A>) -> i32 {
    return (*(*x.upgrade().deref()).a.borrow());
}
pub fn getb_1(x: Ptr<B>) -> i32 {
    let x: Value<Ptr<B>> = Rc::new(RefCell::new(x));
    return (*(*(*x.borrow()).upgrade().deref()).b.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<C> = Rc::new(RefCell::new(C::C({ 1 })));
    assert!((({ CImpl::sum(&c.as_pointer(),) }) == 6));
    assert!((({ geta_0((*c.borrow()).base_A.as_pointer(),) }) == 1));
    assert!((({ getb_1(((*(c.as_pointer()).upgrade().deref()).base_B.as_pointer()),) }) == 2));
    let pb: Value<Ptr<B>> = Rc::new(RefCell::new(
        ((*(c.as_pointer()).upgrade().deref()).base_B.as_pointer()),
    ));
    (*(*(*pb.borrow()).upgrade().deref()).b.borrow_mut()) = 10;
    assert!(((*(*(*c.borrow()).base_B.borrow()).b.borrow()) == 10));
    assert!((({ CImpl::sum(&c.as_pointer(),) }) == 14));
    return 0;
}
pub trait CImpl {
    fn sum(&self) -> i32;
}
impl CImpl for Ptr<C> {
    fn sum(&self) -> i32 {
        return (((*(*((*(*self).upgrade().deref()).base_A.as_pointer())
            .upgrade()
            .deref())
        .a
        .borrow())
            + (*(*((*(*self).upgrade().deref()).base_B.as_pointer())
                .upgrade()
                .deref())
            .b
            .borrow()))
            + (*(*(*self).upgrade().deref()).c.borrow()));
    }
}
