extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let value: Value<i32> = Rc::new(RefCell::new(5));
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((value.as_pointer())));
    let mut ptr_ptr: Ptr<Ptr<i32>> = (ptr.as_pointer());
    let mut converted: Ptr<Ptr<i32>> = ptr_ptr.reinterpret_cast::<Ptr<i32>>();
    assert!(({ (converted).clone() } == { (ptr_ptr).clone() }));
    assert!(({ ({ (converted.read()) } == { (*ptr.borrow()).clone() }) }));
    (*value.borrow_mut()) = 7;
    assert!((((converted.read()).read()) == 7));
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p10_null_converts_to_null_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p10_null_converts_to_null_HasChar {
    #[offset(0)]
    pub character: i8,
}
pub fn test_1() {
    let mut pm: isize = -1_isize;
    let mut converted: isize = pm;
    assert!(((converted) == -1_isize));
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p10_2_data_member_round_trip_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p10_2_data_member_round_trip_HasChar {
    #[offset(0)]
    pub character: i8,
}
pub fn test_2() {
    let mut converted: isize =
        (offset_of_field!(p10_2_data_member_round_trip_HasTwoInts, second) as isize);
    let mut back: isize = converted;
    assert!((back == (offset_of_field!(p10_2_data_member_round_trip_HasTwoInts, second) as isize)));
    let obj: Value<p10_2_data_member_round_trip_HasTwoInts> =
        Rc::new(RefCell::new(p10_2_data_member_round_trip_HasTwoInts {
            first: 3,
            second: 4,
        }));
    assert!((((obj.as_pointer()).member::<i32>(back).read()) == 4));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
