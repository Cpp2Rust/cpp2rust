extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inner {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub p: *mut Inner,
}
pub static mut alpha: Inner = Inner { value: 1 };
pub static mut beta: Inner = Inner { value: 2 };
pub static mut shared: Inner = Inner { value: 42 };
pub static mut items: [*mut Inner; 2] = [
    (&raw mut alpha as *mut Inner),
    (&raw mut beta as *mut Inner),
];
pub static mut obj: Outer = Outer {
    p: (&raw mut shared as *mut Inner),
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*items[(0) as usize]).value) == (1)));
    assert!((((*items[(1) as usize]).value) == (2)));
    assert!((((*obj.p).value) == (42)));
    static mut cache: [*mut Inner; 2] = [
        (&raw mut alpha as *mut Inner),
        (&raw mut beta as *mut Inner),
    ];;
    assert!((((*cache[(0) as usize]).value) == (1)));
    assert!((((*cache[(1) as usize]).value) == (2)));
    return 0;
}
