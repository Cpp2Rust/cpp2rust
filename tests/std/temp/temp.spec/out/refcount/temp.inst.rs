extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p2_reference_does_not_instantiate_Member {
    #[offset(0)]
    pub value: i32,
}
impl p2_reference_does_not_instantiate_Member {
    pub fn new_1(_a0: Ptr<p2_reference_does_not_instantiate_Wrapper_int_>) -> Self {
        Self { value: 2 }
    }
}
impl Default for p2_reference_does_not_instantiate_Member {
    fn default() -> Self {
        { p2_reference_does_not_instantiate_Member { value: 1 } }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct p2_reference_does_not_instantiate_Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub member: p2_reference_does_not_instantiate_Member,
}
impl p2_reference_does_not_instantiate_Holder {
    pub fn move_from(_a0: Ptr<p2_reference_does_not_instantiate_Holder>) -> Self {
        Self {
            member: { (*_a0.upgrade().deref()).member },
        }
    }
}
pub fn test_0() {
    let mut holder: p2_reference_does_not_instantiate_Holder =
        <p2_reference_does_not_instantiate_Holder>::default();
    assert!((holder.member.value == 1));
    holder.member.value = 3;
    assert!((holder.member.value == 3));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p2_reference_does_not_instantiate_Wrapper_int_;
pub fn __cpp2rust_init_globals() {}
