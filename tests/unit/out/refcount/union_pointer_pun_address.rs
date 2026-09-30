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
    (*ptr.borrow_mut()).to_a().write((a.as_pointer()));
    let out: Value<Ptr<node_b>> = Rc::new(RefCell::new(((*ptr.borrow()).to_b().read()).clone()));
    assert!(((({ (*out.borrow()).to_any() } == { (a.as_pointer()).to_any() }) as i32) != 0));
    return 0;
}
#[derive(ByteRepr)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn to_a(&self) -> Ptr<Ptr<node_a>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn to_b(&self) -> Ptr<Ptr<node_b>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
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
