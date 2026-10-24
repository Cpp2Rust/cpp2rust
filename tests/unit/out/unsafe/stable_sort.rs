extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr1: [i32; 5] = [5, 2, 8, 1, 3];
    {
        let __first = arr1.as_mut_ptr();
        let len = arr1.as_mut_ptr().offset((5) as isize).offset_from(__first) as usize;
        let __cmp = FnPtr::<fn(i32, i32) -> bool>::new(|mut x: i32, mut y: i32| -> bool {
            unsafe {
                return ((x) < (y));
            }
        });
        ::std::slice::from_raw_parts_mut(__first, len).sort_by(|x, y| {
            if __cmp.call(*x, *y) {
                std::cmp::Ordering::Less
            } else if __cmp.call(*y, *x) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
