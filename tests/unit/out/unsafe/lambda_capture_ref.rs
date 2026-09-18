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
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    let mut add_base: lambda_0 = (lambda_0 { base: &mut base });
    assert!(((unsafe { lambda_0::operator_call(&add_base, 5,) }) == (15)));
    base = 100;
    assert!(((unsafe { lambda_0::operator_call(&add_base, 5,) }) == (105)));
    let mut s: S = S { x: 1, y: 2 };
    let mut sum: lambda_1 = (lambda_1 { s: &mut s });
    assert!(((unsafe { lambda_1::operator_call(&sum,) }) == (3)));
    s.x = 50;
    assert!(((unsafe { lambda_1::operator_call(&sum,) }) == (52)));
    let mut counter: i32 = 0;
    let mut bump: lambda_2 = (lambda_2 {
        counter: &mut counter,
    });
    (unsafe { lambda_2::operator_call(&bump) });
    (unsafe { lambda_2::operator_call(&bump) });
    assert!(((counter) == (2)));
    let mut arr: [u16; 4] = [3_u16, 1_u16, 2_u16, 0_u16];
    let mut swap: lambda_3 = (lambda_3 { arr: &mut arr });
    (unsafe { lambda_3::operator_call(&swap, 0_usize, 3_usize) });
    assert!(((arr[(0) as usize] as i32) == (0)));
    assert!(((arr[(3) as usize] as i32) == (3)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    base: *mut i32,
}
impl lambda_0 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((x) + (*self.base));
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_0::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    s: *mut S,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self) -> i32 {
        return (((*self.s).x) + ((*self.s).y));
    }
}
impl Callable0<i32> for lambda_1 {
    fn call(&self) -> i32 {
        unsafe { lambda_1::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {
    counter: *mut i32,
}
impl lambda_2 {
    pub unsafe fn operator_call(&self) {
        (*self.counter).postfix_inc();
    }
}
impl Callable0<()> for lambda_2 {
    fn call(&self) -> () {
        unsafe { lambda_2::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {
    arr: *mut [u16; 4],
}
impl lambda_3 {
    pub unsafe fn operator_call(&self, mut i: usize, mut j: usize) {
        let mut t: u16 = (*self.arr)[(j)];
        (*self.arr)[(j)] = (*self.arr)[(i)];
        (*self.arr)[(i)] = t;
    }
}
impl Callable2<usize, usize, ()> for lambda_3 {
    fn call(&self, a1: usize, a2: usize) -> () {
        unsafe { lambda_3::operator_call(self, a1, a2) }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
