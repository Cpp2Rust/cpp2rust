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
    let mut null_pm: MemberPtr<HasTwoInts, i32> = <MemberPtr<HasTwoInts, i32>>::default();
    let mut first_pm: MemberPtr<HasTwoInts, i32> = member_ptr!(HasTwoInts, first);
    let mut second_pm: MemberPtr<HasTwoInts, i32> = member_ptr!(HasTwoInts, second);
    assert!((first_pm != null_pm));
    assert!((second_pm != null_pm));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
