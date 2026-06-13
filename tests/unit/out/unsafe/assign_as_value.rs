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
    let mut buf: [core::ffi::c_char; 2] = [(0 as core::ffi::c_char); 2];
    let mut p: *mut core::ffi::c_char = buf.as_mut_ptr();
    let mut q: *mut core::ffi::c_char = std::ptr::null_mut();
    q = {
        p = (p).wrapping_add((1 as i32) as usize);
        p
    };
    assert!(((((q) == (buf.as_mut_ptr().offset((1) as isize))) as i32) != 0));
    let mut out: core::ffi::c_char = (0 as core::ffi::c_char);
    'switch: {
        let __match_cond = (({
            out = (('x' as i32) as core::ffi::c_char);
            out
        }) as i32);
        match __match_cond {
            __v if __v == ('x' as i32) => {
                assert!((1 != 0));
                break 'switch;
            }
            _ => {
                assert!((0 != 0));
                break 'switch;
            }
        }
    };
    assert!(((((out as i32) == ('x' as i32)) as i32) != 0));
    return 0;
}
