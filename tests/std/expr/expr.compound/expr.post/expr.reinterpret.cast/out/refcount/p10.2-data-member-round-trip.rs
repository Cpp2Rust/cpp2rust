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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasChar {
    #[offset(0)]
    pub character: i8,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut converted: MemberPtr<HasChar, i8> =
        <MemberPtr<HasChar, i8>>::cast_from(member_ptr!(HasTwoInts, second));
    let mut back: MemberPtr<HasTwoInts, i32> = <MemberPtr<HasTwoInts, i32>>::cast_from(converted);
    assert!((back == member_ptr!(HasTwoInts, second)));
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!((((obj.as_pointer()).member(back).read()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
