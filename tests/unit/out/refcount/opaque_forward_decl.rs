extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct container {
    pub p: Value<Ptr<opaque>>,
    pub x: Value<i32>,
}
impl ByteRepr for container {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<container> = Rc::new(RefCell::new(container {
        p: Rc::new(RefCell::new(Ptr::<opaque>::null())),
        x: Rc::new(RefCell::new(42)),
    }));
    (*(*c.borrow()).p.borrow()).clone();
    return ((*(*c.borrow()).x.borrow()) - 42);
}
#[repr(C)]
pub struct opaque {
    _opaque: [u8; 0],
}
