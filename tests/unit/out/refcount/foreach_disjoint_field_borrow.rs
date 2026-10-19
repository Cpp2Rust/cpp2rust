extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Vec<i32>,
    #[offset(24)]
    pub a: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    {
        let __a1 = 1;
        (*s.borrow_mut()).v.push(__a1)
    };
    'loop_: for e in IterRange::new(
        RandomAccessIterator::Vec((field_ref!(s.as_pointer(), v).as_pointer() as Ptr<i32>)),
        RandomAccessIterator::Vec(
            (field_ref!(s.as_pointer(), v).as_pointer() as Ptr<i32>).to_end(),
        ),
    ) {
        let mut e: i32 = (e.iter_deref().read());
        (*s.borrow_mut()).a.postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
