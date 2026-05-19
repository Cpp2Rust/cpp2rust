extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_sigaction_0() {
    let mut sa: sigaction = std::mem::zeroed::<sigaction>();
    assert!(
        ((((libc::sigaction(2, std::ptr::null(), (&mut sa as *mut sigaction))) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::sigaction(15, std::ptr::null(), (&mut sa as *mut sigaction))) == (0)) as i32)
            != 0)
    );
    (*libc::__errno_location()) = 0;
    assert!(
        ((((libc::sigaction(99999, std::ptr::null(), (&mut sa as *mut sigaction))) == (-1_i32))
            as i32)
            != 0)
    );
    assert!(((((*libc::__errno_location()) == (22)) as i32) != 0));
    (*libc::__errno_location()) = 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_sigaction_0() });
    return 0;
}
