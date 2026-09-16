extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn apply_1(x: i32, fn_: Option<FnPtr<fn(i32) -> i32>>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let fn_: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(fn_.unwrap_or(FnPtr::<fn(i32) -> i32>::null())));
    if !(*fn_.borrow()).is_null() {
        return ({ (*(*fn_.borrow()))((*x.borrow())) });
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ apply_1(5, None,) }) == 5));
    assert!((({ apply_1(5, Some(FnPtr::<fn(i32) -> i32>::null()),) }) == 5));
    assert!((({ apply_1(5, Some(FnPtr::<fn(i32) -> i32>::new(identity_0)),) }) == 5));
    let negate: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::new(|a1: i32| {
        let __this: Value<lambda_2> = Rc::new(RefCell::new(lambda_2 {}));
        lambda_2Impl::operator_call(&__this.as_pointer(), a1)
    })));
    assert!((({ apply_1(5, Some((*negate.borrow()).clone()),) }) == -5_i32));
    return 0;
}
#[derive(Clone, Default)]
pub struct lambda_2 {}
impl ByteRepr for lambda_2 {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        let __this: Value<lambda_2> = Rc::new(RefCell::new(self.clone()));
        lambda_2Impl::operator_call(&__this.as_pointer(), a1)
    }
}
pub trait lambda_2Impl {
    fn operator_call(&self, x: i32) -> i32;
}
impl lambda_2Impl for Ptr<lambda_2> {
    fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return -(*x.borrow());
    }
}
