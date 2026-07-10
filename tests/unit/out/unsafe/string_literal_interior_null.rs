extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_bytes_0(mut buf: *const libc::c_char, mut len: u32) -> i32 {
    let mut sum: i32 = 0;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (len)) {
        sum += (((*buf.offset((i) as isize)) as u8) as i32);
        i.postfix_inc();
    }
    return sum;
}
pub static mut g_packet_1: *const libc::c_char = unsafe {
    (&[
        (1 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ])
        .as_ptr()
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = (unsafe {
        sum_bytes_0(
            (&[
                (1 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            2_u32,
        )
    });
    let mut b: i32 = (unsafe { sum_bytes_0(g_packet_1, 2_u32) });
    assert!(((a) == (b)));
    assert!(((a) == (1)));
    let mut c: i32 = (((*c"\r\n.\r\n".as_ptr().offset((0) as isize)) as i32)
        + ((*c"\r\n.\r\n".as_ptr().offset((3) as isize)) as i32));
    assert!(((c) == ((('\r' as libc::c_char) as i32) + (('\r' as libc::c_char) as i32))));
    let mut idx: i32 = 1;
    let mut d: i32 = ((*c"abcd".as_ptr().offset((idx) as isize)) as i32);
    assert!(((d) == (('b' as libc::c_char) as i32)));
    return 0;
}
