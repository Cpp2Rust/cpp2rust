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
    let start: Value<i32> = Rc::new(RefCell::new(5));
    let next: Value<lambda_0> = Rc::new(RefCell::new(
        (lambda_0 {
            start: Rc::new(RefCell::new((*start.borrow()))),
        }),
    ));
    assert!((({ lambda_0::operator_call(&mut (*next.borrow_mut()),) }) == 5));
    assert!((({ lambda_0::operator_call(&mut (*next.borrow_mut()),) }) == 6));
    assert!((({ lambda_0::operator_call(&mut (*next.borrow_mut()),) }) == 7));
    assert!(((*start.borrow()) == 5));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let accumulate: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            total: Rc::new(RefCell::new((*total.borrow()))),
        }),
    ));
    assert!((({ lambda_1::operator_call(&mut (*accumulate.borrow_mut()), 1,) }) == 1));
    assert!((({ lambda_1::operator_call(&mut (*accumulate.borrow_mut()), 2,) }) == 3));
    assert!(((*total.borrow()) == 0));
    return 0;
}
#[derive(Default)]
pub struct lambda_0 {
    start: Value<i32>,
}
impl lambda_0 {
    pub fn operator_call(&self) -> i32 {
        return (*self.start.borrow_mut()).postfix_inc();
    }
}
impl Clone for lambda_0 {
    fn clone(&self) -> Self {
        Self {
            start: Rc::new(RefCell::new((*self.start.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_0 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct lambda_1 {
    total: Value<i32>,
}
impl lambda_1 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        (*self.total.borrow_mut()) += (*x.borrow());
        return (*self.total.borrow());
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            total: Rc::new(RefCell::new((*self.total.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.total.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            total: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn __cpp2rust_init_globals() {}
