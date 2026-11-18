extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl p1_template_constructor_from_own_type_Probe {
    pub unsafe fn new_1(u: *mut p1_template_constructor_from_own_type_Probe) -> Self {
        let mut this = Self {
            value: (((*u).value) + (1)),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new_2, value)]
pub struct p1_template_constructor_from_own_type_Probe {
    pub value: i32,
}
impl p1_template_constructor_from_own_type_Probe {
    pub unsafe fn new_2(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
pub unsafe fn test_0() {
    let mut probe: p1_template_constructor_from_own_type_Probe =
        p1_template_constructor_from_own_type_Probe::new_2({ 22 });
    let mut converted: p1_template_constructor_from_own_type_Probe =
        p1_template_constructor_from_own_type_Probe::new_1({ &mut probe });
    assert!(((converted.value) == (23)));
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
pub unsafe fn __cpp2rust_init_globals() {}
