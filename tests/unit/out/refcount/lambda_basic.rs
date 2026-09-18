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
    let zero: Value<lambda_0> = Rc::new(RefCell::new((lambda_0 {})));
    assert!((({ lambda_0::operator_call() }) == 42));
    let one: Value<lambda_1> = Rc::new(RefCell::new((lambda_1 {})));
    assert!((({ lambda_1::operator_call(1,) }) == 2));
    let three: Value<lambda_2> = Rc::new(RefCell::new((lambda_2 {})));
    assert!((({ lambda_2::operator_call(1, 2, 3,) }) == 123));
    let hits: Value<i32> = Rc::new(RefCell::new(0));
    let no_return: Value<lambda_3> = Rc::new(RefCell::new(
        (lambda_3 {
            hits: hits.as_pointer(),
        }),
    ));
    ({ lambda_3::operator_call(&(*no_return.borrow_mut()), 3) });
    ({ lambda_3::operator_call(&(*no_return.borrow_mut()), 4) });
    assert!(((*hits.borrow()) == 7));
    let a: Value<i32> = Rc::new(RefCell::new(2));
    let b: Value<i32> = Rc::new(RefCell::new(3));
    let product: Value<i32> = Rc::new(RefCell::new(
        ({
            lambda_4::operator_call(
                &(lambda_4 {
                    a: a.as_pointer(),
                    b: b.as_pointer(),
                }),
            )
        }),
    ));
    assert!(((*product.borrow()) == 6));
    return 0;
}
#[derive(Clone, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub fn operator_call() -> i32 {
        return 42;
    }
}
impl ByteRepr for lambda_0 {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
impl Callable0<i32> for lambda_0 {
    fn call(&self) -> i32 {
        { lambda_0::operator_call() }
    }
}
impl lambda_0 {
    pub fn to_free_function(&self) -> FnPtr<fn() -> i32> {
        FnPtr::new(lambda_0::operator_call)
    }
}
#[derive(Clone, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub fn operator_call(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + 1);
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
    pub fn operator_call(x: i32, y: i32, z: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let z: Value<i32> = Rc::new(RefCell::new(z));
        return ((((*x.borrow()) * 100) + ((*y.borrow()) * 10)) + (*z.borrow()));
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
impl Callable3<i32, i32, i32, i32> for lambda_2 {
    fn call(&self, a1: i32, a2: i32, a3: i32) -> i32 {
        { lambda_2::operator_call(a1, a2, a3) }
    }
}
impl lambda_2 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32, i32, i32) -> i32> {
        FnPtr::new(lambda_2::operator_call)
    }
}
#[derive(Clone, Default)]
pub struct lambda_3 {
    hits: Ptr<i32>,
}
impl lambda_3 {
    pub fn operator_call(&self, by: i32) {
        let by: Value<i32> = Rc::new(RefCell::new(by));
        {
            let _ptr = self.hits.clone();
            _ptr.write(_ptr.read() + (*by.borrow()))
        };
    }
}
impl ByteRepr for lambda_3 {}
impl Callable1<i32, ()> for lambda_3 {
    fn call(&self, a1: i32) -> () {
        { lambda_3::operator_call(self, a1) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_4 {
    a: Ptr<i32>,
    b: Ptr<i32>,
}
impl lambda_4 {
    pub fn operator_call(&self) -> i32 {
        return ((self.a.read()) * (self.b.read()));
    }
}
impl ByteRepr for lambda_4 {}
impl Callable0<i32> for lambda_4 {
    fn call(&self) -> i32 {
        { lambda_4::operator_call(self) }
    }
}
pub fn __cpp2rust_init_globals() {}
