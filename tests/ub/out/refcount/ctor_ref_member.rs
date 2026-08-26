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
    pub r: Ptr<i32>,
}
impl S {
    pub fn S(x: Ptr<i32>) -> Self {
        let mut this = Self { r: (x).clone() };
        this
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            r: (self.r).clone(),
        };
        this
    }
}
impl ByteRepr for S {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::S({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
        __tmp_0.as_pointer()
    })));
    assert!((((*s.borrow()).r.read()) == 5));
    return 0;
}
