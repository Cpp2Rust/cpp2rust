extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Probe {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Wrapper_Probe_ {
    #[offset(0)]
    #[byte_size(1)]
    pub base_: Probe,
    #[offset(4)]
    pub tag: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Wrapper_Probe_> = Rc::new(RefCell::new(<Wrapper_Probe_>::default()));
    (*a.borrow_mut()).tag = 3;
    let b: Value<Wrapper_Probe_> = Rc::new(RefCell::new((*a.borrow()).clone()));
    assert!(({ (*b.borrow()).tag } == 3));
    return 0;
}
pub trait ProbeImpl {
    fn operator_inc(&self) -> Ptr<Probe> {
        unimplemented!()
    }
}
impl ProbeImpl for Ptr<Probe> {}
pub fn __cpp2rust_init_globals() {}
