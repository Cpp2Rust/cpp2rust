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
    let mut original: i32 = 67305985;
    let mut halves: *mut i16 = ((&mut original as *mut i32) as *mut i16);
    assert!((((*halves.offset((0) as isize)) as i32) == (513)));
    assert!((((*halves.offset((1) as isize)) as i32) == (1027)));
    (*halves.offset((0) as isize)) = 4112_i16;
    assert!(((original) == (67309584)));
    let mut arr: [i16; 2] = [513_i16, 1027_i16];
    let mut as_int: *mut i32 = (arr.as_mut_ptr() as *mut i32);
    assert!(((*as_int) == (67305985)));
    return 0;
}
