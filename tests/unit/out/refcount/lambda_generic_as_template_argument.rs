extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_int_0(fn_: lambda_1, x: i32) -> i32 {
    let fn_: Value<lambda_1> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_1::operator_call_i32__int_const(&(*fn_.borrow_mut()), (*x.borrow())) });
}
pub fn apply_int_2(fn_: lambda_3, x: i32) -> i32 {
    let fn_: Value<lambda_3> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_3::operator_call_i32__int_const((*x.borrow())) });
}
pub fn apply_int_4(fn_: lambda_5, x: i32) -> i32 {
    let fn_: Value<lambda_5> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_5::operator_call_i32__int_const(&(*fn_.borrow_mut()), (*x.borrow())) });
}
pub fn apply_double_6(fn_: lambda_1, x: f64) -> f64 {
    let fn_: Value<lambda_1> = Rc::new(RefCell::new(fn_));
    let x: Value<f64> = Rc::new(RefCell::new(x));
    return ({ lambda_1::operator_call_f64__double_const(&(*fn_.borrow_mut()), (*x.borrow())) });
}
pub fn apply_double_7(fn_: lambda_5, x: f64) -> f64 {
    let fn_: Value<lambda_5> = Rc::new(RefCell::new(fn_));
    let x: Value<f64> = Rc::new(RefCell::new(x));
    return ({ lambda_5::operator_call_f64__double_const(&(*fn_.borrow_mut()), (*x.borrow())) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
        }),
    ));
    assert!((({ apply_int_0((*scale.borrow()).clone(), 4,) }) == 12));
    assert!((({ apply_double_6((*scale.borrow()).clone(), 1.5E+0,) }) == 4.5E+0));
    assert!((({ apply_int_2((lambda_3 {}), 9,) }) == -9_i32));
    let offset: Value<lambda_5> = Rc::new(RefCell::new(
        (lambda_5 {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
        }),
    ));
    assert!((({ apply_int_4((*offset.borrow()).clone(), 4,) }) == 7));
    assert!((({ apply_double_7((*offset.borrow()).clone(), 1.5E+0,) }) == 4.5E+0));
    return 0;
}
#[derive(Default)]
pub struct lambda_1 {
    factor: Value<i32>,
}
impl lambda_1 {
    pub fn operator_call_i32__int_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * (*self.factor.borrow()));
    }
    pub fn operator_call_f64__double_const(&self, x: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * ((*self.factor.borrow()) as f64));
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            factor: Rc::new(RefCell::new((*self.factor.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {
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
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call_i32__int_const(self, a1) }
    }
}
impl Callable1<f64, f64> for lambda_1 {
    fn call(&self, a1: f64) -> f64 {
        { lambda_1::operator_call_f64__double_const(self, a1) }
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_3 {}
impl lambda_3 {
    pub fn operator_call_i32__int_const(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return -(*x.borrow());
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_3::operator_call_i32__int_const(a1) }
    }
}
#[derive(Default)]
pub struct lambda_5 {
    factor: Value<i32>,
}
impl lambda_5 {
    pub fn operator_call_i32__int_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (*self.factor.borrow()));
    }
    pub fn operator_call_f64__double_const(&self, x: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + ((*self.factor.borrow()) as f64));
    }
}
impl Clone for lambda_5 {
    fn clone(&self) -> Self {
        Self {
            factor: Rc::new(RefCell::new((*self.factor.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_5 {
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
impl Callable1<i32, i32> for lambda_5 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_5::operator_call_i32__int_const(self, a1) }
    }
}
impl Callable1<f64, f64> for lambda_5 {
    fn call(&self, a1: f64) -> f64 {
        { lambda_5::operator_call_f64__double_const(self, a1) }
    }
}
pub fn __cpp2rust_init_globals() {}
