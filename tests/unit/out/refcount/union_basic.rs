extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub union basic {
    pub i: Value<i32>,
    pub f: Value<f32>,
}
impl Default for basic {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for basic {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let u: Value<basic> = <Value<basic>>::default();
    (*(*u.borrow()).i.borrow_mut()) = 42;
    assert!(((*(*u.borrow()).i.borrow()) == 42));
    (*(*u.borrow()).f.borrow_mut()) = 3.140000105E+0;
    assert!(((*(*u.borrow()).f.borrow()) == 3.140000105E+0));
    return 0;
}
