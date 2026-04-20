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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr1: [i32; 5] = [5, 2, 8, 1, 3];
    {
        let len = arr1
            .as_mut_ptr()
            .offset((5) as isize)
            .offset_from(arr1.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(arr1.as_mut_ptr(), len).sort_by(|x, y| {
            if (|x: i32, y: i32| {
                return ((x) < (y));
            })(*x, *y)
            {
                std::cmp::Ordering::Less
            } else if (|x: i32, y: i32| {
                return ((x) < (y));
            })(*y, *x)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    return 0;
}
