extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub v: i32,
}
impl A {
    pub fn new_1() -> Self {
        Self { v: 1 }
    }
    pub fn new_2(mut v: i32) -> Self {
        Self { v: v }
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub v: i32,
}
impl B {
    pub fn new() -> Self {
        Self { v: 2 }
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoDefault {
    #[offset(0)]
    pub v: i32,
}
impl NoDefault {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
}
pub fn used_0(x: Option<A>) -> i32 {
    let mut x: A = x.unwrap_or_else(|| A::new_1());
    return x.v;
}
pub fn used_1(x: Option<B>) -> i32 {
    let mut x: B = x.unwrap_or_else(|| B::new());
    return x.v;
}
pub fn scaled_2(mut x: A, n: Option<i32>) -> i32 {
    let mut n: i32 = n.unwrap_or_else(|| (4usize as i32));
    return (x.v * n);
}
pub fn always_given_3(mut x: NoDefault) -> i32 {
    return x.v;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_NoDefault_ {
    #[offset(0)]
    pub v: i32,
}
impl S_NoDefault_ {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
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
    fn get(&self, mut t: NoDefault) -> i32 {
        return ((*self).with(|__s| __s.v) + t.v);
    }
}
pub fn __cpp2rust_init_globals() {}
