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
    let mut buf: [u8; 2] = [0_u8; 2];
    let mut p: *mut u8 = buf.as_mut_ptr();
    let mut q: *mut u8 = std::ptr::null_mut();
    q = {
        p = (p).wrapping_add(1 as i32 as usize);
        p
    };
    assert!(((((q) == (buf.as_mut_ptr().offset((1) as isize))) as i32) != 0));
    let mut out: u8 = 0_u8;
    'switch: {
        let __match_cond = (({
            out = (('x' as i32) as u8);
            out
        }) as i32);
        match __match_cond {
            v if v == ('x' as i32) => {
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
