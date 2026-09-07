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
    pub unsafe fn operator_add_i32(&mut self, mut a: i32) -> i32 {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_add_i32_const(&self, mut a: i32) -> i32 {
        return (((self.v) + (a)) + (1));
    }
    pub unsafe fn operator_add_i32(&mut self, mut a: i32) -> i32 {
        return (((self.v) + (a)) + (2));
    }
    pub unsafe fn operator_sub_i32(&mut self, mut a: i32) -> i32 {
        return ((self.v) - (a));
    }
    pub unsafe fn operator_sub_i32(&mut self, mut a: i32) -> i32 {
        return (((self.v) - (a)) - (1));
    }
    pub unsafe fn operator_mul_i32_const(&self, mut a: i32) -> i32 {
        return ((self.v) * (a));
    }
    pub unsafe fn operator_mul_i32_const(&self, mut a: i32) -> i32 {
        return (((self.v) * (a)) * (2));
    }
    pub unsafe fn operator_index_i32(&mut self, mut i: i32) -> i32 {
        return ((self.v) + (i));
    }
    pub unsafe fn operator_index_i32_const(&self, mut i: i32) -> i32 {
        return (((self.v) + (i)) + (100));
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 10 };
    let cs: S = S { v: 10 };
    let mut vs: S = S { v: 10 };
    assert!(((unsafe { S::operator_add_i32(&mut s, 1,) }) == (11)));
    assert!(((unsafe { S::operator_add_i32_const(&cs, 1,) }) == (12)));
    assert!(((unsafe { S::operator_add_i32(&mut vs, 1,) }) == (13)));
    assert!(((unsafe { S::operator_sub_i32(&mut s, 1,) }) == (9)));
    assert!(((unsafe { S::operator_sub_i32(&mut S { v: 10 }, 1,) }) == (8)));
    assert!(((unsafe { S::operator_mul_i32_const(&s, 3,) }) == (30)));
    assert!(((unsafe { S::operator_mul_i32_const(&cs, 3,) }) == (30)));
    assert!(((unsafe { S::operator_mul_i32_const(&S { v: 10 }, 3,) }) == (60)));
    assert!(((unsafe { S::operator_index_i32(&mut s, 2,) }) == (12)));
    assert!(((unsafe { S::operator_index_i32_const(&cs, 2,) }) == (112)));
    return 0;
}
