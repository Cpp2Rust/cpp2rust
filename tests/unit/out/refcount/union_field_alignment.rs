extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub union node_anon_0 {
    pub bytes: Value<Box<[u8]>>,
    pub aligner: Value<AnyPtr>,
}
impl Default for node_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for node_anon_0 {}
#[derive(Default)]
pub struct node {
    pub next: Value<Ptr<node>>,
    pub x: Value<node_anon_0>,
}
impl ByteRepr for node {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = <Value<node>>::default();
    (*(*n.borrow()).next.borrow_mut()) = Default::default();
    (*(*(*n.borrow()).x.borrow()).bytes.borrow_mut())[(0) as usize] = 171_u8;
    assert!((((*(*(*n.borrow()).x.borrow()).bytes.borrow())[(0) as usize] as i32) == 171));
    return 0;
}
