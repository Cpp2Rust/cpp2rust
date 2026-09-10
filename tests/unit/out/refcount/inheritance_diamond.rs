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
    pub base_A: Value<A>,
}
impl B {
    pub fn B(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new(A::A({ (*x.borrow()) }))),
        }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for B {
    fn clone(&self) -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new((*self.base_A.borrow()).clone())),
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
        (*self.base_A.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_A: Rc::new(RefCell::new(<A>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct C {
    pub base_A: Value<A>,
}
impl C {
    pub fn C(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<C> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new(A::A({ ((*x.borrow()) + 1) }))),
        }));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for C {
    fn clone(&self) -> Self {
        let __this: Value<C> = Rc::new(RefCell::new(Self {
            base_A: Rc::new(RefCell::new((*self.base_A.borrow()).clone())),
        }));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for C {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_A.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_A: Rc::new(RefCell::new(<A>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct D {
    pub base_B: Value<B>,
    pub base_C: Value<C>,
}
impl D {
    pub fn D(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            base_B: Rc::new(RefCell::new(B::B({ (*x.borrow()) }))),
            base_C: Rc::new(RefCell::new(C::C({ (*x.borrow()) }))),
        }));
        let this: Ptr<D> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for D {
    fn clone(&self) -> Self {
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            base_B: Rc::new(RefCell::new((*self.base_B.borrow()).clone())),
            base_C: Rc::new(RefCell::new((*self.base_C.borrow()).clone())),
        }));
        let this: Ptr<D> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for D {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_B.borrow()).to_bytes(&mut buf[0..4]);
        (*self.base_C.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_B: Rc::new(RefCell::new(<B>::from_bytes(&buf[0..4]))),
            base_C: Rc::new(RefCell::new(<C>::from_bytes(&buf[4..8]))),
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
    let d: Value<D> = Rc::new(RefCell::new(D::D({ 1 })));
    assert!((({ DImpl::sum(&d.as_pointer(),) }) == 3));
    let b: Ptr<B> = ((*d.borrow()).base_B.as_pointer());
    let c: Ptr<C> = ((*d.borrow()).base_C.as_pointer());
    assert!((({ geta_0(((*b.upgrade().deref()).base_A.as_pointer()),) }) == 1));
    assert!((({ geta_0(((*c.upgrade().deref()).base_A.as_pointer()),) }) == 2));
    (*(*(*c.upgrade().deref()).base_A.borrow()).a.borrow_mut()) = 5;
    assert!((({ DImpl::sum(&d.as_pointer(),) }) == 6));
    assert!({
        let _lhs = ((*b.upgrade().deref()).base_A.as_pointer());
        _lhs != ((*c.upgrade().deref()).base_A.as_pointer())
    });
    return 0;
}
pub trait DImpl {
    fn sum(&self) -> i32;
}
impl DImpl for Ptr<D> {
    fn sum(&self) -> i32 {
        return ((*(*((*((*(*self).upgrade().deref()).base_B.as_pointer())
            .upgrade()
            .deref())
        .base_A
        .as_pointer())
        .upgrade()
        .deref())
        .a
        .borrow())
            + (*(*((*((*(*self).upgrade().deref()).base_C.as_pointer())
                .upgrade()
                .deref())
            .base_A
            .as_pointer())
            .upgrade()
            .deref())
            .a
            .borrow()));
    }
}
