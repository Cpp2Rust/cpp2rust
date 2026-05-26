extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    static s_C_inner_const: Value<i32> = Rc::new(RefCell::new(1));
);
#[derive(Default)]
pub struct C {}
impl C {
    pub fn get(&self) -> i32 {
        return (*s_C_inner_const.with(Value::clone).borrow());
    }
}
impl Clone for C {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for C {
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static s_S_inner_const: Value<i32> = Rc::new(RefCell::new(2));
);
#[derive(Default)]
pub struct S {}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for S {
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
    assert!((({ (*c.borrow()).get() }) == 1));
    assert!(((*s_S_inner_const.with(Value::clone).borrow()) == 2));
    return 0;
}
