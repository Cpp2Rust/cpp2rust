extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let negate: Value<lambda_0> = Rc::new(RefCell::new((<lambda_0>::default())));
    let fi: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({ (*negate.borrow()).to_free_function_int_const() }),
    ));
    let fd: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(
        ({ (*negate.borrow()).to_free_function_double_const() }),
    ));
    assert!((({ (*fi.borrow()).call(3,) }) == -3_i32));
    assert!((({ (*fd.borrow()).call(1.5E+0,) }) == -1.5E+0));
    let square: Value<lambda_1> = Rc::new(RefCell::new((<lambda_1>::default())));
    let si: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({ (*square.borrow()).to_free_function_int_const() }),
    ));
    let sd: Value<FnPtr<fn(f64) -> f64>> = Rc::new(RefCell::new(
        ({ (*square.borrow()).to_free_function_double_const() }),
    ));
    assert!((({ (*si.borrow()).call(3,) }) == 9));
    assert!((({ (*sd.borrow()).call(1.5E+0,) }) == 2.25E+0));
    return 0;
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub fn operator_call_i32__int_const(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return -(*x.borrow());
    }
    pub fn operator_call_f64__double_const(x: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        return -(*x.borrow());
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call_i32__int_const(a1) }
    }
}
impl Callable1<f64, f64> for lambda_0 {
    fn call(&self, a1: f64) -> f64 {
        { lambda_0::operator_call_f64__double_const(a1) }
    }
}
impl lambda_0 {
    pub fn to_free_function_int_const(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_0::operator_call_i32__int_const)
    }
}
impl lambda_0 {
    pub fn to_free_function_double_const(&self) -> FnPtr<fn(f64) -> f64> {
        FnPtr::new(lambda_0::operator_call_f64__double_const)
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub fn operator_call_i32__int_const(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * (*x.borrow()));
    }
    pub fn operator_call_f64__double_const(x: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * (*x.borrow()));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call_i32__int_const(a1) }
    }
}
impl Callable1<f64, f64> for lambda_1 {
    fn call(&self, a1: f64) -> f64 {
        { lambda_1::operator_call_f64__double_const(a1) }
    }
}
impl lambda_1 {
    pub fn to_free_function_int_const(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_1::operator_call_i32__int_const)
    }
}
impl lambda_1 {
    pub fn to_free_function_double_const(&self) -> FnPtr<fn(f64) -> f64> {
        FnPtr::new(lambda_1::operator_call_f64__double_const)
    }
}
pub fn __cpp2rust_init_globals() {}
