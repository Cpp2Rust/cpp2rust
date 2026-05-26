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
    pub static s_s: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::<S>::null()));
);
thread_local!(
    pub static s_file: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
);
thread_local!(
    pub static s_size: Value<u64> = <Value<u64>>::default();
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*s_s.with(Value::clone).borrow()).is_null());
    assert!((*s_file.with(Value::clone).borrow()).is_null());
    assert!(((*s_size.with(Value::clone).borrow()) == 0_u64));
    return 0;
}
