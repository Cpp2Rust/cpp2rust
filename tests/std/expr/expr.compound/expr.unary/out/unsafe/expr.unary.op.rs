extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut shared_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub static mut calls_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_2_static_member_is_ordinary_pointer_HasStaticMembers {}
impl p3_2_static_member_is_ordinary_pointer_HasStaticMembers {
    pub unsafe fn run() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_1)).prefix_inc();
    }
}
pub unsafe fn test_2() {
    let mut object_pointer: *mut i32 =
        (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_0)) as *mut i32);
    assert!(((*object_pointer) == (3)));
    (*object_pointer) = 4;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_0)) == (4)));
    let mut function_pointer: Option<unsafe fn()> =
        (Some(p3_2_static_member_is_ordinary_pointer_HasStaticMembers::run));
    (unsafe { (function_pointer).unwrap()() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_1)) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember {
    pub value: i32,
}
impl p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember {
    pub unsafe fn parenthesized(&mut self) -> *mut i32 {
        return (&mut (self.value) as *mut i32);
    }
    pub unsafe fn unqualified(&mut self) -> *mut i32 {
        return (&mut self.value as *mut i32);
    }
}
pub unsafe fn test_3() {
    let mut first: p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember =
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember { value: 3 };
    let mut second: p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember =
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember { value: 4 };
    let mut p: *mut i32 = (unsafe {
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember::parenthesized(
            &mut first,
        )
    });
    assert!(((p) == (&mut first.value as *mut i32)));
    (*p) = 5;
    assert!(((first.value) == (5)));
    assert!(((second.value) == (4)));
    let mut q: *mut i32 = (unsafe {
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember::unqualified(
            &mut second,
        )
    });
    assert!(((q) == (&mut second.value as *mut i32)));
    (*q) = 6;
    assert!(((first.value) == (5)));
    assert!(((second.value) == (6)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_2() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const shared_0);
    std::cell::LazyCell::force(&*&raw const calls_1);
}
