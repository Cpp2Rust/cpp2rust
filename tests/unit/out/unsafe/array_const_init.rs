extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub head: i32,
    pub tail: [i32; 3],
    pub buf: [::core::ffi::c_char; 4],
}
impl Default for S {
    fn default() -> Self {
        S {
            head: 0_i32,
            tail: [0_i32; 3],
            buf: [(0 as ::core::ffi::c_char); 4],
        }
    }
}
pub static mut s_0: S = unsafe {
    S {
        head: 5,
        tail: [0; 3],
        buf: [0; 4],
    }
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((s_0.head) == (5)) as i32) != 0));
    let mut i: i32 = 0;
    'loop_: while ((((i) < (3)) as i32) != 0) {
        assert!(((((s_0.tail[(i) as usize]) == (0)) as i32) != 0));
        i.postfix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((((i) < (4)) as i32) != 0) {
        assert!(((((s_0.buf[(i) as usize] as i32) == (0)) as i32) != 0));
        i.postfix_inc();
    }
    return 0;
}
