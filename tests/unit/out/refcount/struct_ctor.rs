extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct StructWithCtor {
    #[offset(0)]
    x1_: i32,
    #[offset(4)]
    x2_: i32,
}
impl StructWithCtor {
    pub fn new(x1: i32, x2: i32) -> Self {
        let x1: Value<i32> = Rc::new(RefCell::new(x1));
        let x2: Value<i32> = Rc::new(RefCell::new(x2));
        let __this: Value<StructWithCtor> = Rc::new(RefCell::new(Self {
            x1_: (*x1.borrow()),
            x2_: (*x2.borrow()),
        }));
        let this: Ptr<StructWithCtor> = __this.as_pointer();
        field!(this, x1_).with_mut(|__v| __v.prefix_inc());
        field!(this, x2_).with_mut(|__v| __v.prefix_dec());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn foo_0(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Value_ {
    #[offset(0)]
    pub v: i32,
}
impl Value_ {
    pub fn new(u: i32) -> Self {
        let u: Value<i32> = Rc::new(RefCell::new(u));
        let __this: Value<Value_> = Rc::new(RefCell::new(Self { v: (*u.borrow()) }));
        let this: Ptr<Value_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Ptr_ {
    #[offset(0)]
    #[byte_size(4)]
    pub v1: Value_,
    #[offset(4)]
    #[byte_size(4)]
    pub v2: Value_,
}
impl Ptr_ {
    pub fn new() -> Self {
        let __this: Value<Ptr_> = Rc::new(RefCell::new(Self {
            v1: Value_::new({ 11 }),
            v2: Value_::new({ 22 }),
        }));
        let this: Ptr<Ptr_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        { Ptr_::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let struct_with_ctor: Value<StructWithCtor> =
        Rc::new(RefCell::new(StructWithCtor::new({ 1 }, { 2 })));
    let x: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (((({ foo_0(x.as_pointer(),) }).read()) == 3)
            && ((({ StructWithCtorImpl::x1(&struct_with_ctor.as_pointer(),) }).read()) == 2))
            && ((({ StructWithCtorImpl::x2(&struct_with_ctor.as_pointer(),) }).read()) == 1)
    );
    let p: Value<Ptr_> = Rc::new(RefCell::new(Ptr_::new()));
    assert!(({ (*p.borrow()).v1.v } == 11));
    assert!(({ (*p.borrow()).v2.v } == 22));
    return 0;
}
pub trait StructWithCtorImpl {
    fn x1(&self) -> Ptr<i32>;
    fn x2(&self) -> Ptr<i32>;
}
impl StructWithCtorImpl for Ptr<StructWithCtor> {
    fn x1(&self) -> Ptr<i32> {
        return field_ptr!((*self), x1_);
    }
    fn x2(&self) -> Ptr<i32> {
        return field_ptr!((*self), x2_);
    }
}
pub fn __cpp2rust_init_globals() {}
