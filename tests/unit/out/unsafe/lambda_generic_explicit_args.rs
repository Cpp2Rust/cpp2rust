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
pub struct Val {
    pub x: i32,
}
pub unsafe fn sum_0(mut a: Val, mut b: Val) -> i32 {
    return ((a.x) + (b.x));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut total: i32 = 0;
    let mut tally: lambda_1 = (lambda_1 { total: &mut total });
    (unsafe { tally.operator_call_char_char_const() });
    (unsafe { tally.operator_call_int_char_const() });
    assert!(((total) == (7)));
    let mut v: Val = Val { x: 5 };
    let mut acc: i32 = 0;
    let mut pick: lambda_2 = (lambda_2 {
        v: &mut v,
        acc: &mut acc,
    });
    (unsafe { pick.operator_call_struct_Val_ref_const() });
    (unsafe { pick.operator_call_const_struct_Val_ref_const() });
    (unsafe { pick.operator_call_struct_Val_refref_const() });
    assert!(((acc) == (30)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    total: *mut i32,
}
impl lambda_1 {
    pub unsafe fn operator_call_char_char_const(&self) {
        (*self.total) = (((*self.total) as usize).wrapping_add(
            ((::std::mem::size_of::<libc::c_char>() as usize)
                .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize))
                as usize),
        )) as i32;
    }
    pub unsafe fn operator_call_int_char_const(&self) {
        (*self.total) = (((*self.total) as usize).wrapping_add(
            ((::std::mem::size_of::<i32>() as usize)
                .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize))
                as usize),
        )) as i32;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {
    v: *mut Val,
    acc: *mut i32,
}
impl lambda_2 {
    pub unsafe fn operator_call_struct_Val_ref_const(&self) {
        (*self.acc) += (unsafe {
            let _a: Val = (*self.v);
            let _b: Val = (*self.v);
            sum_0(_a, _b)
        });
    }
    pub unsafe fn operator_call_const_struct_Val_ref_const(&self) {
        (*self.acc) += (unsafe {
            let _a: Val = (*self.v);
            let _b: Val = (*self.v);
            sum_0(_a, _b)
        });
    }
    pub unsafe fn operator_call_struct_Val_refref_const(&self) {
        (*self.acc) += (unsafe {
            let _a: Val = (*self.v);
            let _b: Val = (*self.v);
            sum_0(_a, _b)
        });
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
