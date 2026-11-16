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
    let source: Value<MemberPtr<HasTwoInts, i32>> =
        Rc::new(RefCell::new(member_ptr!(HasTwoInts, second)));
    let destination: Value<MemberPtr<HasTwoInts, i32>> =
        Rc::new(RefCell::new(member_ptr!(HasTwoInts, first)));
    match ({ (destination.as_pointer()).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(&(source.as_pointer()).to_any(), 8usize);
            (__tmp_0).clone()
        }
    };
    assert!(({ ((*destination.borrow()) == (*source.borrow())) }));
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!(({ (((obj.as_pointer()).member((*destination.borrow())).read()) == 4) }));
    let null_source: Value<MemberPtr<HasTwoInts, i32>> =
        Rc::new(RefCell::new(<MemberPtr<HasTwoInts, i32>>::default()));
    match ({ (destination.as_pointer()).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(&(null_source.as_pointer()).to_any(), 8usize);
            (__tmp_1).clone()
        }
    };
    assert!(({ (*destination.borrow()).is_null() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
