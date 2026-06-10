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
    pub a: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
        };
        this
    }
}
impl ByteRepr for S {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
thread_local!(
    pub static s_0: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::<S>::null()));
);
thread_local!(
    pub static file_1: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
);
thread_local!(
    pub static size_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*s_0.with(Value::clone).borrow()).is_null());
    assert!((*file_1.with(Value::clone).borrow()).is_null());
    assert!(((*size_2.with(Value::clone).borrow()) == 0_usize));
    return 0;
}
