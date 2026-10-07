extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn inc_1() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) += 1;
}
pub unsafe fn dec_2() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) -= 1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: i32 = 15;
    let h_ref1: *mut i32 = &mut h;
    (*h_ref1) = 16;
    let mut h_ptr: *mut i32 = (h_ref1);
    let h_ref2: *mut i32 = &mut (*h_ptr);
    (*h_ref2) = 17;
    assert!((((*h_ref1) + (*h_ref2)) == (34)));
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let r: *mut i32 = if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    (*r) = 10;
    assert!(((a) == (10)));
    let cr: *const i32 = if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    assert!(((*cr) == (10)));
    let x: i32 = 1;
    let y: i32 = 2;
    let cx: *const i32 = if ((x) < (y)) {
        (&x as *const i32)
    } else {
        (&y as *const i32)
    };
    assert!(((*cx) == (1)));
    let mut cp: *const i32 = (if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    })
    .cast_const();
    assert!(((*cp) == (10)));
    let mut mp: *mut i32 = if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    (*mp) = 20;
    assert!(((b) == (20)));
    (*if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    }) = 30;
    assert!(((a) == (30)));
    assert!(((b) == (20)));
    (*if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    }) += 5;
    assert!(((b) == (25)));
    let mut ap: *mut i32 = (if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    });
    (*ap) = 40;
    assert!(((a) == (40)));
    if ((a) < (b)) {
        (unsafe { inc_1() });
    } else {
        (unsafe { dec_2() });
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) == (-1_i32)));
    if ((a) > (b)) {
        (unsafe { inc_1() });
    } else {
        (unsafe { dec_2() });
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_0);
}
