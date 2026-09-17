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
    pub n: Value<i32>,
    pub step: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            n: Rc::new(RefCell::new((*self.n.borrow()))),
            step: Rc::new(RefCell::new((*self.step.borrow()))),
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
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
        (*self.step.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            step: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct lambda_0 {
    this_: Value<Ptr<S>>,
}
impl lambda_0 {
    fn operator_call(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        (*(*(*self.this_.borrow()).upgrade().deref()).n.borrow_mut()) += (*k.borrow());
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
impl Callable1<i32, ()> for lambda_0 {
    fn call(&self, a1: i32) -> () {
        { lambda_0::operator_call(self, a1) }
    }
}
#[derive(Default)]
pub struct lambda_1 {
    this_: Value<Ptr<S>>,
}
impl lambda_1 {
    fn operator_call(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        ({ SImpl::add(&(*self.this_.borrow()), (*k.borrow())) });
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            this_: Rc::new(RefCell::new((*self.this_.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {
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
impl Callable1<i32, ()> for lambda_1 {
    fn call(&self, a1: i32) -> () {
        { lambda_1::operator_call(self, a1) }
    }
}
#[derive(Default)]
pub struct lambda_2 {
    this_: Value<Ptr<S>>,
}
impl lambda_2 {
    fn operator_call(&self) -> i32 {
        return ({ SImpl::scaled(&(*self.this_.borrow())) });
    }
}
impl Clone for lambda_2 {
    fn clone(&self) -> Self {
        Self {
            this_: Rc::new(RefCell::new((*self.this_.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_2 {
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
impl Callable0<i32> for lambda_2 {
    fn call(&self) -> i32 {
        { lambda_2::operator_call(self) }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        n: Rc::new(RefCell::new(0)),
        step: Rc::new(RefCell::new(2)),
    }));
    ({ SImpl::bump(&s.as_pointer(), 3) });
    assert!(((*(*s.borrow()).n.borrow()) == 6));
    ({ SImpl::bump_via_method(&s.as_pointer(), 4) });
    assert!(((*(*s.borrow()).n.borrow()) == 10));
    assert!((({ SImpl::read_scaled(&s.as_pointer(),) }) == 20));
    return 0;
}
pub trait SImpl {
    fn add(&self, k: i32);
    fn scaled(&self) -> i32;
    fn bump(&self, by: i32);
    fn bump_via_method(&self, by: i32);
    fn read_scaled(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn add(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        (*(*(*self).upgrade().deref()).n.borrow_mut()) += (*k.borrow());
    }
    fn scaled(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).n.borrow())
            * (*(*(*self).upgrade().deref()).step.borrow()));
    }
    fn bump(&self, by: i32) {
        let by: Value<i32> = Rc::new(RefCell::new(by));
        let inc: Value<lambda_0> = Rc::new(RefCell::new(
            (lambda_0 {
                this_: Rc::new(RefCell::new((*self).clone())),
            }),
        ));
        ({ lambda_0::operator_call(&(*inc.borrow_mut()), (*by.borrow())) });
        ({ lambda_0::operator_call(&(*inc.borrow_mut()), (*by.borrow())) });
    }
    fn bump_via_method(&self, by: i32) {
        let by: Value<i32> = Rc::new(RefCell::new(by));
        let inc: Value<lambda_1> = Rc::new(RefCell::new(
            (lambda_1 {
                this_: Rc::new(RefCell::new((*self).clone())),
            }),
        ));
        ({ lambda_1::operator_call(&(*inc.borrow_mut()), (*by.borrow())) });
    }
    fn read_scaled(&self) -> i32 {
        let get: Value<lambda_2> = Rc::new(RefCell::new(
            (lambda_2 {
                this_: Rc::new(RefCell::new((*self).clone())),
            }),
        ));
        return ({ lambda_2::operator_call(&(*get.borrow_mut())) });
    }
}
