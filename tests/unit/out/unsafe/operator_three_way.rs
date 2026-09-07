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
    pub unsafe fn operator_cmp(&self, o: *const S) -> std::cmp::Ordering {
        if ((self.v) < ((*o).v)) {
            return std::cmp::Ordering::Less;
        }
        if ((self.v) > ((*o).v)) {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, o: *const S) -> bool {
        return ((self.v) == ((*o).v));
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 1 };
    let mut b: S = S { v: 2 };
    assert!((unsafe { S::operator_cmp(&a, &b as *const S,) }) == std::cmp::Ordering::Less);
    assert!((unsafe { S::operator_cmp(&b, &a as *const S,) }) == std::cmp::Ordering::Greater);
    assert!((unsafe { S::operator_cmp(&a, &b as *const S,) }) != std::cmp::Ordering::Greater);
    assert!((unsafe { S::operator_cmp(&b, &a as *const S,) }) != std::cmp::Ordering::Less);
    assert!(!(unsafe { S::operator_eq(&a, &b as *const S,) }));
    assert!((unsafe { S::operator_cmp(&a, &b as *const S,) }) == std::cmp::Ordering::Less);
    return 0;
}
