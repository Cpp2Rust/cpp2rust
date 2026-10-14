extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut next: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
        }
    });
    assert!(((unsafe { next.call() }) == (1)));
    assert!(((unsafe { next.call() }) == (2)));
    let mut copy: FnPtr<fn() -> i32> = next.copy_from();
    assert!(((unsafe { copy.call() }) == (3)));
    assert!(((unsafe { next.call() }) == (4)));
    let mut fresh: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
        }
    });
    assert!(((unsafe { fresh.call() }) == (5)));
    assert!(((unsafe { next.call() }) == (6)));
    let mut per_type: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: i32| -> i32 {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2)).postfix_inc();
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2));
        },
        |mut x: f64| -> i32 {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_3)).postfix_inc();
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_3));
        }
    );
    assert!(((unsafe { per_type.spec::<fn(i32) -> i32>(0).call(1,) }) == (1)));
    assert!(((unsafe { per_type.spec::<fn(i32) -> i32>(0).call(2,) }) == (2)));
    assert!(((unsafe { per_type.spec::<fn(f64) -> i32>(1).call(1_f64,) }) == (1)));
    assert!(((unsafe { per_type.spec::<fn(i32) -> i32>(0).call(3,) }) == (3)));
    let mut read: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut _a0: i32| -> i32 {
        unsafe {
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0));
        }
    });
    assert!(((unsafe { read.call(0,) }) == (0)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)) = 1;
    assert!(((unsafe { read.call(0,) }) == (1)));
    let mut local: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            let mut p: P_4 = P_4 { x: 2, y: 3 };
            return ((p.x) * (p.y));
        }
    });
    assert!(((unsafe { local.call() }) == (6)));
    return 0;
}
static mut n_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut calls_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut calls_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P_4 {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const a_0);
}
