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
pub static mut alpha_0: Inner = unsafe { Inner { value: 1 } };
pub static mut beta_1: Inner = unsafe { Inner { value: 2 } };
pub static mut shared_2: Inner = unsafe { Inner { value: 42 } };
pub static mut items_3: [*mut Inner; 2] = unsafe {
    [
        (&raw mut alpha_0 as *mut Inner),
        (&raw mut beta_1 as *mut Inner),
    ]
};
pub static mut obj_4: Outer = unsafe {
    Outer {
        p: (&raw mut shared_2 as *mut Inner),
    }
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*items_3[(0) as usize]).value) == (1)));
    assert!((((*items_3[(1) as usize]).value) == (2)));
    assert!((((*obj_4.p).value) == (42)));
    static mut cache_5: [*mut Inner; 2] = unsafe {
        [
            (&raw mut alpha_0 as *mut Inner),
            (&raw mut beta_1 as *mut Inner),
        ]
    };;
    assert!((((*cache_5[(0) as usize]).value) == (1)));
    assert!((((*cache_5[(1) as usize]).value) == (2)));
    return 0;
}
