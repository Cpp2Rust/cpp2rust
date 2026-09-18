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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(3));
    let by_value: Value<lambda_0> = Rc::new(RefCell::new(
        (lambda_0 {
            a: Rc::new(RefCell::new((*a.borrow()))),
            b: Rc::new(RefCell::new((*b.borrow()))),
            c: Rc::new(RefCell::new((*c.borrow()))),
        }),
    ));
    assert!((({ lambda_0::operator_call(&(*by_value.borrow_mut()), 10,) }) == 16));
    (*a.borrow_mut()) = 100;
    assert!((({ lambda_0::operator_call(&(*by_value.borrow_mut()), 10,) }) == 16));
    let by_ref: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            a: a.as_pointer(),
            b: b.as_pointer(),
            c: c.as_pointer(),
        }),
    ));
    assert!((({ lambda_1::operator_call(&(*by_ref.borrow_mut()), 10,) }) == 115));
    (*b.borrow_mut()) = 200;
    assert!((({ lambda_1::operator_call(&(*by_ref.borrow_mut()), 10,) }) == 313));
    let mixed: Value<lambda_2> = Rc::new(RefCell::new(
        (lambda_2 {
            c: c.as_pointer(),
            a: Rc::new(RefCell::new((*a.borrow()))),
            b: Rc::new(RefCell::new((*b.borrow()))),
        }),
    ));
    assert!((({ lambda_2::operator_call(&(*mixed.borrow_mut()), 1,) }) == ((100 + 200) + 4)));
    assert!(((*c.borrow()) == 4));
    return 0;
}
#[derive(Default)]
pub struct lambda_0 {
    a: Value<i32>,
    b: Value<i32>,
    c: Value<i32>,
}
impl lambda_0 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((((*self.a.borrow()) + (*self.b.borrow())) + (*self.c.borrow())) + (*x.borrow()));
    }
}
impl Clone for lambda_0 {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
            c: Rc::new(RefCell::new((*self.c.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_0 {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
        (*self.c.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            c: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call(self, a1) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_1 {
    a: Ptr<i32>,
    b: Ptr<i32>,
    c: Ptr<i32>,
}
impl lambda_1 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((((self.a.read()) + (self.b.read())) + (self.c.read())) + (*x.borrow()));
    }
}
impl ByteRepr for lambda_1 {}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call(self, a1) }
    }
}
#[derive(Default)]
pub struct lambda_2 {
    c: Ptr<i32>,
    a: Value<i32>,
    b: Value<i32>,
}
impl lambda_2 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        {
            let _ptr = self.c.clone();
            _ptr.write(_ptr.read() + (*x.borrow()))
        };
        return (((*self.a.borrow()) + (*self.b.borrow())) + (self.c.read()));
    }
}
impl Clone for lambda_2 {
    fn clone(&self) -> Self {
        Self {
            c: self.c.clone(),
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_2 {}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_2::operator_call(self, a1) }
    }
}
pub fn __cpp2rust_init_globals() {}
