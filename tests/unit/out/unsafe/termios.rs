extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_tcgetattr_0() {
    let mut t: termios = std::mem::zeroed::<termios>();
    (*libc::__errno_location()) = 0;
    assert!(((((libc::tcgetattr(0, (&mut t as *mut termios))) == (-1_i32)) as i32) != 0));
    assert!(((((*libc::__errno_location()) == (25)) as i32) != 0));
    (*libc::__errno_location()) = 0;
}
pub unsafe fn test_tcsetattr_1() {
    let mut t: termios = std::mem::zeroed::<termios>();
    {
        let byte_0 = ((&mut t as *mut termios) as *mut termios as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<termios>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut t as *mut termios) as *mut termios as *mut ::libc::c_void)
    };
    (*libc::__errno_location()) = 0;
    assert!(
        ((((libc::tcsetattr(-1_i32, 0, (&mut t as *mut termios).cast_const())) == (-1_i32))
            as i32)
            != 0)
    );
    assert!(((((*libc::__errno_location()) == (9)) as i32) != 0));
    (*libc::__errno_location()) = 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_tcgetattr_0() });
    (unsafe { test_tcsetattr_1() });
    return 0;
}
