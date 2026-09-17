extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Counter {
    pub n: Value<i32>,
}
impl Clone for Counter {
    fn clone(&self) -> Self {
        let __this: Value<Counter> = Rc::new(RefCell::new(Self {
            n: Rc::new(RefCell::new((*self.n.borrow()))),
        }));
        let this: Ptr<Counter> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Counter {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct lambda_0 {
    this_: Value<Ptr<Counter>>,
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
            this_: Rc::new(RefCell::new(<Ptr<Counter>>::from_bytes(&buf[0..8]))),
        }
    }
}
impl Callable1<i32, ()> for lambda_0 {
    fn call(&self, a1: i32) -> () {
        { lambda_0::operator_call(self, a1) }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Counter> = Rc::new(RefCell::new(<Counter>::default()));
    ({ CounterImpl::bump(&c.as_pointer(), 3) });
    assert!(((*(*c.borrow()).n.borrow()) == 6));
    return 0;
}
pub trait CounterImpl {
    fn bump(&self, by: i32);
}
impl CounterImpl for Ptr<Counter> {
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
}
