extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub v: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct lambda_1 {
    this_: Value<Ptr<S>>,
    y: Value<i32>,
}
impl lambda_1 {
    fn operator_call(&self, z: i32) -> i32 {
        let z: Value<i32> = Rc::new(RefCell::new(z));
        return (((*(*(*self.this_.borrow()).upgrade().deref()).v.borrow()) + (*self.y.borrow()))
            + (*z.borrow()));
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            this_: Rc::new(RefCell::new((*self.this_.borrow()).clone())),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.this_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.y.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            this_: Rc::new(RefCell::new(<Ptr<S>>::from_bytes(&buf[0..8]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call(self, a1) }
    }
}
#[derive(Default)]
pub struct lambda_0 {
    this_: Value<Ptr<S>>,
}
impl lambda_0 {
    fn operator_call(&self, y: i32) -> i32 {
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let inner: Value<lambda_1> = Rc::new(RefCell::new(
            (lambda_1 {
                this_: Rc::new(RefCell::new((*self.this_.borrow()).clone())),
                y: Rc::new(RefCell::new((*y.borrow()))),
            }),
        ));
        return ({ lambda_1::operator_call(&(*inner.borrow_mut()), 1) });
    }
}
impl Clone for lambda_0 {
    fn clone(&self) -> Self {
        Self {
            this_: Rc::new(RefCell::new((*self.this_.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.this_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            this_: Rc::new(RefCell::new(<Ptr<S>>::from_bytes(&buf[0..8]))),
        }
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call(self, a1) }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let outer: Value<lambda_2> = Rc::new(RefCell::new((lambda_2 { x: x.as_pointer() })));
    assert!((({ lambda_2::operator_call(&(*outer.borrow_mut()), 20,) }) == 31));
    (*x.borrow_mut()) = 100;
    assert!((({ lambda_2::operator_call(&(*outer.borrow_mut()), 20,) }) == 121));
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(5)),
    }));
    assert!((({ SImpl::nested_this(&s.as_pointer(),) }) == 26));
    return 0;
}
#[derive(Default)]
pub struct lambda_3 {
    x: Ptr<i32>,
    y: Value<i32>,
}
impl lambda_3 {
    pub fn operator_call(&self, z: i32) -> i32 {
        let z: Value<i32> = Rc::new(RefCell::new(z));
        return (((self.x.read()) + (*self.y.borrow())) + (*z.borrow()));
    }
}
impl Clone for lambda_3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_3 {}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_3::operator_call(self, a1) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_2 {
    x: Ptr<i32>,
}
impl lambda_2 {
    pub fn operator_call(&self, y: i32) -> i32 {
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let inner: Value<lambda_3> = Rc::new(RefCell::new(
            (lambda_3 {
                x: (self.x).clone(),
                y: Rc::new(RefCell::new((*y.borrow()))),
            }),
        ));
        return ({ lambda_3::operator_call(&(*inner.borrow_mut()), 1) });
    }
}
impl ByteRepr for lambda_2 {}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_2::operator_call(self, a1) }
    }
}
pub trait SImpl {
    fn nested_this(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn nested_this(&self) -> i32 {
        let outer: Value<lambda_0> = Rc::new(RefCell::new(
            (lambda_0 {
                this_: Rc::new(RefCell::new((*self).clone())),
            }),
        ));
        return ({ lambda_0::operator_call(&(*outer.borrow_mut()), 20) });
    }
}
pub fn __cpp2rust_init_globals() {}
