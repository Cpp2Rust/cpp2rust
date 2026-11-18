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
pub struct p4_1_both_null_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_0() {
    let mut first_null: isize = -1_isize;
    let mut second_null: isize = -1_isize;
    assert!((first_null == second_null));
    assert!(!(first_null != second_null));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p4_2_one_null_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_1() {
    let mut null_pm: isize = -1_isize;
    let mut first_pm: isize = (offset_of_field!(p4_2_one_null_HasTwoInts, first) as isize);
    assert!((first_pm != null_pm));
    assert!(!(first_pm == null_pm));
    assert!((null_pm != first_pm));
    assert!(!(null_pm == first_pm));
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct p4_5_members_of_same_union_HasTwoIntMembers {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl p4_5_members_of_same_union_HasTwoIntMembers {
    pub fn first(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn second(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
}
impl Default for p4_5_members_of_same_union_HasTwoIntMembers {
    fn default() -> Self {
        p4_5_members_of_same_union_HasTwoIntMembers {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub fn test_2() {
    let mut first_pm: isize = 0_isize;
    let mut second_pm: isize = 0_isize;
    assert!((first_pm == second_pm));
    assert!(!(first_pm != second_pm));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p4_6_data_members_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_3() {
    let mut first_pm: isize = (offset_of_field!(p4_6_data_members_HasTwoInts, first) as isize);
    let mut also_first_pm: isize = (offset_of_field!(p4_6_data_members_HasTwoInts, first) as isize);
    let mut second_pm: isize = (offset_of_field!(p4_6_data_members_HasTwoInts, second) as isize);
    assert!((first_pm == also_first_pm));
    assert!(!(first_pm != also_first_pm));
    assert!((first_pm != second_pm));
    assert!(!(first_pm == second_pm));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
