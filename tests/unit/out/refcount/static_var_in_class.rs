extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    static inner_const_0: Value<i32> = Rc::new(RefCell::new(1));
);
#[derive(Default)]
pub struct C {}
pub trait CImpl {
    fn get(&self) -> i32;
}
impl Clone for C {
    fn clone(&self) -> Self {
        let __this: Value<C> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<C> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for C {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static inner_const_1: Value<i32> = Rc::new(RefCell::new(2));
);
#[derive(Default)]
pub struct S {}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<C> = Rc::new(RefCell::new(<C>::default()));
    assert!((({ CImpl::get(&c.as_pointer(),) }) == 1));
    assert!(((*inner_const_1.with(Value::clone).borrow()) == 2));
    return 0;
}
impl CImpl for Ptr<C> {
    fn get(&self) -> i32 {
        return (*inner_const_0.with(Value::clone).borrow());
    }
}
