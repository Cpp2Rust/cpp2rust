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
pub struct TakesAddressOfOwnMember {
    pub value: i32,
}
impl TakesAddressOfOwnMember {
    pub unsafe fn parenthesized(&mut self) -> *mut i32 {
        return (&mut (self.value) as *mut i32);
    }
    pub unsafe fn unqualified(&mut self) -> *mut i32 {
        return (&mut self.value as *mut i32);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first: TakesAddressOfOwnMember = TakesAddressOfOwnMember { value: 3 };
    let mut second: TakesAddressOfOwnMember = TakesAddressOfOwnMember { value: 4 };
    let mut p: *mut i32 = (unsafe { TakesAddressOfOwnMember::parenthesized(&mut first) });
    assert!(((p) == (&mut first.value as *mut i32)));
    (*p) = 5;
    assert!(((first.value) == (5)));
    assert!(((second.value) == (4)));
    let mut q: *mut i32 = (unsafe { TakesAddressOfOwnMember::unqualified(&mut second) });
    assert!(((q) == (&mut second.value as *mut i32)));
    (*q) = 6;
    assert!(((first.value) == (5)));
    assert!(((second.value) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
