extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct node_a {
    #[offset(0)]
    pub n: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node_b {
    #[offset(0)]
    #[byte_size(8)]
    pub data: AnyPtr,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node_b>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<node_a> = Rc::new(RefCell::new(node_a { n: 123 }));
    let ptr: Value<anon_0> = <Value<anon_0>>::default();
    anon_0::to_a(ptr.as_pointer()).write((a.as_pointer()));
    let mut out: Ptr<node_b> = (anon_0::to_b(ptr.as_pointer()).read());
    assert!(((({ (out).to_any() } == { (a.as_pointer()).to_any() }) as i32) != 0));
    return 0;
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn to_a(this: Ptr<Self>) -> Ptr<Ptr<node_a>> {
        this.reinterpret_cast()
    }
    pub fn to_b(this: Ptr<Self>) -> Ptr<Ptr<node_b>> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
pub fn __cpp2rust_init_globals() {}
