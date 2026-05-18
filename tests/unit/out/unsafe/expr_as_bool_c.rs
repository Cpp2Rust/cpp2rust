extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmp_eq_0(mut rc: i32) -> i32 {
    return (((rc) == (-1_i32)) as i32);
}
pub unsafe fn cmp_or_ptr_1(mut p: *const u8, mut q: *const u8) -> i32 {
    return (((!(p).is_null()) || (!(q).is_null())) as i32);
}
pub unsafe fn both_null_2(mut s1: *const u8, mut s2: *const u8) -> i32 {
    return ((((((s1).is_null()) as i32) != 0) && ((((s2).is_null()) as i32) != 0)) as i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0_i32;
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (((({
        b = a;
        b
    }) != (0)) as i32)
        != 0)
    {}
    if (a != 0) {}
    if ((((a) == (b)) as i32) != 0) {}
    if ((((a) < (b)) as i32) != 0) {}
    assert!(((((a) == (b)) as i32) != 0));
    assert!(
        ((!(({
            a = b;
            a
        }) != 0) as i32)
            != 0)
    );
    let mut c: bool = false;
    c = ({
        a = b;
        a
    } != 0);
    c = (((({
        b = a;
        b
    }) != (0)) as i32)
        != 0);
    c = (a != 0);
    c = ((((a) == (b)) as i32) != 0);
    c = ((((a) < (b)) as i32) != 0);
    let mut x: i32 = 5;
    let mut y: i32 = 5;
    let mut eq: i32 = (((x) == (y)) as i32);
    let mut lt: i32 = (((x) < (y)) as i32);
    let mut neq: i32 = (((x) != (y)) as i32);
    assert!(((((eq) == (1)) as i32) != 0));
    assert!(((((lt) == (0)) as i32) != 0));
    assert!(((((neq) == (0)) as i32) != 0));
    let mut p1: *const u8 = b"hi\0".as_ptr().cast_mut().cast_const();
    let mut p2: *const u8 = std::ptr::null();
    let mut either: i32 = (((!(p1).is_null()) || (!(p2).is_null())) as i32);
    let mut both: i32 = (((!(p1).is_null()) && (!(p2).is_null())) as i32);
    assert!(((((either) == (1)) as i32) != 0));
    assert!(((((both) == (0)) as i32) != 0));
    assert!(
        ((((unsafe {
            let _rc: i32 = -1_i32;
            cmp_eq_0(_rc)
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _rc: i32 = 0;
            cmp_eq_0(_rc)
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _p: *const u8 = p1;
            let _q: *const u8 = p2;
            cmp_or_ptr_1(_p, _q)
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _p: *const u8 = std::ptr::null();
            let _q: *const u8 = std::ptr::null();
            cmp_or_ptr_1(_p, _q)
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _s1: *const u8 = std::ptr::null();
            let _s2: *const u8 = std::ptr::null();
            both_null_2(_s1, _s2)
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _s1: *const u8 = p1;
            let _s2: *const u8 = std::ptr::null();
            both_null_2(_s1, _s2)
        }) == (0)) as i32)
            != 0)
    );
    return 0;
}
