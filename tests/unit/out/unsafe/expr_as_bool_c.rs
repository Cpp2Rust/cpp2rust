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
pub unsafe fn cmp_or_ptr_1(mut p: *const libc::c_char, mut q: *const libc::c_char) -> i32 {
    return (((!(p).is_null()) || (!(q).is_null())) as i32);
}
pub unsafe fn both_null_2(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> i32 {
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
    let mut p1: *const libc::c_char = (c"hi".as_ptr().cast_mut()).cast_const();
    let mut p2: *const libc::c_char = std::ptr::null();
    let mut either: i32 = (((!(p1).is_null()) || (!(p2).is_null())) as i32);
    let mut both: i32 = (((!(p1).is_null()) && (!(p2).is_null())) as i32);
    assert!(((((either) == (1)) as i32) != 0));
    assert!(((((both) == (0)) as i32) != 0));
    assert!(((((unsafe { cmp_eq_0(-1_i32,) }) == (1)) as i32) != 0));
    assert!(((((unsafe { cmp_eq_0(0,) }) == (0)) as i32) != 0));
    assert!(((((unsafe { cmp_or_ptr_1(p1, p2,) }) == (1)) as i32) != 0));
    assert!(
        ((((unsafe { cmp_or_ptr_1(std::ptr::null(), std::ptr::null(),) }) == (0)) as i32) != 0)
    );
    assert!(((((unsafe { both_null_2(std::ptr::null(), std::ptr::null(),) }) == (1)) as i32) != 0));
    assert!(((((unsafe { both_null_2(p1, std::ptr::null(),) }) == (0)) as i32) != 0));
    return 0;
}
