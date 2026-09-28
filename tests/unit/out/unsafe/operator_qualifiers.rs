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
    pub v: i32,
}
impl S {
    pub unsafe fn operator_add_1(&mut self, mut a: i32) -> i32 {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_add_2(&self, mut a: i32) -> i32 {
        return (((self.v) + (a)) + (1));
    }
    pub unsafe fn operator_add_3(&mut self, mut a: i32) -> i32 {
        return (((self.v) + (a)) + (2));
    }
    pub unsafe fn operator_sub_4(&mut self, mut a: i32) -> i32 {
        return ((self.v) - (a));
    }
    pub unsafe fn operator_sub_5(&mut self, mut a: i32) -> i32 {
        return (((self.v) - (a)) - (1));
    }
    pub unsafe fn operator_mul_6(&self, mut a: i32) -> i32 {
        return ((self.v) * (a));
    }
    pub unsafe fn operator_mul_7(&self, mut a: i32) -> i32 {
        return (((self.v) * (a)) * (2));
    }
    pub unsafe fn operator_index_8(&mut self, mut i: i32) -> i32 {
        return ((self.v) + (i));
    }
    pub unsafe fn operator_index_9(&self, mut i: i32) -> i32 {
        return (((self.v) + (i)) + (100));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 10 };
    let cs: S = S { v: 10 };
    let mut vs: S = S { v: 10 };
    assert!(((unsafe { S::operator_add_1(&mut s, 1,) }) == (11)));
    assert!(((unsafe { S::operator_add_2(&cs, 1,) }) == (12)));
    assert!(((unsafe { S::operator_add_3(&mut vs, 1,) }) == (13)));
    assert!(((unsafe { S::operator_sub_4(&mut s, 1,) }) == (9)));
    assert!(((unsafe { S::operator_sub_5(&mut S { v: 10 }, 1,) }) == (8)));
    assert!(((unsafe { S::operator_mul_6(&s, 3,) }) == (30)));
    assert!(((unsafe { S::operator_mul_6(&cs, 3,) }) == (30)));
    assert!(((unsafe { S::operator_mul_7(&S { v: 10 }, 3,) }) == (60)));
    assert!(((unsafe { S::operator_index_8(&mut s, 2,) }) == (12)));
    assert!(((unsafe { S::operator_index_9(&cs, 2,) }) == (112)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
