extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(ByteRepr)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn bytes(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn aligner(&self) -> Ptr<AnyPtr> {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<node>,
    #[offset(8)]
    #[byte_size(8)]
    pub x: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = <Value<node>>::default();
    (*n.borrow_mut()).next = Ptr::<node>::null();
    ((*n.borrow()).x.bytes().reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((0) as isize)
        .write(171_u8);
    assert!(
        (((((((*n.borrow()).x.bytes().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == 171) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
