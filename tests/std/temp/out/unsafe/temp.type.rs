extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_
{}
pub static mut count_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_
{}
pub static mut count_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_nullptr_ {}
pub unsafe fn test_3() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_1)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)) == (0)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_1)) += 2;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_1)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)) == (0)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)) += 3;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_1)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)) == (3)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count_1);
    std::cell::LazyCell::force(&*&raw const count_2);
}
