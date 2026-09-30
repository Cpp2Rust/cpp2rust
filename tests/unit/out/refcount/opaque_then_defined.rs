extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct list {
    #[offset(0)]
    pub head: Ptr<node>,
    #[offset(8)]
    pub size: i32,
}
impl ByteRepr for list {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.head.to_bytes(&mut buf[0..8]);
        self.size.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: <Ptr<node>>::from_bytes(&buf[0..8]),
            size: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    pub next: Ptr<node>,
}
impl ByteRepr for node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
            next: <Ptr<node>>::from_bytes(&buf[8..16]),
        }
    }
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
