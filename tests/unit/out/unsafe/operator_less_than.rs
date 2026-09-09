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
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
impl Pair {
    pub unsafe fn operator_lt(&mut self, other: *const Pair) -> bool {
        return ((self.x) < ((*other).x))
            || (((self.x) == ((*other).x)) && ((self.y) < ((*other).y)));
    }
}
impl std::cmp::Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Pair::operator_lt(&mut *(&raw const *self).cast_mut(), other as *const Pair) {
                std::cmp::Ordering::Less
            } else if Pair::operator_lt(&mut *(&raw const *other).cast_mut(), self as *const Pair) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Pair::operator_lt(&mut *(&raw const *self).cast_mut(), other as *const Pair))
                && !(Pair::operator_lt(&mut *(&raw const *other).cast_mut(), self as *const Pair))
        }
    }
}
impl std::cmp::Eq for Pair {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pair1: Pair = Pair { x: 1, y: 2 };
    let mut pair2: Pair = Pair { x: 1, y: 3 };
    assert!((unsafe { Pair::operator_lt(&mut pair1, &pair2 as *const Pair,) }));
    return 0;
}
