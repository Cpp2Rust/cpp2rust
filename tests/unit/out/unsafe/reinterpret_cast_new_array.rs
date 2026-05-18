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
    let mut arr: *mut u32 =
        Box::leak((0..2_u64).map(|_| 0_u32).collect::<Box<[u32]>>()).as_mut_ptr();
    (*arr.offset((0) as isize)) = 67305985_u32;
    (*arr.offset((1) as isize)) = 134678021_u32;
    let mut bytes: *mut u8 = (arr as *mut u8);
    assert!((((*bytes.offset((0) as isize)) as i32) == (1)));
    assert!((((*bytes.offset((4) as isize)) as i32) == (5)));
    assert!((((*bytes.offset((7) as isize)) as i32) == (8)));
    (*bytes.offset((0) as isize)) = 170_u8;
    assert!(((*arr.offset((0) as isize)) == (67306154_u32)));
    (*bytes.offset((5) as isize)) = 187_u8;
    assert!(((*arr.offset((1) as isize)) == (134724357_u32)));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        arr,
        libcc2rs::malloc_usable_size(arr as *mut ::libc::c_void) / ::std::mem::size_of::<u32>(),
    )));
    return 0;
}
