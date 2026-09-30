extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct list {
    #[offset(0)]
    #[byte_size(8)]
    pub head: Ptr<node>,
    #[offset(8)]
    pub size: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = Rc::new(RefCell::new(node {
        value: 42,
        next: Ptr::<node>::null(),
    }));
    let l: Value<list> = Rc::new(RefCell::new(list {
        head: (n.as_pointer()),
        size: 1,
    }));
    assert!(((({ (*l.borrow()).head.clone() }.with(|__s| __s.value) == 42) as i32) != 0));
    assert!(((({ (*l.borrow()).size } == 1) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
