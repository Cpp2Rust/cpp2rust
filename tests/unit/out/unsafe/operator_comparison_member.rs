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
    pub unsafe fn operator_eq(&self, o: *const S) -> bool {
        return ((self.v) == ((*o).v));
    }
    pub unsafe fn operator_ne(&self, o: *const S) -> bool {
        return ((self.v) != ((*o).v));
    }
    pub unsafe fn operator_lt_pconstS_const(&self, o: *const S) -> bool {
        return ((self.v) < ((*o).v));
    }
    pub unsafe fn operator_gt(&self, o: *const S) -> bool {
        return ((self.v) > ((*o).v));
    }
    pub unsafe fn operator_le(&self, o: *const S) -> bool {
        return ((self.v) <= ((*o).v));
    }
    pub unsafe fn operator_ge(&self, o: *const S) -> bool {
        return ((self.v) >= ((*o).v));
    }
    pub unsafe fn operator_lt_i32_const(&self, mut o: i32) -> bool {
        return ((self.v) < (o));
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if S::operator_lt_pconstS_const(self, other as *const S) {
                std::cmp::Ordering::Less
            } else if S::operator_lt_pconstS_const(other, self as *const S) {
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
        unsafe { S::operator_eq(self, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 1 };
    let mut b: S = S { v: 2 };
    let mut c: S = S { v: 1 };
    assert!((unsafe { S::operator_eq(&a, &c as *const S,) }));
    assert!((unsafe { S::operator_ne(&a, &b as *const S,) }));
    assert!((unsafe { S::operator_lt_pconstS_const(&a, &b as *const S,) }));
    assert!((unsafe { S::operator_gt(&b, &a as *const S,) }));
    assert!((unsafe { S::operator_le(&a, &c as *const S,) }));
    assert!((unsafe { S::operator_ge(&a, &c as *const S,) }));
    assert!(!(unsafe { S::operator_lt_pconstS_const(&b, &a as *const S,) }));
    assert!((unsafe { S::operator_lt_i32_const(&a, 5,) }));
    return 0;
}
