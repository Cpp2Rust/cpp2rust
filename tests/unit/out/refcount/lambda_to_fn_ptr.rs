extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(x: i32, fn_: FnPtr<fn(i32) -> i32>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return ({ (*(*fn_.borrow()))((*x.borrow())) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fresh: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(({ (lambda_1 {}).to_free_function() })));
    assert!((({ (*(*fresh.borrow()))(5,) }) == -5_i32));
    let twice: Value<lambda_2> = Rc::new(RefCell::new((lambda_2 {})));
    let named: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(({ (*twice.borrow()).to_free_function() })));
    assert!((({ (*(*named.borrow()))(5,) }) == 10));
    assert!((({ apply_0(5, ({ (*twice.borrow()).to_free_function() }),) }) == 10));
    (*named.borrow_mut()) = (*fresh.borrow()).clone();
    assert!((({ (*(*named.borrow()))(3,) }) == -3_i32));
    return 0;
}
#[derive(Clone, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub fn operator_call(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return -(*x.borrow());
    }
}
impl ByteRepr for lambda_1 {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call(a1) }
    }
}
impl lambda_1 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_1::operator_call)
    }
}
#[derive(Clone, Default)]
pub struct lambda_2 {}
impl lambda_2 {
    pub fn operator_call(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * 2);
    }
}
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
        { lambda_2::operator_call(a1) }
    }
}
impl lambda_2 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_2::operator_call)
    }
}
