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
    pub unsafe fn lt(&self, other: *const Pair) -> bool {
        return (((self.x) < ((*other).x))
            || ((((self.x) == ((*other).x)) && ((self.y) < ((*other).y))) as bool));
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if self.lt(other) {
                std::cmp::Ordering::Less
            } else if other.lt(other) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        unsafe { !(self.lt(other)) && !(other.lt(other)) }
    }
}
impl Eq for Pair {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pair1: Pair = Pair { x: 1, y: 2 };
    let mut pair2: Pair = Pair { x: 1, y: 3 };
    return (pair1.lt(&mut pair2) as i32);
}
