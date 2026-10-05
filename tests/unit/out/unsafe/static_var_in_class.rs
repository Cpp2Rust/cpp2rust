extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut inner_const_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
impl C {
    pub unsafe fn get(&mut self) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_1));
    }
}
pub static mut inner_const_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 2 });
pub type anon_0 = u32;
pub const anon_0_kValue: anon_0 = 3;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn f() -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3));
    }
}
pub static mut counter_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = <C>::default();
    assert!(((unsafe { C::get(&mut c,) }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_2)) == (2)));
    let mut s: S = <S>::default();
    let mut p: *mut S = (&mut s as *mut S);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_2)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_2)) == (2)));
    assert!(((anon_0_kValue as i32) == (3)));
    assert!(((anon_0_kValue as i32) == (3)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3)) = 20;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3)) == (20)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3)) += 5;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3)) == (25)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3)) == (25)));
    assert!(((unsafe { S::f() }) == (25)));
    assert!(((unsafe { S::f() }) == (25)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const inner_const_1);
    std::cell::LazyCell::force(&*&raw const inner_const_2);
    std::cell::LazyCell::force(&*&raw const counter_3);
}
