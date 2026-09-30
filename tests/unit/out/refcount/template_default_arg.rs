extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct A {
    #[offset(0)]
    pub v: i32,
}
impl A {
    pub fn new_1() -> Self {
        let __this: Value<A> = Rc::new(RefCell::new(Self { v: 1 }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<A> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<A> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new_1() }
    }
}
impl ByteRepr for A {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct B {
    #[offset(0)]
    pub v: i32,
}
impl B {
    pub fn new() -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self { v: 2 }));
        let this: Ptr<B> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
impl ByteRepr for B {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct NoDefault {
    #[offset(0)]
    pub v: i32,
}
impl NoDefault {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<NoDefault> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<NoDefault> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for NoDefault {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn used_0(x: Option<A>) -> i32 {
    let x: Value<A> = Rc::new(RefCell::new(x.unwrap_or(A::new_1())));
    return { (*x.borrow()).v };
}
pub fn used_1(x: Option<B>) -> i32 {
    let x: Value<B> = Rc::new(RefCell::new(x.unwrap_or(B::new())));
    return { (*x.borrow()).v };
}
pub fn scaled_2(x: A, n: Option<i32>) -> i32 {
    let x: Value<A> = Rc::new(RefCell::new(x));
    let n: Value<i32> = Rc::new(RefCell::new(n.unwrap_or((4usize as i32))));
    return ({ (*x.borrow()).v } * (*n.borrow()));
}
pub fn always_given_3(x: NoDefault) -> i32 {
    let x: Value<NoDefault> = Rc::new(RefCell::new(x));
    return { (*x.borrow()).v };
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S_NoDefault_ {
    #[offset(0)]
    pub v: i32,
}
impl S_NoDefault_ {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<S_NoDefault_> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<S_NoDefault_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S_NoDefault_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ used_0(None,) }) == 1));
    assert!((({ used_0(Some(A::new_2({ 5 },)),) }) == 5));
    assert!((({ used_1(None,) }) == 2));
    assert!((({ scaled_2(A::new_2({ 3 },), None,) }) == (3 * (4usize as i32))));
    assert!((({ scaled_2(A::new_2({ 3 },), Some(2),) }) == 6));
    assert!((({ always_given_3(NoDefault::new({ 3 },),) }) == 3));
    let s: Value<S_NoDefault_> = Rc::new(RefCell::new(S_NoDefault_::new({ 1 })));
    assert!((({ S_NoDefault_Impl::get(&s.as_pointer(), NoDefault::new({ 4 },),) }) == 5));
    return 0;
}
pub trait S_NoDefault_Impl {
    fn get(&self, t: NoDefault) -> i32;
}
impl S_NoDefault_Impl for Ptr<S_NoDefault_> {
    fn get(&self, t: NoDefault) -> i32 {
        let t: Value<NoDefault> = Rc::new(RefCell::new(t));
        return ((*self).with(|__s| __s.v) + { (*t.borrow()).v });
    }
}
pub fn __cpp2rust_init_globals() {}
