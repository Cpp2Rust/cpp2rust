extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct A {
    pub v: Value<i32>,
}
impl A {
    pub fn A1() -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(1)),
        }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn A2(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for A {
    fn default() -> Self {
        { A::A1() }
    }
}
impl ByteRepr for A {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive()]
pub struct B {
    pub v: Value<i32>,
}
impl B {
    pub fn B() -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(2)),
        }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for B {
    fn clone(&self) -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for B {
    fn default() -> Self {
        { B::B() }
    }
}
impl ByteRepr for B {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct NoDefault {
    pub v: Value<i32>,
}
impl NoDefault {
    pub fn NoDefault(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<NoDefault> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<NoDefault> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for NoDefault {
    fn clone(&self) -> Self {
        let __this: Value<NoDefault> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<NoDefault> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for NoDefault {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn used_0(x: Option<A>) -> i32 {
    let x: Value<A> = Rc::new(RefCell::new(x.unwrap_or(A::A1())));
    return (*(*x.borrow()).v.borrow());
}
pub fn used_1(x: Option<B>) -> i32 {
    let x: Value<B> = Rc::new(RefCell::new(x.unwrap_or(B::B())));
    return (*(*x.borrow()).v.borrow());
}
pub fn scaled_2(x: A, n: Option<i32>) -> i32 {
    let x: Value<A> = Rc::new(RefCell::new(x));
    let n: Value<i32> = Rc::new(RefCell::new(n.unwrap_or((4usize as i32))));
    return ((*(*x.borrow()).v.borrow()) * (*n.borrow()));
}
pub fn always_given_3(x: NoDefault) -> i32 {
    let x: Value<NoDefault> = Rc::new(RefCell::new(x));
    return (*(*x.borrow()).v.borrow());
}
#[derive(Default)]
pub struct S_NoDefault_ {
    pub v: Value<i32>,
}
impl S_NoDefault_ {
    pub fn S_NoDefault_(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<S_NoDefault_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<S_NoDefault_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for S_NoDefault_ {
    fn clone(&self) -> Self {
        let __this: Value<S_NoDefault_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<S_NoDefault_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S_NoDefault_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ used_0(None,) }) == 1));
    assert!((({ used_0(Some(A::A2({ 5 },)),) }) == 5));
    assert!((({ used_1(None,) }) == 2));
    assert!((({ scaled_2(A::A2({ 3 },), None,) }) == (3 * (4usize as i32))));
    assert!((({ scaled_2(A::A2({ 3 },), Some(2),) }) == 6));
    assert!((({ always_given_3(NoDefault::NoDefault({ 3 },),) }) == 3));
    let s: Value<S_NoDefault_> = Rc::new(RefCell::new(S_NoDefault_::S_NoDefault_({ 1 })));
    assert!((({ S_NoDefault_Impl::get(&s.as_pointer(), NoDefault::NoDefault({ 4 },),) }) == 5));
    return 0;
}
pub trait S_NoDefault_Impl {
    fn get(&self, t: NoDefault) -> i32;
}
impl S_NoDefault_Impl for Ptr<S_NoDefault_> {
    fn get(&self, t: NoDefault) -> i32 {
        let t: Value<NoDefault> = Rc::new(RefCell::new(t));
        return ((*(*(*self).upgrade().deref()).v.borrow()) + (*(*t.borrow()).v.borrow()));
    }
}
pub fn __cpp2rust_init_globals() {}
