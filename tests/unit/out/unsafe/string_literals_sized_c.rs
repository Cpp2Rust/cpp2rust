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
    let mut empty_buf: [libc::c_char; 256] = [0 as libc::c_char; 256];
    assert!(((((empty_buf[(0) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((empty_buf[(255) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let mut prefix_buf: [libc::c_char; 32] =
        libcc2rs::char_array(b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert!(((((prefix_buf[(0) as usize] as i32) == ('%' as i32)) as i32) != 0));
    assert!(((((prefix_buf[(1) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((prefix_buf[(31) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let mut short_buf: [libc::c_char; 16] = libcc2rs::char_array(b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert!(((((short_buf[(0) as usize] as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((short_buf[(1) as usize] as i32) == ('i' as i32)) as i32) != 0));
    assert!(((((short_buf[(2) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((short_buf[(15) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let mut exact_buf: [libc::c_char; 6] = libcc2rs::char_array(b"hello\0");
    assert!(((((exact_buf[(0) as usize] as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((exact_buf[(4) as usize] as i32) == ('o' as i32)) as i32) != 0));
    assert!(((((exact_buf[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((::std::mem::size_of::<[libc::c_char; 6]>()) == (6_usize)) as i32) != 0));
    assert!(
        (((((::std::mem::size_of::<[libc::c_char; 6]>() as usize).wrapping_sub(1_usize))
            == (5_usize)) as i32)
            != 0)
    );
    assert!(((((::std::mem::size_of::<[libc::c_char; 1]>()) == (1_usize)) as i32) != 0));
    assert!(
        (((((::std::mem::size_of::<[libc::c_char; 16]>() as usize).wrapping_sub(1_usize))
            == (15_usize)) as i32)
            != 0)
    );
    return 0;
}
