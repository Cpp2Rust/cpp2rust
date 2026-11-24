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
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 2] = [(0 as libc::c_char); 2];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut q: *mut libc::c_char = std::ptr::null_mut();
    q = match ({ 1 }, &mut (p)) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (*__tmp_1).offset((__tmp_0 as isize));
            (*__tmp_1).clone()
        }
    };
    assert!(((q) == (buf.as_mut_ptr().offset((1) as isize))));
    q = match ({ 1 }, &mut (p)) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (*__tmp_3).offset(-(__tmp_2 as isize));
            (*__tmp_3).clone()
        }
    };
    assert!(((p) == (buf.as_mut_ptr())));
    assert!(((q) == (buf.as_mut_ptr())));
    let mut out: libc::c_char = (0 as libc::c_char);
    'switch: {
        match {
            ((match ({ (('x' as i32) as libc::c_char) }, &mut (out)) {
                (__tmp_4, __tmp_5) => {
                    (*__tmp_5) = __tmp_4;
                    (*__tmp_5).clone()
                }
            }) as i32)
        } {
            __v if __v == ('x' as i32) => {
                assert!(true);
                break 'switch;
            }
            _ => {
                assert!(false);
                break 'switch;
            }
        }
    };
    assert!(((out as i32) == ('x' as i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
