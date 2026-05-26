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
pub static mut s_alpha: Inner = unsafe { Inner { value: 1 } };
pub static mut s_beta: Inner = unsafe { Inner { value: 2 } };
pub static mut s_shared: Inner = unsafe { Inner { value: 42 } };
pub static mut s_items: [*mut Inner; 2] = unsafe {
    [
        (&raw mut s_alpha as *mut Inner),
        (&raw mut s_beta as *mut Inner),
    ]
};
pub static mut s_obj: Outer = unsafe {
    Outer {
        p: (&raw mut s_shared as *mut Inner),
    }
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*s_items[(0) as usize]).value) == (1)));
    assert!((((*s_items[(1) as usize]).value) == (2)));
    assert!((((*s_obj.p).value) == (42)));
    static mut s_cache: [*mut Inner; 2] = unsafe {
        [
            (&raw mut s_alpha as *mut Inner),
            (&raw mut s_beta as *mut Inner),
        ]
    };;
    assert!((((*s_cache[(0) as usize]).value) == (1)));
    assert!((((*s_cache[(1) as usize]).value) == (2)));
    return 0;
}
