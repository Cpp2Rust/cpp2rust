extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i64> = Rc::new(RefCell::new((-1_i32 as i64)));
    let pp: Value<anon_0> = <Value<anon_0>>::default();
    anon_0::as_signed(pp.as_pointer()).write((x.as_pointer()));
    (anon_0::as_unsigned(pp.as_pointer()).read()).write(42_u64);
    assert!(((((*x.borrow()) == 42_i64) as i32) != 0));
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
    pub fn as_unsigned(this: Ptr<Self>) -> Ptr<Ptr<u64>> {
        this.reinterpret_cast()
    }
    pub fn as_signed(this: Ptr<Self>) -> Ptr<Ptr<i64>> {
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
