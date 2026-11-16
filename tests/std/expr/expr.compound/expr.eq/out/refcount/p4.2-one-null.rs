extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut null_pm: isize = -1_isize;
    let mut first_pm: isize = (<HasTwoInts as Record>::OFFSETS.first as isize);
    assert!((first_pm != null_pm));
    assert!(!(first_pm == null_pm));
    assert!((null_pm != first_pm));
    assert!(!(null_pm == first_pm));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
