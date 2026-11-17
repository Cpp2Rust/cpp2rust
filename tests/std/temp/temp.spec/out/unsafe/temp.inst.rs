extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p2_reference_does_not_instantiate_Member {
    pub value: i32,
}
impl p2_reference_does_not_instantiate_Member {
    pub unsafe fn new_1(_a0: *const p2_reference_does_not_instantiate_Wrapper_int_) -> Self {
        let mut this = Self { value: 2 };
        this
    }
}
impl Default for p2_reference_does_not_instantiate_Member {
    fn default() -> Self {
        unsafe { p2_reference_does_not_instantiate_Member { value: 1 } }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p2_reference_does_not_instantiate_Holder {
    pub member: p2_reference_does_not_instantiate_Member,
}
impl p2_reference_does_not_instantiate_Holder {
    pub unsafe fn move_from(_a0: *mut p2_reference_does_not_instantiate_Holder) -> Self {
        let mut this = Self {
            member: (*_a0).member,
        };
        this
    }
}
pub unsafe fn test_0() {
    let mut holder: p2_reference_does_not_instantiate_Holder =
        <p2_reference_does_not_instantiate_Holder>::default();
    assert!(((holder.member.value) == (1)));
    holder.member.value = 3;
    assert!(((holder.member.value) == (3)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct p2_reference_does_not_instantiate_Wrapper_int_;
pub unsafe fn __cpp2rust_init_globals() {}
