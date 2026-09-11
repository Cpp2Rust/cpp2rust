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
pub struct S {
    pub base: i32,
}
impl S {
    pub unsafe fn plain_i32_const(&self, mut x: i32) -> i32 {
        return ((self.base) + (x));
    }
    pub unsafe fn plain_i64_const(&self, mut x: i64) -> i32 {
        return (((self.base) + (x as i32)) + (1));
    }
    pub unsafe fn width_i32__char_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<libc::c_char>() as i32)));
    }
    pub unsafe fn width_i32__int_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<i32>() as i32)));
    }
    pub unsafe fn scale_i32__2_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (2)));
    }
    pub unsafe fn scale_i32__3_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (3)));
    }
    pub unsafe fn count_i32_const(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (0 as i32));
    }
    pub unsafe fn count_i32__int_long_const(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (2 as i32));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Box {
    pub v: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { base: 100 };
    assert!(((unsafe { S::width_i32__char_const(&s, 3,) }) == (103)));
    assert!(((unsafe { S::width_i32__int_const(&s, 3,) }) == (112)));
    assert!(((unsafe { S::scale_i32__2_const(&s, 5,) }) == (110)));
    assert!(((unsafe { S::scale_i32__3_const(&s, 5,) }) == (115)));
    assert!(((unsafe { S::count_i32_const(&s, 1,) }) == (101)));
    assert!(((unsafe { S::count_i32__int_long_const(&s, 1,) }) == (103)));
    assert!(((unsafe { S::plain_i32_const(&s, 1,) }) == (101)));
    assert!(((unsafe { S::plain_i64_const(&s, 1_i64,) }) == (102)));
    let mut b: Box = Box { v: 4 };
    assert!(((b.v) == (4)));
    return 0;
}
