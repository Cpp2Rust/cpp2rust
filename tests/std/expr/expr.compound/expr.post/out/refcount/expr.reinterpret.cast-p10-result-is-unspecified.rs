extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct AlsoHasTwoInts {
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
    let mut converted: isize = (offset_of_field!(HasTwoInts, second) as isize);
    return ((converted == (offset_of_field!(AlsoHasTwoInts, second) as isize)) as i32);
}
pub fn __cpp2rust_init_globals() {}
