extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const S, y: *const S) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_lt_1(x: *const S, y: *const S) -> bool {
    return (((*x).data_) < ((*y).data_));
}
#[repr(C)]
#[derive(Default)]
pub struct S {
    data_: i32,
}
impl S {
    pub unsafe fn S(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const S, other as *const S) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const S, self as *const S) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const S, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S::S({ 1 });
    let mut b: S = S::S({ 2 });
    let mut c: S = S::S({ 1 });
    assert!(
        (unsafe {
            let _x: *const S = &a as *const S;
            operator_eq_0(_x, &c as *const S)
        })
    );
    assert!(
        (unsafe {
            let _x: *const S = &a as *const S;
            operator_lt_1(_x, &b as *const S)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const S = &b as *const S;
            operator_lt_1(_x, &a as *const S)
        })
    );
    return 0;
}
