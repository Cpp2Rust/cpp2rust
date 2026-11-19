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
pub struct p1_as_const_Counter {
    pub value: i32,
}
impl p1_as_const_Counter {
    pub unsafe fn get_1(&mut self) -> i32 {
        return ((self.value) + (1));
    }
    pub unsafe fn get_2(&self) -> i32 {
        return self.value;
    }
}
pub unsafe fn test_0() {
    let mut number: i32 = 3;
    let ref_: *const i32 = &number;
    assert!(((ref_) == ((&mut number as *mut i32).cast_const())));
    number = 4;
    assert!(((*ref_) == (4)));
    assert!(((number) == (4)));
    let mut pointer: *mut i32 = (&mut number as *mut i32);
    assert!(((pointer) == (&mut number as *mut i32)));
    let mut counter: p1_as_const_Counter = p1_as_const_Counter { value: 5 };
    assert!(((unsafe { p1_as_const_Counter::get_2(&counter,) }) == (5)));
    assert!(((unsafe { p1_as_const_Counter::get_1(&mut counter,) }) == (6)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
