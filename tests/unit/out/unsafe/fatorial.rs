extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fatorial_0(mut n: i32) -> i32 {
    if ((n) == (0)) {
        return 1;
    }
    return ((n)
        * (unsafe {
            let _n: i32 = ((n) - (1));
            fatorial_0(_n)
        }));
}
pub unsafe fn fatorial_by_ref_1(n: *mut i32) {
    if ((*n) == (1)) {
        (*n) *= 1;
        return;
    }
    let mut n_1: i32 = ((*n) - (1));
    (unsafe {
        let _n: *mut i32 = &mut n_1 as *mut i32;
        fatorial_by_ref_1(_n)
    });
    (*n) *= n_1;
}
pub unsafe fn fatorial_by_ptr_2(mut n: *mut i32) {
    if ((*n) == (1)) {
        (*n) *= 1;
        return;
    }
    let mut n_1: i32 = ((*n) - (1));
    (unsafe {
        let _n: *mut i32 = (&mut n_1 as *mut i32);
        fatorial_by_ptr_2(_n)
    });
    (*n) *= n_1;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n1: i32 = 2;
    (unsafe {
        let _n: *mut i32 = (&mut n1 as *mut i32);
        fatorial_by_ptr_2(_n)
    });
    let mut n: i32 = ((n1) + (1));
    (unsafe {
        let _n: *mut i32 = &mut n as *mut i32;
        fatorial_by_ref_1(_n)
    });
    return (unsafe {
        let _n: i32 = n;
        fatorial_0(_n)
    });
}
