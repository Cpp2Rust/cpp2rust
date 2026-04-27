extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i64> = Rc::new(RefCell::new((-1_i32 as i64)));
    #[derive(Default)]
    pub union anon_0 {
        pub as_unsigned: Value<Ptr<u64>>,
        pub as_signed: Value<Ptr<i64>>,
    }
    impl Default for anon_0 {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    }
    impl ByteRepr for anon_0 {};
    let pp: Value<anon_0> = <Value<anon_0>>::default();
    (*(*pp.borrow()).as_signed.borrow_mut()) = (x.as_pointer());
    (*(*pp.borrow()).as_unsigned.borrow()).write(42_u64);
    assert!(((*x.borrow()) == 42_i64));
    return 0;
}
