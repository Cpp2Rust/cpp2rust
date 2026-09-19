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
#[derive(Clone, Default)]
pub struct C {}
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
#[derive(Clone, Default)]
pub struct S {}
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<C> = Rc::new(RefCell::new(<C>::default()));
    assert!((({ CImpl::get(&c.as_pointer(),) }) == 1));
    assert!((inner_const_1.with(|rc| rc.borrow().clone()) == 2));
    return 0;
}
pub trait CImpl {
    fn get(&self) -> i32;
}
impl CImpl for Ptr<C> {
    fn get(&self) -> i32 {
        return inner_const_0.with(|rc| rc.borrow().clone());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = inner_const_0.with(|_| ());
    let _ = inner_const_1.with(|_| ());
}
