extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let outer: Value<lambda_0> = Rc::new(RefCell::new((lambda_0 { x: x.as_pointer() })));
    assert!((({ lambda_0::operator_call(&(*outer.borrow_mut()), 20,) }) == 31));
    (*x.borrow_mut()) = 100;
    assert!((({ lambda_0::operator_call(&(*outer.borrow_mut()), 20,) }) == 121));
    return 0;
}
#[derive(Default)]
pub struct lambda_1 {
    x: Ptr<i32>,
    y: Value<i32>,
}
impl lambda_1 {
    pub fn operator_call(&self, z: i32) -> i32 {
        let z: Value<i32> = Rc::new(RefCell::new(z));
        return (((self.x.read()) + (*self.y.borrow())) + (*z.borrow()));
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call(self, a1) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_0 {
    x: Ptr<i32>,
}
impl lambda_0 {
    pub fn operator_call(&self, y: i32) -> i32 {
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let inner: Value<lambda_1> = Rc::new(RefCell::new(
            (lambda_1 {
                x: (self.x).clone(),
                y: Rc::new(RefCell::new((*y.borrow()))),
            }),
        ));
        return ({ lambda_1::operator_call(&(*inner.borrow_mut()), 1) });
    }
}
impl ByteRepr for lambda_0 {}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call(self, a1) }
    }
}
