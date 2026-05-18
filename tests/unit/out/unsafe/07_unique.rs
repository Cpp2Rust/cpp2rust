extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut u: Option<Box<i32>>) -> Option<Box<i32>> {
    (*u.as_deref_mut().unwrap()) = 10;
    return u;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Option<Box<i32>> = Some(Box::new(8));
    (*f.as_deref_mut().unwrap()) = 9;
    let mut f_ptr1: *mut i32 = f
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut i32);
    (*f_ptr1) = 10;
    let mut f_ptr2: *mut i32 = (&mut (*f.as_deref_mut().unwrap()) as *mut i32);
    (*f_ptr2) = 11;
    f = Some(Box::new(9));
    f = (unsafe {
        let _u: Option<Box<i32>> = f;
        fn_0(_u)
    });
    return (*f.as_deref_mut().unwrap());
}
