extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(fn_: lambda_1, x: i32) -> i32 {
    let fn_: Value<lambda_1> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_1Impl::operator_call(&fn_.as_pointer(), (*x.borrow())) });
}
pub fn apply_2(fn_: lambda_3, x: i32) -> i32 {
    let fn_: Value<lambda_3> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_3Impl::operator_call(&fn_.as_pointer(), (*x.borrow())) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            base: base.as_pointer(),
        }),
    ));
    assert!((({ apply_0((*add_base.borrow()).clone(), 5,) }) == 15));
    (*base.borrow_mut()) = 100;
    assert!((({ apply_0((*add_base.borrow()).clone(), 5,) }) == 105));
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<lambda_3> = Rc::new(RefCell::new(
        (lambda_3 {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
        }),
    ));
    assert!((({ apply_2((*scale.borrow()).clone(), 4,) }) == 12));
    return 0;
}
#[derive(Clone, Default)]
pub struct lambda_1 {
    base: Ptr<i32>,
}
impl ByteRepr for lambda_1 {}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        let __this: Value<lambda_1> = Rc::new(RefCell::new(self.clone()));
        lambda_1Impl::operator_call(&__this.as_pointer(), a1)
    }
}
#[derive(Default)]
pub struct lambda_3 {
    factor: Value<i32>,
}
impl Clone for lambda_3 {
    fn clone(&self) -> Self {
        Self {
            factor: Rc::new(RefCell::new((*self.factor.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_3 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.factor.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            factor: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        let __this: Value<lambda_3> = Rc::new(RefCell::new(self.clone()));
        lambda_3Impl::operator_call(&__this.as_pointer(), a1)
    }
}
pub trait lambda_1Impl {
    fn operator_call(&self, x: i32) -> i32;
}
impl lambda_1Impl for Ptr<lambda_1> {
    fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + ((*(*self).upgrade().deref()).base.read()));
    }
}
pub trait lambda_3Impl {
    fn operator_call(&self, x: i32) -> i32;
}
impl lambda_3Impl for Ptr<lambda_3> {
    fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * (*(*(*self).upgrade().deref()).factor.borrow()));
    }
}
