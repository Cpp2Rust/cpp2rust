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
    let pm: Value<MemberPtr<HasTwoInts, i32>> =
        Rc::new(RefCell::new(member_ptr!(HasTwoInts, second)));
    let bytes: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..8).map(|_| 0_u8).collect::<Box<[u8]>>()));
    match ({ (bytes.decay()).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(&(pm.as_pointer()).to_any(), 8usize);
            (__tmp_0).clone()
        }
    };
    (*pm.borrow_mut()) = member_ptr!(HasTwoInts, first);
    match ({ (pm.as_pointer()).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(&(bytes.decay()).to_any(), 8usize);
            (__tmp_1).clone()
        }
    };
    assert!(({ ((*pm.borrow()) == member_ptr!(HasTwoInts, second)) }));
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!(({ (((obj.as_pointer()).member((*pm.borrow())).read()) == 4) }));
    let null_pm: Value<MemberPtr<HasTwoInts, i32>> =
        Rc::new(RefCell::new(<MemberPtr<HasTwoInts, i32>>::default()));
    match ({ (bytes.decay()).to_any() },) {
        (mut __tmp_2,) => {
            __tmp_2.memcpy(&(null_pm.as_pointer()).to_any(), 8usize);
            (__tmp_2).clone()
        }
    };
    (*null_pm.borrow_mut()) = member_ptr!(HasTwoInts, first);
    match ({ (null_pm.as_pointer()).to_any() },) {
        (mut __tmp_3,) => {
            __tmp_3.memcpy(&(bytes.decay()).to_any(), 8usize);
            (__tmp_3).clone()
        }
    };
    assert!(({ (*null_pm.borrow()).is_null() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
