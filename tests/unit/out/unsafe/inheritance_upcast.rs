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
pub struct Base {
    pub buf: *mut i32,
    pub n: usize,
}
impl Base {
    pub unsafe fn Base(mut b: *mut i32, mut n: usize) -> Self {
        let mut this = Self { buf: b, n: n };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Derived {
    pub __base: Base,
}
pub unsafe fn count_0(b: *const Base) -> usize {
    return (*b).n;
}
pub unsafe fn first_1(mut p: *mut Base) -> i32 {
    return (*(*p).buf.offset((0) as isize));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i32; 3] = [7, 8, 9];
    let mut d: Derived = Derived::Derived1({ arr.as_mut_ptr() }, { 3_usize });
    assert!(((unsafe { count_0(&d as *const Base,) }) == (3_usize)));
    assert!(((unsafe { first_1((&mut d as *mut Derived),) }) == (7)));
    let mut copy: Base = d;
    assert!(((copy.n) == (3_usize)));
    assert!(((copy.buf) == (arr.as_mut_ptr())));
    return 0;
}
