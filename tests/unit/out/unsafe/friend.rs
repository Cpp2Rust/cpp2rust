extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(v: *const V) -> i32 {
    return (*v).x;
}
pub unsafe fn operator_eq_1(a: *const V, b: *const V) -> bool {
    return (((*a).x) == ((*b).x));
}
pub unsafe fn scaled_2(v: *const V, mut k: i32) -> i32 {
    return (((*v).x) * (k));
}
pub unsafe fn scaled_3(v: *const V, mut k: f64) -> f64 {
    return (((*v).x as f64) * (k));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct V {
    pub x: i32,
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const V, other as *const V) }
    }
}
impl std::cmp::Eq for V {}
pub unsafe fn get_4(w: *const W_int_) -> i32 {
    return (*w).x;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct W_int_ {
    pub x: i32,
}
pub unsafe fn get_5(w: *const W_long_) -> i64 {
    return (*w).x;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct W_long_ {
    pub x: i64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct D {
    pub x: i32,
}
pub unsafe fn declared_then_defined_6(d: *const D) -> i32 {
    return (((*d).x) + (1));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: V = V { x: 3 };
    let mut b: V = V { x: 3 };
    let mut c: V = V { x: 4 };
    assert!(((unsafe { get_0(&a as *const V,) }) == (3)));
    assert!(
        (unsafe {
            let _a: *const V = &a as *const V;
            operator_eq_1(_a, &b as *const V)
        })
    );
    assert!(
        !(unsafe {
            let _a: *const V = &a as *const V;
            operator_eq_1(_a, &c as *const V)
        })
    );
    assert!(((unsafe { scaled_2(&c as *const V, 2,) }) == (8)));
    assert!(((unsafe { scaled_3(&c as *const V, 1.5E+0,) }) == (6.0E+0)));
    let mut wi: W_int_ = W_int_ { x: 5 };
    let mut wl: W_long_ = W_long_ { x: 6_i64 };
    assert!(((unsafe { get_4(&wi as *const W_int_,) }) == (5)));
    assert!(((unsafe { get_5(&wl as *const W_long_,) }) == (6_i64)));
    let mut d: D = D { x: 7 };
    assert!(((unsafe { declared_then_defined_6(&d as *const D,) }) == (8)));
    return 0;
}
