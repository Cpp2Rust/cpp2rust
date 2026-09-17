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
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<lambda_0> = Rc::new(RefCell::new(
        (lambda_0 {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
        }),
    ));
    assert!((({ lambda_0::operator_call(&(*scale.borrow_mut()), 4,) }) == 12));
    (*factor.borrow_mut()) = 100;
    assert!((({ lambda_0::operator_call(&(*scale.borrow_mut()), 4,) }) == 12));
    let slot: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((slot.as_pointer())));
    let read_ptr: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            p: Rc::new(RefCell::new((*p.borrow()).clone())),
        }),
    ));
    (*slot.borrow_mut()) = 8;
    assert!((({ lambda_1::operator_call(&(*read_ptr.borrow_mut()),) }) == 8));
    let s: Value<S> = Rc::new(RefCell::new(S {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    }));
    let sum: Value<lambda_2> = Rc::new(RefCell::new(
        (lambda_2 {
            s: Rc::new(RefCell::new((*s.borrow()).clone())),
        }),
    ));
    (*(*s.borrow()).x.borrow_mut()) = 50;
    assert!((({ lambda_2::operator_call(&(*sum.borrow_mut()),) }) == 3));
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let shifted: Value<lambda_3> = Rc::new(RefCell::new(
        (lambda_3 {
            y: Rc::new(RefCell::new(((*base.borrow()) + 1))),
        }),
    ));
    assert!((({ lambda_3::operator_call(&(*shifted.borrow_mut()), 5,) }) == 16));
    return 0;
}
#[derive(Default)]
pub struct lambda_0 {
    factor: Value<i32>,
}
impl lambda_0 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * (*self.factor.borrow()));
    }
}
impl Clone for lambda_0 {
    fn clone(&self) -> Self {
        Self {
            factor: Rc::new(RefCell::new((*self.factor.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_0 {
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
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call(self, a1) }
    }
}
#[derive(Default)]
pub struct lambda_1 {
    p: Value<Ptr<i32>>,
}
impl lambda_1 {
    pub fn operator_call(&self) -> i32 {
        return ((*self.p.borrow()).read());
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.p.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
        }
    }
}
impl Callable0<i32> for lambda_1 {
    fn call(&self) -> i32 {
        { lambda_1::operator_call(self) }
    }
}
#[derive(Default)]
pub struct lambda_2 {
    s: Value<S>,
}
impl lambda_2 {
    pub fn operator_call(&self) -> i32 {
        return ((*(*self.s.borrow()).x.borrow()) + (*(*self.s.borrow()).y.borrow()));
    }
}
impl Clone for lambda_2 {
    fn clone(&self) -> Self {
        Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_2 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..8]))),
        }
    }
}
impl Callable0<i32> for lambda_2 {
    fn call(&self) -> i32 {
        { lambda_2::operator_call(self) }
    }
}
#[derive(Default)]
pub struct lambda_3 {
    y: Value<i32>,
}
impl lambda_3 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (*self.y.borrow()));
    }
}
impl Clone for lambda_3 {
    fn clone(&self) -> Self {
        Self {
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_3 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.y.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_3::operator_call(self, a1) }
    }
}
